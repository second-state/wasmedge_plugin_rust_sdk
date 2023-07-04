//! Defines WasmEdge Executor.

use super::instance::function::FuncRef;

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
