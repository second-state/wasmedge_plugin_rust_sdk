use std::{ffi::CString, fmt::Debug};

#[cfg(feature = "wasm_ref")]
use super::instance::function::{FuncRef, InnerFunc};

use ffi::WasmEdge_HeapTypeCode_None;
use wasmedge_sys_ffi as ffi;

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

#[cfg(feature = "wasm_ref")]
#[derive(Debug, Clone)]
pub struct Extern {
    ctx: *mut std::ffi::c_void,
}
#[cfg(feature = "wasm_ref")]
impl Extern {
    pub unsafe fn new<T>(ptr: *mut T) -> Self {
        Extern { ctx: ptr.cast() }
    }

    pub const fn cast<T>(&self) -> *mut T {
        self.ctx.cast()
    }
}

#[derive(Debug, Clone)]
pub enum WasmVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(i128),
    #[cfg(feature = "wasm_ref")]
    FuncRef(FuncRef),
    #[cfg(feature = "wasm_ref")]
    ExternRef(Extern),
    None,
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
            #[cfg(feature = "wasm_ref")]
            (FuncRef(i), FuncRef(other)) => i.inner.0 == other.inner.0,
            #[cfg(feature = "wasm_ref")]
            (ExternRef(i), ExternRef(other)) => i.ctx == other.ctx,
            (None, None) => true,
            _ => false,
        }
    }
}
impl Eq for WasmVal {}

impl From<ffi::WasmEdge_Value> for WasmVal {
    fn from(raw_val: ffi::WasmEdge_Value) -> Self {
        unsafe {
            match raw_val.Type.TypeCode {
                ffi::WasmEdge_ValTypeCode_I32 => WasmVal::I32(ffi::WasmEdge_ValueGetI32(raw_val)),
                ffi::WasmEdge_ValTypeCode_I64 => WasmVal::I64(ffi::WasmEdge_ValueGetI64(raw_val)),
                ffi::WasmEdge_ValTypeCode_F32 => WasmVal::F32(ffi::WasmEdge_ValueGetF32(raw_val)),
                ffi::WasmEdge_ValTypeCode_F64 => WasmVal::F64(ffi::WasmEdge_ValueGetF64(raw_val)),
                ffi::WasmEdge_ValTypeCode_V128 => {
                    WasmVal::V128(ffi::WasmEdge_ValueGetV128(raw_val))
                }
                #[cfg(feature = "wasm_ref")]
                ffi::WasmEdge_RefTypeCode_Ref => match raw_val.Type.Ext.HeapType.HeapTypeCode {
                    ffi::WasmEdge_HeapTypeCode_Any => todo!(),
                    ffi::WasmEdge_HeapTypeCode_Func => {
                        let fun_ref = ffi::WasmEdge_ValueGetFuncRef(raw_val);
                        WasmVal::FuncRef(FuncRef {
                            inner: InnerFunc(fun_ref),
                        })
                    }
                    ffi::WasmEdge_HeapTypeCode_Extern => {
                        let extern_ref = ffi::WasmEdge_ValueGetExternRef(raw_val);
                        WasmVal::ExternRef(Extern { ctx: extern_ref })
                    }
                    _ => todo!(),
                },
                #[cfg(feature = "wasm_ref")]
                ffi::WasmEdge_RefTypeCode_RefNull => {
                    if ffi::WasmEdge_ValueIsNullRef(raw_val) {
                        WasmVal::None
                    } else {
                        match raw_val.Type.Ext.HeapType.HeapTypeCode {
                            ffi::WasmEdge_HeapTypeCode_Any => todo!(),
                            ffi::WasmEdge_HeapTypeCode_Func => {
                                let fun_ref = ffi::WasmEdge_ValueGetFuncRef(raw_val);
                                WasmVal::FuncRef(FuncRef {
                                    inner: InnerFunc(fun_ref),
                                })
                            }
                            ffi::WasmEdge_HeapTypeCode_Extern => {
                                let extern_ref = ffi::WasmEdge_ValueGetExternRef(raw_val);
                                WasmVal::ExternRef(Extern { ctx: extern_ref })
                            }
                            _ => todo!(),
                        }
                    }
                }
                _ => WasmVal::None,
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
                #[cfg(feature = "wasm_ref")]
                WasmVal::FuncRef(r) => {
                    // leak
                    let new_ctx = std::mem::ManuallyDrop::new(r.inner.clone());
                    ffi::WasmEdge_ValueGenFuncRef(new_ctx.0)
                }
                #[cfg(feature = "wasm_ref")]
                WasmVal::ExternRef(r) => ffi::WasmEdge_ValueGenExternRef(r.ctx),
                WasmVal::None => ffi::WasmEdge_ValueGenNullRef(ffi::WasmEdge_RefType_ExternRef),
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
                #[cfg(feature = "wasm_ref")]
                WasmVal::FuncRef(r) => {
                    // leak
                    let new_ctx = std::mem::ManuallyDrop::new(r.inner.clone());
                    ffi::WasmEdge_ValueGenFuncRef(new_ctx.0)
                }
                #[cfg(feature = "wasm_ref")]
                WasmVal::ExternRef(r) => ffi::WasmEdge_ValueGenExternRef(r.ctx),
                WasmVal::None => ffi::WasmEdge_ValueGenNullRef(ffi::WasmEdge_RefType_ExternRef),
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    #[cfg(feature = "wasm_ref")]
    FuncRef,
    #[cfg(feature = "wasm_ref")]
    ExternRef,
    #[cfg(feature = "wasm_ref")]
    AnyRef,
    #[cfg(feature = "wasm_ref")]
    StructRef,
}

const fn number_type(number_code: ffi::WasmEdge_ValTypeCode) -> ffi::WasmEdge_FullValType {
    ffi::WasmEdge_FullValType {
        TypeCode: number_code,
        Ext: ffi::WasmEdge_ValTypeExt {
            HeapType: ffi::WasmEdge_HeapType {
                HeapTypeCode: WasmEdge_HeapTypeCode_None,
                DefinedTypeIdx: 0,
            },
        },
    }
}

const fn heap_ref_type(number_code: ffi::WasmEdge_HeapTypeCode) -> ffi::WasmEdge_FullValType {
    ffi::WasmEdge_FullValType {
        TypeCode: ffi::WasmEdge_RefTypeCode_RefNull,
        Ext: ffi::WasmEdge_ValTypeExt {
            HeapType: ffi::WasmEdge_HeapType {
                HeapTypeCode: number_code,
                DefinedTypeIdx: 0,
            },
        },
    }
}

impl ValType {
    const _I32: ffi::WasmEdge_FullValType = number_type(ffi::WasmEdge_NumType_I32);
    const _I64: ffi::WasmEdge_FullValType = number_type(ffi::WasmEdge_NumType_I64);
    const _F32: ffi::WasmEdge_FullValType = number_type(ffi::WasmEdge_NumType_F32);
    const _F64: ffi::WasmEdge_FullValType = number_type(ffi::WasmEdge_NumType_F64);
    const _V128: ffi::WasmEdge_FullValType = number_type(ffi::WasmEdge_NumType_V128);
    #[cfg(feature = "wasm_ref")]
    const _FUNC_REF: ffi::WasmEdge_FullValType = heap_ref_type(ffi::WasmEdge_HeapTypeCode_Func);
    #[cfg(feature = "wasm_ref")]
    const _EXTERN_REF: ffi::WasmEdge_FullValType = heap_ref_type(ffi::WasmEdge_HeapTypeCode_Extern);
    #[cfg(feature = "wasm_ref")]
    const _ANY_REF: ffi::WasmEdge_FullValType = heap_ref_type(ffi::WasmEdge_HeapTypeCode_Any);
    #[cfg(feature = "wasm_ref")]
    const _STRUCT_REF: ffi::WasmEdge_FullValType = heap_ref_type(ffi::WasmEdge_HeapTypeCode_Struct);
}

impl Debug for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::I32 => write!(f, "ValType::I32"),
            Self::I64 => write!(f, "ValType::I64"),
            Self::F32 => write!(f, "ValType::F32"),
            Self::F64 => write!(f, "ValType::F64"),
            Self::V128 => write!(f, "ValType::V128"),
            #[cfg(feature = "wasm_ref")]
            Self::FuncRef => write!(f, "ValType::FuncRef"),
            #[cfg(feature = "wasm_ref")]
            Self::ExternRef => write!(f, "ValType::ExternRef"),
            #[cfg(feature = "wasm_ref")]
            Self::AnyRef => write!(f, "ValType::AnyRef"),
            #[cfg(feature = "wasm_ref")]
            Self::StructRef => write!(f, "ValType::StructRef"),
        }
    }
}

