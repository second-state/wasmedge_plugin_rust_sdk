use std::{ffi::CString, io::Write};

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::{ValType, WasmVal},
};

#[derive(Debug)]
pub enum PluginError {
    ParamError,
    MemoryError,
    UTF8Error,
}

pub fn create_module() -> SyncModule<()> {
    fn to_uppercase<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        main_memory: &'a mut Memory,
        _data: &'a mut (),
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        fn to_uppercase_(
            main_memory: &mut Memory,
            data_ptr: &WasmVal,
            data_len: &WasmVal,
        ) -> Result<(), PluginError> {
            if let (WasmVal::I32(data_ptr), WasmVal::I32(data_len)) = (data_ptr, data_len) {
                let mut bytes = main_memory
                    .data_pointer_mut(*data_ptr as usize, *data_len as usize)
                    .ok_or(PluginError::MemoryError)?;

                let uppercase = std::str::from_utf8_mut(bytes)
                    .map_err(|_| PluginError::UTF8Error)?
                    .to_uppercase();

                let _ = bytes.write_all(uppercase.as_bytes());

                Ok(())
            } else {
                Err(PluginError::ParamError)
            }
        }

        match to_uppercase_(main_memory, &args[0], &args[1]) {
            Ok(_) => Ok(vec![WasmVal::I32(0)]),
            Err(PluginError::ParamError) => Ok(vec![WasmVal::I32(-1)]),
            Err(PluginError::MemoryError) => Ok(vec![WasmVal::I32(-2)]),
            Err(PluginError::UTF8Error) => Ok(vec![WasmVal::I32(-3)]),
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
