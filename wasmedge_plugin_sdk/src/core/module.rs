//! Defines WasmEdge Instance and other relevant types.

use wasmedge_sys_ffi as ffi;

use crate::{core::types::WasmVal, error::InstanceError};

use super::{
    instance::{function::FuncRef, memory::Memory},
    instance::{
        function::InnerFunc,
        global::{ConstGlobal, Global, MutGlobal},
    },
    types::WasmEdgeString,
};

pub(crate) trait AsInnerInstance {
    unsafe fn get_mut_ptr(&self) -> *mut ffi::WasmEdge_ModuleInstanceContext;
}

impl AsInnerInstance for InnerInstance {
    unsafe fn get_mut_ptr(&self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.0
    }
}

#[derive(Debug)]
pub struct InnerInstance(*mut ffi::WasmEdge_ModuleInstanceContext);
impl InnerInstance {
    pub unsafe fn from_raw(ctx: *mut ffi::WasmEdge_ModuleInstanceContext) -> Self {
        Self(ctx)
    }

    pub unsafe fn raw(&mut self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.0
    }
}

impl Into<*mut ffi::WasmEdge_ModuleInstanceContext> for InnerInstance {
    fn into(self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.0
    }
}

unsafe impl Send for InnerInstance {}
unsafe impl Sync for InnerInstance {}

pub trait AsInstance {
    /// Returns the exported [function instance](crate::Function) by name.
    ///
    /// # Argument
    ///
    /// * `name` - The name of the target exported [function instance](crate::Function).
    ///
    /// # Error
    ///
    /// If fail to find the target [function](crate::Function), then an error is returned.
    fn get_func(&self, name: impl AsRef<str>) -> Result<FuncRef, InstanceError>;

    /// Returns the length of the exported [function instances](crate::Function) in this module instance.
    fn func_len(&self) -> u32;

    /// Returns the names of the exported [function instances](crate::Function) in this module instance.
    fn func_names(&self) -> Option<Vec<String>>;

    /// Returns the exported [memory instance](crate::Memory) by name.
    ///
    /// # Argument
    ///
    /// * `name` - The name of the target exported [memory instance](crate::Memory).
    ///
    /// # Error
    ///
    /// If fail to find the target [memory instance](crate::Memory), then an error is returned.
    fn get_memory(&self, name: &str) -> Result<Memory, InstanceError>;

    fn get_all_exports_memories(&self) -> Vec<(String, Memory)>;

    /// Returns the length of the exported [memory instances](crate::Memory) in this module instance.
    fn mem_len(&self) -> u32;

    /// Returns the names of all exported [memory instances](crate::Memory) in this module instance.
    fn mem_names(&self) -> Option<Vec<String>>;

    fn get_all_exports_globals(&self) -> Vec<Global>;

    fn set_global(&mut self, global: MutGlobal) -> Result<(), InstanceError>;
}

#[derive(Debug)]
pub struct ImportModule<T: Sized + Send> {
    pub inner: InnerInstance,
    pub data_ptr: *mut T,
}

unsafe extern "C" fn host_data_finalizer<T: Sized + Send>(raw: *mut ::std::os::raw::c_void) {
    let host_data: Box<T> = Box::from_raw(raw as *mut T);
    drop(host_data);
}

impl<T: Sized + Send> ImportModule<T> {
    pub fn create<S: AsRef<str>>(name: S, data: T) -> Result<Self, InstanceError> {
        let raw_name = WasmEdgeString::new(name.as_ref())?;
        let host_data = Box::leak(Box::new(data)) as *mut T;
        let ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceCreateWithData(
                raw_name.as_raw(),
                host_data as _,
                Some(host_data_finalizer::<T>),
            )
        };

        match ctx.is_null() {
            true => {
                unsafe { host_data_finalizer::<T>(host_data as _) };
                Err(InstanceError::CreateImportModule)
            }
            false => Ok(Self {
                inner: InnerInstance(ctx),
                data_ptr: host_data,
            }),
        }
    }
}

impl<T: Sized + Send> AsInnerInstance for ImportModule<T> {
    unsafe fn get_mut_ptr(&self) -> *mut ffi::WasmEdge_ModuleInstanceContext {
        self.inner.0
    }
}

