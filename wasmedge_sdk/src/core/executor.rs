//! Defines WasmEdge Executor.

#[cfg(feature = "embedded")]
use super::ast_module::AstModule;
#[cfg(feature = "embedded")]
use super::config::Config;
use super::instance::function::FuncRef;
#[cfg(feature = "embedded")]
use super::module::InnerInstance;
use super::types::WasmVal;

use crate::error::{CoreError, CoreExecutionError};
use crate::utils::check;

use wasmedge_sys_ffi as ffi;

/// Defines an execution environment for both pure WASM and compiled WASM.
#[derive(Debug)]
pub struct Executor {
    pub(crate) inner: InnerExecutor,
}
impl Executor {
    #[cfg(feature = "embedded")]
    pub fn create(config: &Option<Config>) -> Option<Self> {
        unsafe {
            let conf_ctx = match config {
                Some(cfg) => cfg.inner.0,
                None => std::ptr::null_mut(),
            };
            let ctx = ffi::WasmEdge_ExecutorCreate(conf_ctx, std::ptr::null_mut());
            if ctx.is_null() {
                None
            } else {
                Some(Executor {
                    inner: InnerExecutor(ctx),
                })
            }
        }
    }

    #[cfg(feature = "embedded")]
    pub fn instantiate(
        &self,
        store: &InnerStore,
        module: &AstModule,
    ) -> Result<InnerInstance, CoreError> {
        let mut instance_ctx = std::ptr::null_mut();
        unsafe {
            check(ffi::WasmEdge_ExecutorInstantiate(
                self.inner.0,
                &mut instance_ctx,
                store.0,
                module.inner,
            ))?;
        }

        debug_assert!(!instance_ctx.is_null());
        if instance_ctx.is_null() {
            return Err(CoreError::runtime());
        }

        Ok(unsafe { InnerInstance::from_raw(instance_ctx) })
    }

    pub fn run_func_ref(
        &self,
        func: &FuncRef,
        params: &[WasmVal],
    ) -> Result<Vec<WasmVal>, CoreError> {
        let raw_params = params.into_iter().map(|x| x.into()).collect::<Vec<_>>();

        // get the length of the function's returns
        let returns_len = func
            .func_return_size()
            .ok_or(CoreError::Execution(CoreExecutionError::FuncTypeMismatch))?;

        unsafe {
            let mut returns = Vec::with_capacity(returns_len);
            check(ffi::WasmEdge_ExecutorInvoke(
                self.inner.0,
                func.inner.0,
                raw_params.as_ptr(),
                raw_params.len() as u32,
                returns.as_mut_ptr(),
                returns_len as u32,
            ))?;
            returns.set_len(returns_len);
            Ok(returns.into_iter().map(Into::into).collect::<Vec<_>>())
        }
    }
}

#[derive(Debug)]
pub(crate) struct InnerExecutor(pub(crate) *mut ffi::WasmEdge_ExecutorContext);
impl Drop for InnerExecutor {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::WasmEdge_ExecutorDelete(self.0) }
        }
    }
}
unsafe impl Send for InnerExecutor {}

#[derive(Debug)]
pub struct InnerStore(pub *mut ffi::WasmEdge_StoreContext);
impl Drop for InnerStore {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::WasmEdge_StoreDelete(self.0) }
        }
    }
}
unsafe impl Send for InnerStore {}
unsafe impl Sync for InnerStore {}
