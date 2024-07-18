use std::{ffi::CString, fmt::Debug};

#[cfg(feature = "wasm_ref")]
use super::instance::function::{FuncRef, InnerFunc};

use wasmedge_sys::ffi;

/// Struct of WasmEdge String.
#[derive(Debug)]
pub(crate) struct WasmEdgeString {
    inner: InnerWasmEdgeString,
}
impl Drop for InnerWasmEdgeString {
    fn drop(&mut self) {
        unsafe { ffi::WasmEdge_StringDelete(self.0) }
    }
}
impl WasmEdgeString {
    pub fn new(s: &str) -> Result<Self, std::ffi::NulError> {
        let cs = CString::new(s)?;
        let ctx = unsafe { ffi::WasmEdge_StringCreateByCString(cs.as_ptr()) };

        Ok(Self {
            inner: InnerWasmEdgeString(ctx),
        })
    }
    pub(crate) fn as_raw(&self) -> ffi::WasmEdge_String {
        self.inner.0
    }
}
impl PartialEq for WasmEdgeString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::WasmEdge_StringIsEqual(self.inner.0, other.inner.0) }
    }
}
impl Eq for WasmEdgeString {}
impl AsRef<str> for WasmEdgeString {
    fn as_ref(&self) -> &str {
        unsafe {
            let bs = std::slice::from_raw_parts(
                self.as_raw().Buf as *const u8,
                self.as_raw().Length as usize,
            );
            std::str::from_utf8_unchecked(bs)
        }
    }
}
impl From<WasmEdgeString> for String {
    fn from(s: WasmEdgeString) -> Self {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(s.as_raw().Buf as *const u8, s.as_raw().Length as usize)
        };

        String::from_utf8(bytes.to_vec()).unwrap_or_default()
    }
}

#[derive(Debug)]
pub(crate) struct InnerWasmEdgeString(pub(crate) ffi::WasmEdge_String);
unsafe impl Send for InnerWasmEdgeString {}
unsafe impl Sync for InnerWasmEdgeString {}

#[derive(Debug, Clone)]
pub enum WasmVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(i128),
    UnknownType(ffi::WasmEdge_Value),
}

impl PartialEq for WasmVal {
    fn eq(&self, other: &Self) -> bool {
        use WasmVal::*;
        match (self, other) {
            (I32(i), I32(other)) => *i == *other,
            (I64(i), I64(other)) => *i == *other,
            (F32(i), F32(other)) => *i == *other,
            (F64(i), F64(other)) => *i == *other,
            (V128(i), V128(other)) => *i == *other,
            (UnknownType(..), UnknownType(..)) => false,
            _ => false,
        }
    }
}
impl Eq for WasmVal {}

impl From<ffi::WasmEdge_Value> for WasmVal {
    fn from(raw_val: ffi::WasmEdge_Value) -> Self {
        unsafe {
            if ffi::WasmEdge_ValTypeIsI32(raw_val.Type) {
                return WasmVal::I32(ffi::WasmEdge_ValueGetI32(raw_val));
            } else if ffi::WasmEdge_ValTypeIsI64(raw_val.Type) {
                return WasmVal::I64(ffi::WasmEdge_ValueGetI64(raw_val));
            } else if ffi::WasmEdge_ValTypeIsF32(raw_val.Type) {
                return WasmVal::F32(ffi::WasmEdge_ValueGetF32(raw_val));
            } else if ffi::WasmEdge_ValTypeIsF64(raw_val.Type) {
                return WasmVal::F64(ffi::WasmEdge_ValueGetF64(raw_val));
            } else if ffi::WasmEdge_ValTypeIsV128(raw_val.Type) {
                return WasmVal::V128(ffi::WasmEdge_ValueGetV128(raw_val));
            } else {
                #[cfg(debug_assertions)]
                panic!("Received an unexpected type {:?}.", raw_val.Type);

                #[cfg(not(debug_assertions))]
                {
                    log::error!("Received an unexpected type {:?}.", raw_val.Type);
                    WasmVal::UnknownType(raw_val)
                }
            }
        }
    }
}

