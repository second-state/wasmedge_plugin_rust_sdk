use std::os::raw::c_void;

use crate::core::executor::Executor;
use crate::core::types::{ValType, WasmVal};
use crate::error::CoreError;
use wasmedge_sys_ffi as ffi;

pub type FnWrapper = unsafe extern "C" fn(
    key_ptr: *mut c_void,
    data_ptr: *mut c_void,
    calling_frame_ctx: *const ffi::WasmEdge_CallingFrameContext,
    params: *const ffi::WasmEdge_Value,
    param_len: u32,
    returns: *mut ffi::WasmEdge_Value,
    return_len: u32,
) -> ffi::WasmEdge_Result;

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub(crate) inner: InnerFunc,
}

impl Function {
    pub unsafe fn custom_create(
        ty: (Vec<ValType>, Vec<ValType>),
        wrapper_fn: FnWrapper,
        real_fn: *mut c_void,
        data: *mut c_void,
    ) -> Option<Self> {
        unsafe {
            let ty = FuncType::create(ty.0, ty.1)?;
            let ctx = ffi::WasmEdge_FunctionInstanceCreateBinding(
                ty.inner.0,
                Some(wrapper_fn),
                real_fn,
                data.cast(),
                0,
            );
            ty.delete();
            if ctx.is_null() {
                None
            } else {
                Some(Self {
                    inner: InnerFunc(ctx),
                })
            }
        }
    }
}

impl Function {
    #[allow(dead_code)]
    pub fn func_type(&self) -> Option<(Vec<ValType>, Vec<ValType>)> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some((
                ty.params_type_iter().collect(),
                ty.returns_type_iter().collect(),
            ))
        }
    }

    #[allow(dead_code)]
    pub fn func_param_size(&self) -> Option<usize> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some(ty.params_len() as usize)
        }
    }

    #[allow(dead_code)]
    pub fn func_return_size(&self) -> Option<usize> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some(ty.returns_len() as usize)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn delete(self) {
        unsafe { ffi::WasmEdge_FunctionInstanceDelete(self.inner.0 as *mut _) }
    }
}

#[derive(Debug)]
pub(crate) struct InnerFunc(pub(crate) *const ffi::WasmEdge_FunctionInstanceContext);
impl Clone for InnerFunc {
    fn clone(&self) -> Self {
        InnerFunc(self.0)
    }
}
unsafe impl Send for InnerFunc {}
unsafe impl Sync for InnerFunc {}

#[derive(Debug)]
pub struct FuncType {
    pub(crate) inner: InnerFuncType,
}
impl FuncType {
    pub fn create<I: IntoIterator<Item = ValType>, R: IntoIterator<Item = ValType>>(
        args: I,
        returns: R,
    ) -> Option<Self> {
        let param_tys = args
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<ffi::WasmEdge_ValType>>();
        let ret_tys = returns
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<ffi::WasmEdge_ValType>>();

        let ctx = unsafe {
            ffi::WasmEdge_FunctionTypeCreate(
                param_tys.as_ptr() as *const _,
                param_tys.len() as u32,
                ret_tys.as_ptr() as *const _,
                ret_tys.len() as u32,
            )
        };
        if ctx.is_null() {
            None
        } else {
            Some(Self {
                inner: InnerFuncType(ctx),
            })
        }
    }

    pub fn params_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_FunctionTypeGetParametersLength(self.inner.0) }
    }

    pub fn params_type_iter(&self) -> impl Iterator<Item = ValType> {
        let len = self.params_len();
        let mut types = Vec::with_capacity(len as usize);
        unsafe {
            ffi::WasmEdge_FunctionTypeGetParameters(self.inner.0, types.as_mut_ptr(), len);
            types.set_len(len as usize);
        }

        types.into_iter().map(Into::into)
    }

    pub fn returns_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_FunctionTypeGetReturnsLength(self.inner.0) }
    }

    pub fn returns_type_iter(&self) -> impl Iterator<Item = ValType> {
        let len = self.returns_len();
        let mut types = Vec::with_capacity(len as usize);
        unsafe {
            ffi::WasmEdge_FunctionTypeGetReturns(self.inner.0, types.as_mut_ptr(), len);
            types.set_len(len as usize);
        }

        types.into_iter().map(Into::into)
    }

    pub(crate) fn delete(self) {
        unsafe { ffi::WasmEdge_FunctionTypeDelete(self.inner.0 as *mut _) };
    }
}

#[derive(Debug)]
pub(crate) struct InnerFuncType(pub(crate) *const ffi::WasmEdge_FunctionTypeContext);
unsafe impl Send for InnerFuncType {}
unsafe impl Sync for InnerFuncType {}

#[derive(Debug, Clone)]
pub struct FuncRef {
    pub(crate) inner: InnerFunc,
}

impl FuncRef {
    pub fn from_raw(ctx: *const ffi::WasmEdge_FunctionInstanceContext) -> Self {
        Self {
            inner: InnerFunc(ctx),
        }
    }

    pub fn func_type(&self) -> Option<(Vec<ValType>, Vec<ValType>)> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some((
                ty.params_type_iter().collect(),
                ty.returns_type_iter().collect(),
            ))
        }
    }

    pub fn func_param_size(&self) -> Option<usize> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some(ty.params_len() as usize)
        }
    }

    pub fn func_return_size(&self) -> Option<usize> {
        let ty = unsafe { ffi::WasmEdge_FunctionInstanceGetFunctionType(self.inner.0 as *mut _) };
        if ty.is_null() {
            None
        } else {
            let ty = FuncType {
                inner: InnerFuncType(ty),
            };
            Some(ty.returns_len() as usize)
        }
    }

    pub fn call(&self, engine: &Executor, args: &[WasmVal]) -> Result<Vec<WasmVal>, CoreError> {
        engine.run_func_ref(self, args)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerFuncRef(());
unsafe impl Send for InnerFuncRef {}
unsafe impl Sync for InnerFuncRef {}