impl<T: AsInnerInstance> AsInstance for T {
    fn get_func(&self, name: impl AsRef<str>) -> Result<FuncRef, InstanceError> {
        let func_name = WasmEdgeString::new(name.as_ref())?;
        let func_ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindFunction(self.get_mut_ptr(), func_name.as_raw())
        };
        if func_ctx.is_null() {
            Err(InstanceError::NotFoundFunc(name.as_ref().to_string()))
        } else {
            Ok(FuncRef {
                inner: InnerFunc(func_ctx),
            })
        }
    }

    fn get_memory(&self, name: &str) -> Result<Memory, InstanceError> {
        let mem_name: WasmEdgeString = WasmEdgeString::new(name)?;
        let ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindMemory(self.get_mut_ptr(), mem_name.as_raw())
        };
        if ctx.is_null() {
            Err(InstanceError::NotFoundMem(name.to_string()))
        } else {
            Ok(Memory::from_raw(ctx))
        }
    }

    fn get_all_exports_memories(&self) -> Vec<(String, Memory)> {
        unsafe {
            let mut memories = vec![];

            let len_mem_names = ffi::WasmEdge_ModuleInstanceListMemoryLength(self.get_mut_ptr());
            let mut mem_names = Vec::with_capacity(len_mem_names as usize);
            let len = ffi::WasmEdge_ModuleInstanceListMemory(
                self.get_mut_ptr(),
                mem_names.as_mut_ptr(),
                len_mem_names,
            );
            mem_names.set_len(len as usize);

            for mem_name in mem_names {
                let ptr = ffi::WasmEdge_ModuleInstanceFindMemory(self.get_mut_ptr(), mem_name);
                if !ptr.is_null() {
                    let mem_name: Result<String, std::str::Utf8Error> = mem_name.into();
                    if let Ok(name) = mem_name {
                        memories.push((name, Memory::from_raw(ptr)));
                    }
                }
            }

            memories
        }
    }

    /// Returns the length of the exported [function instances](crate::Function) in this module instance.
    fn func_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListFunctionLength(self.get_mut_ptr()) }
    }

    /// Returns the names of the exported [function instances](crate::Function) in this module instance.
    fn func_names(&self) -> Option<Vec<String>> {
        let len_func_names = self.func_len();
        if len_func_names > 0 {
            let mut func_names = Vec::with_capacity(len_func_names as usize);
            unsafe {
                ffi::WasmEdge_ModuleInstanceListFunction(
                    self.get_mut_ptr(),
                    func_names.as_mut_ptr(),
                    len_func_names,
                );
                func_names.set_len(len_func_names as usize);
            }

            let names = func_names
                .into_iter()
                .map(|x| {
                    let r: Result<String, std::str::Utf8Error> = x.into();
                    r.unwrap_or_default()
                })
                .collect::<Vec<String>>();
            Some(names)
        } else {
            None
        }
    }

    /// Returns the length of the exported [memory instances](crate::Memory) in this module instance.
    fn mem_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListMemoryLength(self.get_mut_ptr()) }
    }

    /// Returns the names of all exported [memory instances](crate::Memory) in this module instance.
    fn mem_names(&self) -> Option<Vec<String>> {
        let len_mem_names = self.mem_len();
        match len_mem_names > 0 {
            true => {
                let mut mem_names = Vec::with_capacity(len_mem_names as usize);
                unsafe {
                    let len = ffi::WasmEdge_ModuleInstanceListMemory(
                        self.get_mut_ptr(),
                        mem_names.as_mut_ptr(),
                        len_mem_names,
                    );
                    mem_names.set_len(len as usize);
                }

                let names = mem_names
                    .into_iter()
                    .map(|x| {
                        let r: Result<String, std::str::Utf8Error> = x.into();
                        r.unwrap_or_default()
                    })
                    .collect::<Vec<String>>();
                Some(names)
            }
            false => None,
        }
    }

    fn get_all_exports_globals(&self) -> Vec<Global> {
        unsafe {
            let mut globals = vec![];
            let module = self.get_mut_ptr();
            let globals_num = ffi::WasmEdge_ModuleInstanceListGlobalLength(module);
            let mut global_names = Vec::with_capacity(globals_num as usize);
            let len = ffi::WasmEdge_ModuleInstanceListGlobal(
                self.get_mut_ptr(),
                global_names.as_mut_ptr(),
                globals_num,
            );
            global_names.set_len(len as usize);

            for name in global_names {
                let global_ctx = ffi::WasmEdge_ModuleInstanceFindGlobal(module, name);
                let global_type = ffi::WasmEdge_GlobalInstanceGetGlobalType(global_ctx);
                let val = WasmVal::from(ffi::WasmEdge_GlobalInstanceGetValue(global_ctx));
                if ffi::WasmEdge_Mutability_Const
                    == ffi::WasmEdge_GlobalTypeGetMutability(global_type)
                {
                    let name: Result<String, std::str::Utf8Error> = name.into();
                    if let Ok(name) = name {
                        globals.push(Global::Const(ConstGlobal { name, val }));
                    }
                } else {
                    let name: Result<String, std::str::Utf8Error> = name.into();
                    if let Ok(name) = name {
                        globals.push(Global::Mut(MutGlobal { name, val }));
                    }
                };
            }

            globals
        }
    }

    fn set_global(&mut self, global: MutGlobal) -> Result<(), InstanceError> {
        unsafe {
            let module = self.get_mut_ptr();
            let MutGlobal { name, val } = global;
            let wasmedge_name = WasmEdgeString::new(&name)?;
            let global_ctx = ffi::WasmEdge_ModuleInstanceFindGlobal(module, wasmedge_name.as_raw());
            if global_ctx.is_null() {
                return Err(InstanceError::NotFoundMutGlobal(name));
            }
            let global_type = ffi::WasmEdge_GlobalInstanceGetGlobalType(global_ctx);
            if global_type.is_null() {
                return Err(InstanceError::NotFoundMutGlobal(name));
            }
            if ffi::WasmEdge_Mutability_Const
                == ffi::WasmEdge_GlobalTypeGetMutability(global_ctx as *const _)
            {
                return Err(InstanceError::NotFoundMutGlobal(name));
            }

            ffi::WasmEdge_GlobalInstanceSetValue(global_ctx, val.into());
        }
        Ok(())
    }
}
