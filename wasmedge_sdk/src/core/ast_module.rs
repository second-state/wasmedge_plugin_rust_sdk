use super::config::Config;
use crate::error::{CoreCommonError, CoreError};
use crate::utils::check;

use wasmedge_sys_ffi as ffi;

pub struct Loader {
    loader_inner: *mut ffi::WasmEdge_LoaderContext,
    validator_inner: *mut ffi::WasmEdge_ValidatorContext,
}
impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            if !self.loader_inner.is_null() {
                ffi::WasmEdge_LoaderDelete(self.loader_inner)
            }
            if !self.validator_inner.is_null() {
                ffi::WasmEdge_ValidatorDelete(self.validator_inner)
            }
        }
    }
}
unsafe impl Send for Loader {}

impl Loader {
    pub fn create(config: &Option<Config>) -> Option<Self> {
        unsafe {
            let config_ctx = if let Some(c) = config {
                c.inner.0
            } else {
                std::ptr::null()
            };

            let loader_inner = ffi::WasmEdge_LoaderCreate(config_ctx);
            if loader_inner.is_null() {
                return None;
            }

            let validator_inner = ffi::WasmEdge_ValidatorCreate(config_ctx);
            if validator_inner.is_null() {
                ffi::WasmEdge_LoaderDelete(loader_inner);
                return None;
            }
            Some(Self {
                loader_inner,
                validator_inner,
            })
        }
    }

    pub fn load_module_from_bytes(&self, wasm: &[u8]) -> Result<AstModule, CoreError> {
        unsafe {
            let mut mod_ctx: *mut ffi::WasmEdge_ASTModuleContext = std::ptr::null_mut();

            check(ffi::WasmEdge_LoaderParseFromBuffer(
                self.loader_inner,
                &mut mod_ctx,
                wasm.as_ptr(),
                wasm.len() as u32,
            ))?;

            debug_assert!(!mod_ctx.is_null());
            if mod_ctx.is_null() {
                return Err(CoreError::Common(CoreCommonError::RuntimeError));
            }

            let validate_result = check(ffi::WasmEdge_ValidatorValidate(
                self.validator_inner,
                mod_ctx,
            ));

            if let Err(e) = validate_result {
                ffi::WasmEdge_ASTModuleDelete(mod_ctx);
                return Err(e);
            }

            Ok(AstModule { inner: mod_ctx })
        }
    }
}

#[derive(Debug)]
pub struct AstModule {
    pub inner: *mut ffi::WasmEdge_ASTModuleContext,
}
impl Drop for AstModule {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { ffi::WasmEdge_ASTModuleDelete(self.inner) };
        }
    }
}
unsafe impl Send for AstModule {}
unsafe impl Sync for AstModule {}
