use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{PluginModule, SyncInstanceRef},
    types::{ValType, WasmVal},
};

pub fn create_module() -> PluginModule<(i32, i32)> {
    fn add_x<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        data: &'a mut (i32, i32),
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        let arg = &args[0];
        if let WasmVal::I32(v) = arg {
            let v = *v;
            data.0 += v;
            println!("[plugin] add x with {}", v);
        }
        println!("[plugin] module data  = {:?}", data);
        Ok(vec![WasmVal::I32(data.0)])
    }

    fn add_y<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        data: &'a mut (i32, i32),
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        let arg = &args[0];
        if let WasmVal::I32(v) = arg {
            let v = *v;
            data.1 += v;
            println!("[plugin] add y with {}", v);
        }
        println!("[plugin] module data  = {:?}", data);
        Ok(vec![WasmVal::I32(data.1)])
    }

    let mut module = PluginModule::create("stateful_module", (0, 0)).unwrap();

    module
        .add_func("add_x", (vec![ValType::I32], vec![ValType::I32]), add_x)
        .unwrap();
    module
        .add_func("add_y", (vec![ValType::I32], vec![ValType::I32]), add_y)
        .unwrap();
    module
}

wasmedge_plugin_sdk::plugin::register_plugin!(
    plugin_name="stateful_plugin",
    plugin_description="a demo plugin",
    version=(0,0,0,0),
    modules=[
        {"stateful_module","a demo of module",create_module}
    ]
);
