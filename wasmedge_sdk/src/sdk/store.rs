use wasmedge_sys_ffi as ffi;

use crate::{
    core::{
        executor::{Executor, InnerStore},
        module::ImportModule,
    },
    error::CoreError,
    utils::check,
};

pub struct Store<'import> {
    pub(crate) inner_store: InnerStore,
    imports: std::marker::PhantomData<&'import ImportModule<()>>,
}

impl<'import> Store<'import> {
    pub fn create() -> Option<Self> {
        unsafe {
            let ctx = ffi::WasmEdge_StoreCreate();
            if ctx.is_null() {
                None
            } else {
                Some(Store {
                    inner_store: InnerStore(ctx),
                    imports: Default::default(),
                })
            }
        }
    }

    pub fn register_import_object<T: Sized + Send>(
        &mut self,
        executor: &Executor,
        import: &'import mut ImportModule<T>,
    ) -> Result<(), CoreError> {
        unsafe {
            check(ffi::WasmEdge_ExecutorRegisterImport(
                executor.inner.0,
                self.inner_store.0,
                import.inner.raw(),
            ))?;
            Ok(())
        }
    }
}
