use std::ffi::CString;

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::{ValType, WasmVal},
};

pub fn create_module() -> SyncModule<(i32, i32)> {
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
            println!("add x with {}", v);
        }
        println!("module data  = {:?}", data);
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
            println!("add y with {}", v);
        }
        println!("module data  = {:?}", data);
        Ok(vec![WasmVal::I32(data.1)])
    }

    let mut module = SyncModule::create("demo_module", (0, 0)).unwrap();

    module
        .add_func("add_x", (vec![ValType::I32], vec![ValType::I32]), add_x)
        .unwrap();
    module
        .add_func("add_y", (vec![ValType::I32], vec![ValType::I32]), add_y)
        .unwrap();
    module
}

#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> PluginDescriptorRef {
    let mut builder = PluginBuilder::create(
        CString::new("demo_plugin").unwrap(),
        CString::new("a demo of plugin").unwrap(),
    );
    builder.add_module(
        CString::new("demo_module").unwrap(),
        CString::new("a demo of module").unwrap(),
        create_module,
    );

    builder.build()
}
