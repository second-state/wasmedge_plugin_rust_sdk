use std::ffi::CString;

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::WasmVal,
};

pub fn create_module() -> SyncModule<()> {
    fn hello<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        println!("[plugin] hello wasmedge plugin");
        Ok(vec![])
    }

    let mut module = SyncModule::create("hello_module", ()).unwrap();

    module.add_func("hello", (vec![], vec![]), hello).unwrap();

    module
}

#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> PluginDescriptorRef {
    let mut builder = PluginBuilder::create(
        CString::new("hello_plugin").unwrap(),
        CString::new("a demo plugin").unwrap(),
    );
    builder.add_module(
        CString::new("hello_module").unwrap(),
        CString::new("a demo of module").unwrap(),
        create_module,
    );

    builder.build()
}
