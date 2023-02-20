pub mod ffi;
pub use ffi::*;

impl From<ffi::WasmEdge_String> for Result<String, std::str::Utf8Error> {
    fn from(s: ffi::WasmEdge_String) -> Self {
        let cstr = unsafe { std::ffi::CStr::from_ptr(s.Buf as *const _) };
        Ok(cstr.to_str()?.to_string())
    }
}
