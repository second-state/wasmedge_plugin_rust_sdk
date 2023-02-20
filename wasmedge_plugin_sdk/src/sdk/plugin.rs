use std::ptr::{null, null_mut};

use wasmedge_sys_ffi as ffi;

pub use wasmedge_sys_ffi::WasmEdge_PluginDescriptor;
pub type PluginDescriptorRef = *const ffi::WasmEdge_PluginDescriptor;

pub struct PluginBuilder {
    plugin_name: std::ffi::CString,
    plugin_description: std::ffi::CString,
    descriptor: ffi::WasmEdge_PluginDescriptor,
    module_descs: Vec<(std::ffi::CString, std::ffi::CString)>,
    modules: Vec<ffi::WasmEdge_ModuleDescriptor>,
}

const fn must_zero<T>() -> bool {
    if std::mem::size_of::<T>() > 0 {
        panic!("T size must is zero")
    };
    true
}

unsafe extern "C" fn instance_create_wrap<T, F>(
    _: *const ffi::WasmEdge_ModuleDescriptor,
) -> *mut ffi::WasmEdge_ModuleInstanceContext
where
    T: Send + Sized,
    F: Fn() -> crate::module::SyncModule<T>,
{
    unsafe {
        let f = std::mem::zeroed::<F>();
        let module = Box::leak(Box::new(f()));
        module.inner.raw()
    }
}

impl PluginBuilder {
    pub fn create(name: std::ffi::CString, description: std::ffi::CString) -> Self {
        Self {
            plugin_name: name,
            plugin_description: description,
            descriptor: ffi::WasmEdge_PluginDescriptor {
                Name: null(),
                Description: null(),
                APIVersion: ffi::WasmEdge_Plugin_CurrentAPIVersion,
                Version: ffi::WasmEdge_PluginVersionData {
                    Major: 0,
                    Minor: 0,
                    Patch: 0,
                    Build: 0,
                },
                ModuleCount: 0,
                ProgramOptionCount: 0,
                ModuleDescriptions: null_mut(),
                ProgramOptions: null_mut(),
            },
            module_descs: vec![],
            modules: vec![],
        }
    }

    pub fn version(&mut self, major: u32, minor: u32, patch: u32, build: u32) -> &mut Self {
        self.descriptor.Version.Major = major;
        self.descriptor.Version.Minor = minor;
        self.descriptor.Version.Patch = patch;
        self.descriptor.Version.Build = build;
        self
    }

    pub fn add_module<T: Send + Sized, F>(
        &mut self,
        module_name: std::ffi::CString,
        module_description: std::ffi::CString,
        _module_producer: F,
    ) -> &mut Self
    where
        F: Fn() -> crate::module::SyncModule<T>,
    {
        struct FnCheck<T: Send + Sized, F: Fn() -> crate::module::SyncModule<T>> {
            _p: std::marker::PhantomData<(T, F)>,
        }
        impl<T: Send + Sized, F: Fn() -> crate::module::SyncModule<T>> FnCheck<T, F> {
            const RESULT: bool = must_zero::<F>();
        }

        let _ = FnCheck::<T, F>::RESULT;

        self.modules.push(ffi::WasmEdge_ModuleDescriptor {
            Name: module_name.as_ptr(),
            Description: module_description.as_ptr(),
            Create: Some(instance_create_wrap::<T, F>),
        });
        self.module_descs.push((module_name, module_description));
        self
    }

    pub fn build(self) -> PluginDescriptorRef {
        let plugin = Box::new(self);
        let plugin_raw = Box::leak(plugin);

        plugin_raw.descriptor.Name = plugin_raw.plugin_name.as_ptr();
        plugin_raw.descriptor.Description = plugin_raw.plugin_description.as_ptr();

        let modules = plugin_raw.modules.as_mut_ptr();
        plugin_raw.descriptor.ModuleDescriptions = modules;
        plugin_raw.descriptor.ModuleCount = plugin_raw.modules.len() as u32;

        &plugin_raw.descriptor
    }
}
