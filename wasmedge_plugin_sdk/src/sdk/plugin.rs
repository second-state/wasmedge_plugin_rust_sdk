pub use paste::paste;

use std::ptr::null_mut;

use wasmedge_sys_ffi as ffi;

pub use wasmedge_sys_ffi::WasmEdge_PluginDescriptor;

pub type PluginDescriptorRef = *const ffi::WasmEdge_PluginDescriptor;
pub type PluginVersionData = ffi::WasmEdge_PluginVersionData;
pub type ModuleDescriptor = ffi::WasmEdge_ModuleDescriptor;
pub type ModuleInstanceContext = ffi::WasmEdge_ModuleInstanceContext;

#[macro_export]
macro_rules! register_plugin {

    (
        plugin_name=$plugin_name:expr,
        plugin_description=$plugin_description:expr,
        version=($major:expr, $minor: expr, $patch: expr, $build: expr),
        modules= [$({$module_name:expr,$module_description:expr,$module_create_fn:ident}),*]) => {


    wasmedge_plugin_sdk::plugin::paste! {
        #[export_name = "WasmEdge_Plugin_GetDescriptor"]
        pub extern "C" fn plugin_hook() -> wasmedge_plugin_sdk::plugin::PluginDescriptorRef {
            const DESC: wasmedge_plugin_sdk::plugin::PluginDescriptorRef = &wasmedge_plugin_sdk::plugin::create_plugin(
                concat!($plugin_name, '\0'),
                concat!($plugin_description, '\0'),
                wasmedge_plugin_sdk::plugin::version($major,$minor,$patch,$build),
                &[
                     $(wasmedge_plugin_sdk::plugin::create_module(
                        concat!($module_name,'\0'),
                        concat!($module_description,'\0'),
                        {
                            unsafe extern "C" fn [<wasmedge_wrap_ $module_create_fn>]  (
                                _: *const wasmedge_plugin_sdk::plugin::ModuleDescriptor,
                            ) -> *mut wasmedge_plugin_sdk::plugin::ModuleInstanceContext {
                                $module_create_fn().into()
                            }
                            [<wasmedge_wrap_ $module_create_fn>]
                        },
                    )),*
                ],
            );
            DESC as *const _
        }
    }
    };
}
pub use register_plugin;

pub const fn version(major: u32, minor: u32, patch: u32, build: u32) -> PluginVersionData {
    PluginVersionData {
        Major: major,
        Minor: minor,
        Patch: patch,
        Build: build,
    }
}

pub const fn create_plugin<const N: usize>(
    cstr_name: &'static str,
    cstr_description: &'static str,
    version: ffi::WasmEdge_PluginVersionData,
    modules: &'static [ffi::WasmEdge_ModuleDescriptor; N],
) -> ffi::WasmEdge_PluginDescriptor {
    ffi::WasmEdge_PluginDescriptor {
        Name: cstr_name.as_ptr().cast(),
        Description: cstr_description.as_ptr().cast(),
        APIVersion: ffi::WasmEdge_Plugin_CurrentAPIVersion,
        Version: version,
        ModuleCount: N as u32,
        ModuleDescriptions: modules as *const _ as *mut _,
        ProgramOptionCount: 0,
        ProgramOptions: null_mut(),
    }
}

pub const fn create_module(
    cstr_name: &'static str,
    cstr_description: &'static str,
    module_producer: unsafe extern "C" fn(
        arg1: *const ffi::WasmEdge_ModuleDescriptor,
    ) -> *mut ffi::WasmEdge_ModuleInstanceContext,
) -> ffi::WasmEdge_ModuleDescriptor {
    ffi::WasmEdge_ModuleDescriptor {
        Name: cstr_name.as_ptr().cast(),
        Description: cstr_description.as_ptr().cast(),
        Create: Some(module_producer),
    }
}