impl From<&ffi::WasmEdge_FullValType> for ValType {
    fn from(value: &ffi::WasmEdge_FullValType) -> Self {
        if value == &ValType::_I32 {
            ValType::I32
        } else if value == &ValType::_I64 {
            ValType::I64
        } else if value == &ValType::_F32 {
            ValType::F32
        } else if value == &ValType::_F64 {
            ValType::F64
        } else if value == &ValType::_FUNC_REF {
            ValType::FuncRef
        } else if value == &ValType::_EXTERN_REF {
            ValType::ExternRef
        } else if value == &ValType::_ANY_REF {
            ValType::AnyRef
        } else if value == &ValType::_STRUCT_REF {
            ValType::StructRef
        } else {
            panic!("[wasmedge-types] Invalid WasmEdge_ValType: {:#?}", value);
        }
    }
}

impl From<ValType> for ffi::WasmEdge_FullValType {
    fn from(value: ValType) -> Self {
        match value {
            ValType::I32 => ValType::_I32,
            ValType::I64 => ValType::_I64,
            ValType::F32 => ValType::_F32,
            ValType::F64 => ValType::_F64,
            ValType::V128 => ValType::_V128,
            ValType::FuncRef => ValType::_FUNC_REF,
            ValType::ExternRef => ValType::_EXTERN_REF,
            ValType::AnyRef => ValType::_ANY_REF,
            ValType::StructRef => ValType::_STRUCT_REF,
        }
    }
}
