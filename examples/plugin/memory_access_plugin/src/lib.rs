use std::{ffi::CString, io::Write};

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::{ValType, WasmVal},
};

pub fn create_module() -> SyncModule<()> {
    fn to_uppercase<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        main_memory: &'a mut Memory,
        _data: &'a mut (),
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        if let (WasmVal::I32(data_ptr), WasmVal::I32(data_len)) = (args[0].clone(), args[1].clone())
        {
            if let Some(mut bytes) =
                main_memory.data_pointer_mut(data_ptr as usize, data_len as usize)
            {
                let uppercase = if let Ok(s) = std::str::from_utf8_mut(bytes) {
                    s.to_uppercase()
                } else {
                    return Ok(vec![WasmVal::I32(-3)]);
                };

                let _ = bytes.write_all(uppercase.as_bytes());
                Ok(vec![WasmVal::I32(0)])
            } else {
                Ok(vec![WasmVal::I32(-2)])
            }
        } else {
            Ok(vec![WasmVal::I32(-1)])
        }
    }

    let mut module = SyncModule::create("memory_access_module", ()).unwrap();

    module
        .add_func(
            "to_uppercase",
            (vec![ValType::I32, ValType::I32], vec![ValType::I32]),
            to_uppercase,
        )
        .unwrap();

    module
}

#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> PluginDescriptorRef {
    println!("memory_access_plugin loading");
    let mut builder = PluginBuilder::create(
        CString::new("memory_access_plugin").unwrap(),
        CString::new("a demo plugin").unwrap(),
    );
    builder.add_module(
        CString::new("memory_access_module").unwrap(),
        CString::new("a demo of module").unwrap(),
        create_module,
    );

    builder.build()
}
