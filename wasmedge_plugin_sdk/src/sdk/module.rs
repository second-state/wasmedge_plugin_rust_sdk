use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    core::{
        executor::{Executor, InnerExecutor},
        instance::function::{FnWrapper, Function},
        instance::memory::Memory,
        module::{AsInnerInstance, AsInstance, ImportModule, InnerInstance},
        types::{ValType, WasmEdgeString, WasmVal},
    },
    error::{CoreError, InstanceError},
};
use thiserror::Error;
use wasmedge_sys::ffi;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CallError {
    #[error("{0}")]
    InstanceError(#[from] InstanceError),
    #[error("{0}")]
    RuntimeError(#[from] CoreError),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AddFuncError {
    #[error("Found an interior nul byte")]
    NameError(#[from] std::ffi::NulError),
    #[error("Illegal Async Function name ")]
    IllegalName,
    #[error("Fail to create Function instance")]
    FunctionCreate,
}

use std::ffi::c_void;

pub(crate) unsafe extern "C" fn wrapper_sync_fn<T: Sized + Send>(
    key_ptr: *mut c_void,
    data_ptr: *mut c_void,
    calling_frame_ctx: *const ffi::WasmEdge_CallingFrameContext,
    params: *const ffi::WasmEdge_Value,
    param_len: u32,
    returns: *mut ffi::WasmEdge_Value,
    return_len: u32,
) -> ffi::WasmEdge_Result {
    let cous = || -> Result<(), CoreError> {
        let inst_ctx = ffi::WasmEdge_CallingFrameGetModuleInstance(calling_frame_ctx);
        let executor_ctx = ffi::WasmEdge_CallingFrameGetExecutor(calling_frame_ctx);
        let main_mem_ctx = ffi::WasmEdge_CallingFrameGetMemoryInstance(calling_frame_ctx, 0);

        let mut inst = std::mem::ManuallyDrop::new(SyncInstanceRef {
            inst: InnerInstance::from_raw(inst_ctx.cast_mut()),
            executor: Executor {
                inner: InnerExecutor(executor_ctx),
            },
        });

        let mut mem = Memory::from_raw(main_mem_ctx);

        let data_ptr = data_ptr.cast::<T>().as_mut();
        debug_assert!(data_ptr.is_some());
        let data_ptr = data_ptr.unwrap();

        let real_fn: fn(
            &mut SyncInstanceRef,
            &mut Memory,
            &mut T,
            Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> = std::mem::transmute(key_ptr);

        let input = {
            let raw_input = std::slice::from_raw_parts(params, param_len as usize);
            raw_input
                .iter()
                .map(|r| (*r).into())
                .collect::<Vec<WasmVal>>()
        };
        let v = real_fn(&mut inst, &mut mem, data_ptr, input)?;

        let return_len = return_len as usize;
        let raw_returns = std::slice::from_raw_parts_mut(returns, return_len);

        for (idx, item) in v.into_iter().enumerate() {
            raw_returns[idx] = item.into();
        }
        Ok(())
    };
    match cous() {
        Ok(_) => ffi::WasmEdge_Result { Code: 0x0 },
        Err(e) => e.into(),
    }
}

pub type SyncWasmFn<T> = for<'a> fn(
    &'a mut SyncInstanceRef,
    &'a mut Memory,
    &'a mut T,
    Vec<WasmVal>,
) -> Result<Vec<WasmVal>, CoreError>;

pub struct SyncInstanceRef {
    inst: InnerInstance,
    executor: Executor,
}

impl AsInnerInstance for SyncInstanceRef {
    unsafe fn get_mut_ptr(&self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.inst.get_mut_ptr()
    }
}

impl SyncInstanceRef {
    pub fn call<'r>(&mut self, name: &str, args: Vec<WasmVal>) -> Result<Vec<WasmVal>, CoreError> {
        let func = self
            .get_func(name)
            .map_err(|_| CoreError::Common(crate::error::CoreCommonError::FuncNotFound))?;
        let result = self.executor.run_func_ref(&func, &args)?;
        Ok(result)
    }
}

pub struct SyncInstance {
    inst_ref: SyncInstanceRef,
}

impl AsInnerInstance for SyncInstance {
    unsafe fn get_mut_ptr(&self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.inst_ref.get_mut_ptr()
    }
}

impl SyncInstance {
    pub fn call<'r>(&mut self, name: &str, args: Vec<WasmVal>) -> Result<Vec<WasmVal>, CoreError> {
        self.inst_ref.call(name, args)
    }
}

pub struct PluginModule<T: Send + Sized> {
    pub(crate) inner: ImportModule<T>,
}

impl<T: Send + Sized> Into<*mut ffi::WasmEdge_ModuleInstanceContext> for PluginModule<T> {
    fn into(self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.inner.inner.into()
    }
}

impl<T: Send + Sized> Deref for PluginModule<T> {
    type Target = ImportModule<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Send + Sized> DerefMut for PluginModule<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Send + Sized> PluginModule<T> {
    pub fn create<S: AsRef<str>>(name: S, data: T) -> Result<Self, InstanceError> {
        let inner = ImportModule::create(name, data)?;
        Ok(Self { inner })
    }

    pub unsafe fn add_custom_func(
        &mut self,
        name: &str,
        ty: (Vec<ValType>, Vec<ValType>),
        wrapper_fn: FnWrapper,
        real_fn: *mut c_void,
        data: *mut T,
    ) -> Result<(), AddFuncError> {
        let func_name = WasmEdgeString::new(name)?;
        let func = Function::custom_create(ty, wrapper_fn, real_fn, data.cast())
            .ok_or(AddFuncError::FunctionCreate)?;

        ffi::WasmEdge_ModuleInstanceAddFunction(
            self.inner.inner.raw(),
            func_name.as_raw(),
            func.inner.0 as *mut _,
        );
        Ok(())
    }

    pub fn add_func(
        &mut self,
        name: &str,
        ty: (Vec<ValType>, Vec<ValType>),
        real_fn: SyncWasmFn<T>,
    ) -> Result<(), AddFuncError> {
        unsafe {
            self.add_custom_func(
                name,
                ty,
                wrapper_sync_fn::<T>,
                real_fn as *mut _,
                self.inner.data_ptr,
            )
        }
    }
}
