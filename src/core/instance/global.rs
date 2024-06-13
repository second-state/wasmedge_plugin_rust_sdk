use crate::core::types::WasmVal;

#[derive(Debug, Clone)]
pub struct ConstGlobal {
    pub name: String,
    pub val: WasmVal,
}

#[derive(Debug, Clone)]
pub struct MutGlobal {
    pub name: String,
    pub val: WasmVal,
}

#[derive(Debug, Clone)]
pub enum Global {
    Const(ConstGlobal),
    Mut(MutGlobal),
}
