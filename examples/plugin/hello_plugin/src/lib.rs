use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{PluginModule, SyncInstanceRef},
    types::WasmVal,
};

pub struct HostData(String);

pub fn create_module() -> PluginModule<HostData> {
    fn hello<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        data: &'a mut HostData,
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        println!("[plugin] hello wasmedge plugin. {}", data.0);
        Ok(vec![])
    }

    let mut module =
        PluginModule::create("hello_module", HostData("hello, Host Data".into())).unwrap();

    module.add_func("hello", (vec![], vec![]), hello).unwrap();

    module
}

wasmedge_plugin_sdk::plugin::register_plugin!(
    plugin_name="hello_plugin",
    plugin_description="a demo plugin",
    version=(0,0,0,0),
    modules=[
        {"hello_module","a demo of module",create_module}
    ]
);