impl Into<ffi::WasmEdge_Value> for WasmVal {
    fn into(self) -> ffi::WasmEdge_Value {
        unsafe {
            match self {
                WasmVal::I32(n) => ffi::WasmEdge_ValueGenI32(n),
                WasmVal::I64(n) => ffi::WasmEdge_ValueGenI64(n),
                WasmVal::F32(n) => ffi::WasmEdge_ValueGenF32(n),
                WasmVal::F64(n) => ffi::WasmEdge_ValueGenF64(n),
                WasmVal::V128(n) => ffi::WasmEdge_ValueGenV128(n),
                WasmVal::UnknownType(v) => v,
            }
        }
    }
}

impl Into<ffi::WasmEdge_Value> for &WasmVal {
    fn into(self) -> ffi::WasmEdge_Value {
        unsafe {
            match self {
                WasmVal::I32(n) => ffi::WasmEdge_ValueGenI32(*n),
                WasmVal::I64(n) => ffi::WasmEdge_ValueGenI64(*n),
                WasmVal::F32(n) => ffi::WasmEdge_ValueGenF32(*n),
                WasmVal::F64(n) => ffi::WasmEdge_ValueGenF64(*n),
                WasmVal::V128(n) => ffi::WasmEdge_ValueGenV128(*n),
                WasmVal::UnknownType(v) => v.clone(),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ValType {
    /// 32-bit integer.
    ///
    /// Integers are not inherently signed or unsigned, their interpretation is determined by individual operations.
    I32,
    /// 64-bit integer.
    ///
    /// Integers are not inherently signed or unsigned, their interpretation is determined by individual operations.
    I64,
    /// 32-bit floating-point data as defined by the [IEEE 754-2019](https://ieeexplore.ieee.org/document/8766229).
    F32,
    /// 64-bit floating-point data as defined by the [IEEE 754-2019](https://ieeexplore.ieee.org/document/8766229).
    F64,
    /// 128-bit vector of packed integer or floating-point data.
    ///
    /// The packed data can be interpreted as signed or unsigned integers, single or double precision floating-point
    /// values, or a single 128 bit type. The interpretation is determined by individual operations.
    V128,
    /// A reference to a host function.
    FuncRef,
    /// A reference to object.
    ExternRef,
    /// A reference that unsupported by c-api.
    UnsupportedRef,
}

impl From<ffi::WasmEdge_ValType> for ValType {
    fn from(value: ffi::WasmEdge_ValType) -> Self {
        unsafe {
            if ffi::WasmEdge_ValTypeIsI32(value) {
                ValType::I32
            } else if ffi::WasmEdge_ValTypeIsI64(value) {
                ValType::I64
            } else if ffi::WasmEdge_ValTypeIsF32(value) {
                ValType::F32
            } else if ffi::WasmEdge_ValTypeIsF64(value) {
                ValType::F64
            } else if ffi::WasmEdge_ValTypeIsV128(value) {
                ValType::V128
            } else if ffi::WasmEdge_ValTypeIsRef(value) {
                if ffi::WasmEdge_ValTypeIsFuncRef(value) {
                    ValType::FuncRef
                } else if ffi::WasmEdge_ValTypeIsExternRef(value) {
                    ValType::ExternRef
                } else {
                    log::warn!(
                        "capi unsupport WasmEdge_RefType `{:x}`",
                        u64::from_be_bytes(value.Data)
                    );
                    ValType::UnsupportedRef
                }
            } else {
                log::warn!(
                    "unknown WasmEdge_ValType `{:x}`",
                    u64::from_be_bytes(value.Data)
                );
                ValType::UnsupportedRef
            }
        }
    }
}

impl From<ValType> for ffi::WasmEdge_ValType {
    fn from(value: ValType) -> Self {
        unsafe {
            match value {
                ValType::I32 => ffi::WasmEdge_ValTypeGenI32(),
                ValType::I64 => ffi::WasmEdge_ValTypeGenI64(),
                ValType::F32 => ffi::WasmEdge_ValTypeGenF32(),
                ValType::F64 => ffi::WasmEdge_ValTypeGenF64(),
                ValType::V128 => ffi::WasmEdge_ValTypeGenV128(),
                ValType::FuncRef => ffi::WasmEdge_ValTypeGenFuncRef(),
                ValType::ExternRef => ffi::WasmEdge_ValTypeGenExternRef(),
                // C API is temporarily unsupported.
                ValType::UnsupportedRef => ffi::WasmEdge_ValTypeGenExternRef(),
            }
        }
    }
}
