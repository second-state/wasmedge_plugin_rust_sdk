mod plugin {
    #[link(wasm_import_module = "memory_access_module")]
    extern "C" {
        fn to_uppercase(ptr: i32, len: i32) -> i32;
        fn get_data() -> i32;
    }

    #[derive(Debug)]
    pub enum PluginError {
        ParamError,
        MemoryError,
        UTF8Error,
    }

    pub fn plugin_to_uppercase(s: &mut String) -> Result<(), PluginError> {
        unsafe {
            let raw = s.as_bytes_mut();
            let e = to_uppercase(raw.as_mut_ptr() as usize as i32, raw.len() as i32);
            match e {
                -1 => Err(PluginError::ParamError),
                -2 => Err(PluginError::MemoryError),
                -3 => Err(PluginError::UTF8Error),
                _ => Ok(()),
            }
        }
    }

    pub fn plugin_get_data() -> Result<String, PluginError> {
        unsafe {
            let ptr = get_data();
            if ptr <= 0 {
                Err(PluginError::MemoryError)
            } else {
                let raw_ptr = ptr as usize as *mut i8;
                let s = std::ffi::CString::from_raw(raw_ptr);
                let s = s.to_str().map_err(|_| PluginError::UTF8Error)?;
                Ok(s.to_string())
            }
        }
    }
}

fn main() {
    let mut s = "hello plugin".to_string();
    plugin::plugin_to_uppercase(&mut s).unwrap();
    println!("[wasm] after plugin_to_uppercase, s = {s}");

    println!("[wasm] get_data = {:?}", plugin::plugin_get_data());
}
