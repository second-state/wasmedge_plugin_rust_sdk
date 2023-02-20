use crate::error::{
    CoreCommonError, CoreError, CoreExecutionError, CoreInstantiationError, CoreLoadError,
    CoreValidationError,
};

use wasmedge_sys_ffi::{self as ffi, WasmEdge_Result, WasmEdge_ResultGetCode, WasmEdge_ResultOK};

// Checks the result of a `FFI` function.
pub(crate) fn check(result: WasmEdge_Result) -> Result<(), CoreError> {
    let category = unsafe { ffi::WasmEdge_ResultGetCategory(result) };
    let code = unsafe {
        if !WasmEdge_ResultOK(result) {
            WasmEdge_ResultGetCode(result)
        } else {
            0u32
        }
    };

    match category {
        ffi::WasmEdge_ErrCategory_UserLevelError => Err(CoreError::User(code)),
        ffi::WasmEdge_ErrCategory_WASM => gen_runtime_error(code),
        _ => Err(CoreError::runtime()),
    }
}

fn gen_runtime_error(code: u32) -> Result<(), CoreError> {
    match code {
        // Success or terminated (exit and return success)
        0x00 => Ok(()),

        // Common errors
        0x01 => Err(CoreError::Common(CoreCommonError::Terminated)),
        0x02 => Err(CoreError::Common(CoreCommonError::RuntimeError)),
        0x03 => Err(CoreError::Common(CoreCommonError::CostLimitExceeded)),
        0x04 => Err(CoreError::Common(CoreCommonError::WrongVMWorkflow)),
        0x05 => Err(CoreError::Common(CoreCommonError::FuncNotFound)),
        0x06 => Err(CoreError::Common(CoreCommonError::AOTDisabled)),
        0x07 => Err(CoreError::Common(CoreCommonError::Interrupted)),
        0x08 => Err(CoreError::Common(CoreCommonError::NotValidated)),
        0x09 => Err(CoreError::Common(CoreCommonError::UserDefError)),
        0x0A => Err(CoreError::Asyncify),
        0x0B => Err(CoreError::Yield),

        // Load phase
        0x20 => Err(CoreError::Load(CoreLoadError::IllegalPath)),
        0x21 => Err(CoreError::Load(CoreLoadError::ReadError)),
        0x22 => Err(CoreError::Load(CoreLoadError::UnexpectedEnd)),
        0x23 => Err(CoreError::Load(CoreLoadError::MalformedMagic)),
        0x24 => Err(CoreError::Load(CoreLoadError::MalformedVersion)),
        0x25 => Err(CoreError::Load(CoreLoadError::MalformedSection)),
        0x26 => Err(CoreError::Load(CoreLoadError::SectionSizeMismatch)),
        0x27 => Err(CoreError::Load(CoreLoadError::NameSizeOutOfBounds)),
        0x28 => Err(CoreError::Load(CoreLoadError::JunkSection)),
        0x29 => Err(CoreError::Load(CoreLoadError::IncompatibleFuncCode)),
        0x2A => Err(CoreError::Load(CoreLoadError::IncompatibleDataCount)),
        0x2B => Err(CoreError::Load(CoreLoadError::DataCountRequired)),
        0x2C => Err(CoreError::Load(CoreLoadError::MalformedImportKind)),
        0x2D => Err(CoreError::Load(CoreLoadError::MalformedExportKind)),
        0x2E => Err(CoreError::Load(CoreLoadError::ExpectedZeroByte)),
        0x2F => Err(CoreError::Load(CoreLoadError::InvalidMut)),
        0x30 => Err(CoreError::Load(CoreLoadError::TooManyLocals)),
        0x31 => Err(CoreError::Load(CoreLoadError::MalformedValType)),
        0x32 => Err(CoreError::Load(CoreLoadError::MalformedElemType)),
        0x33 => Err(CoreError::Load(CoreLoadError::MalformedRefType)),
        0x34 => Err(CoreError::Load(CoreLoadError::MalformedUTF8)),
        0x35 => Err(CoreError::Load(CoreLoadError::IntegerTooLarge)),
        0x36 => Err(CoreError::Load(CoreLoadError::IntegerTooLong)),
        0x37 => Err(CoreError::Load(CoreLoadError::IllegalOpCode)),
        0x38 => Err(CoreError::Load(CoreLoadError::IllegalGrammar)),

        // Validation phase
        0x40 => Err(CoreError::Validation(CoreValidationError::InvalidAlignment)),
        0x41 => Err(CoreError::Validation(CoreValidationError::TypeCheckFailed)),
        0x42 => Err(CoreError::Validation(CoreValidationError::InvalidLabelIdx)),
        0x43 => Err(CoreError::Validation(CoreValidationError::InvalidLocalIdx)),
        0x44 => Err(CoreError::Validation(
            CoreValidationError::InvalidFuncTypeIdx,
        )),
        0x45 => Err(CoreError::Validation(CoreValidationError::InvalidFuncIdx)),
        0x46 => Err(CoreError::Validation(CoreValidationError::InvalidTableIdx)),
        0x47 => Err(CoreError::Validation(CoreValidationError::InvalidMemoryIdx)),
        0x48 => Err(CoreError::Validation(CoreValidationError::InvalidGlobalIdx)),
        0x49 => Err(CoreError::Validation(CoreValidationError::InvalidElemIdx)),
        0x4A => Err(CoreError::Validation(CoreValidationError::InvalidDataIdx)),
        0x4B => Err(CoreError::Validation(CoreValidationError::InvalidRefIdx)),
        0x4C => Err(CoreError::Validation(
            CoreValidationError::ConstExprRequired,
        )),
        0x4D => Err(CoreError::Validation(CoreValidationError::DupExportName)),
        0x4E => Err(CoreError::Validation(CoreValidationError::ImmutableGlobal)),
        0x4F => Err(CoreError::Validation(
            CoreValidationError::InvalidResultArity,
        )),
        0x50 => Err(CoreError::Validation(CoreValidationError::MultiTables)),
        0x51 => Err(CoreError::Validation(CoreValidationError::MultiMemories)),
        0x52 => Err(CoreError::Validation(CoreValidationError::InvalidLimit)),
        0x53 => Err(CoreError::Validation(CoreValidationError::InvalidMemPages)),
        0x54 => Err(CoreError::Validation(CoreValidationError::InvalidStartFunc)),
        0x55 => Err(CoreError::Validation(CoreValidationError::InvalidLaneIdx)),

        // Instantiation phase
        0x60 => Err(CoreError::Instantiation(
            CoreInstantiationError::ModuleNameConflict,
        )),
        0x61 => Err(CoreError::Instantiation(
            CoreInstantiationError::IncompatibleImportType,
        )),
        0x62 => Err(CoreError::Instantiation(
            CoreInstantiationError::UnknownImport,
        )),
        0x63 => Err(CoreError::Instantiation(
            CoreInstantiationError::DataSegDoesNotFit,
        )),
        0x64 => Err(CoreError::Instantiation(
            CoreInstantiationError::ElemSegDoesNotFit,
        )),

        // Execution phase
        0x80 => Err(CoreError::Execution(
            CoreExecutionError::WrongInstanceAddress,
        )),
        0x81 => Err(CoreError::Execution(CoreExecutionError::WrongInstanceIndex)),
        0x82 => Err(CoreError::Execution(CoreExecutionError::InstrTypeMismatch)),
        0x83 => Err(CoreError::Execution(CoreExecutionError::FuncTypeMismatch)),
        0x84 => Err(CoreError::Execution(CoreExecutionError::DivideByZero)),
        0x85 => Err(CoreError::Execution(CoreExecutionError::IntegerOverflow)),
        0x86 => Err(CoreError::Execution(CoreExecutionError::InvalidConvToInt)),
        0x87 => Err(CoreError::Execution(CoreExecutionError::TableOutOfBounds)),
        0x88 => Err(CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)),
        0x89 => Err(CoreError::Execution(CoreExecutionError::Unreachable)),
        0x8A => Err(CoreError::Execution(
            CoreExecutionError::UninitializedElement,
        )),
        0x8B => Err(CoreError::Execution(CoreExecutionError::UndefinedElement)),
        0x8C => Err(CoreError::Execution(
            CoreExecutionError::IndirectCallTypeMismatch,
        )),
        0x8D => Err(CoreError::Execution(CoreExecutionError::HostFuncFailed)),
        0x8E => Err(CoreError::Execution(CoreExecutionError::RefTypeMismatch)),
        0x8F => Err(CoreError::Execution(
            CoreExecutionError::UnalignedAtomicAccess,
        )),
        0x90 => Err(CoreError::Execution(
            CoreExecutionError::WaitOnUnsharedMemory,
        )),

        _ => Err(CoreError::Common(CoreCommonError::RuntimeError)),
    }
}

impl Into<WasmEdge_Result> for CoreError {
    fn into(self) -> WasmEdge_Result {
        let code = match self {
            // Common errors
            CoreError::Common(CoreCommonError::Terminated) => 0x01,
            CoreError::Common(CoreCommonError::RuntimeError) => 0x02,
            CoreError::Common(CoreCommonError::CostLimitExceeded) => 0x03,
            CoreError::Common(CoreCommonError::WrongVMWorkflow) => 0x04,
            CoreError::Common(CoreCommonError::FuncNotFound) => 0x05,
            CoreError::Common(CoreCommonError::AOTDisabled) => 0x06,
            CoreError::Common(CoreCommonError::Interrupted) => 0x07,
            CoreError::Common(CoreCommonError::NotValidated) => 0x08,
            CoreError::Common(CoreCommonError::UserDefError) => 0x09,

            // Load phase
            CoreError::Load(CoreLoadError::IllegalPath) => 0x20,
            CoreError::Load(CoreLoadError::ReadError) => 0x21,
            CoreError::Load(CoreLoadError::UnexpectedEnd) => 0x22,
            CoreError::Load(CoreLoadError::MalformedMagic) => 0x23,
            CoreError::Load(CoreLoadError::MalformedVersion) => 0x24,
            CoreError::Load(CoreLoadError::MalformedSection) => 0x25,
            CoreError::Load(CoreLoadError::SectionSizeMismatch) => 0x26,
            CoreError::Load(CoreLoadError::NameSizeOutOfBounds) => 0x27,
            CoreError::Load(CoreLoadError::JunkSection) => 0x28,
            CoreError::Load(CoreLoadError::IncompatibleFuncCode) => 0x29,
            CoreError::Load(CoreLoadError::IncompatibleDataCount) => 0x2A,
            CoreError::Load(CoreLoadError::DataCountRequired) => 0x2B,
            CoreError::Load(CoreLoadError::MalformedImportKind) => 0x2C,
            CoreError::Load(CoreLoadError::MalformedExportKind) => 0x2D,
            CoreError::Load(CoreLoadError::ExpectedZeroByte) => 0x2E,
            CoreError::Load(CoreLoadError::InvalidMut) => 0x2F,
            CoreError::Load(CoreLoadError::TooManyLocals) => 0x30,
            CoreError::Load(CoreLoadError::MalformedValType) => 0x31,
            CoreError::Load(CoreLoadError::MalformedElemType) => 0x32,
            CoreError::Load(CoreLoadError::MalformedRefType) => 0x33,
            CoreError::Load(CoreLoadError::MalformedUTF8) => 0x34,
            CoreError::Load(CoreLoadError::IntegerTooLarge) => 0x35,
            CoreError::Load(CoreLoadError::IntegerTooLong) => 0x36,
            CoreError::Load(CoreLoadError::IllegalOpCode) => 0x37,
            CoreError::Load(CoreLoadError::IllegalGrammar) => 0x38,

            // Validation phase
            CoreError::Validation(CoreValidationError::InvalidAlignment) => 0x40,
            CoreError::Validation(CoreValidationError::TypeCheckFailed) => 0x41,
            CoreError::Validation(CoreValidationError::InvalidLabelIdx) => 0x42,
            CoreError::Validation(CoreValidationError::InvalidLocalIdx) => 0x43,
            CoreError::Validation(CoreValidationError::InvalidFuncTypeIdx) => 0x44,
            CoreError::Validation(CoreValidationError::InvalidFuncIdx) => 0x45,
            CoreError::Validation(CoreValidationError::InvalidTableIdx) => 0x46,
            CoreError::Validation(CoreValidationError::InvalidMemoryIdx) => 0x47,
            CoreError::Validation(CoreValidationError::InvalidGlobalIdx) => 0x48,
            CoreError::Validation(CoreValidationError::InvalidElemIdx) => 0x49,
            CoreError::Validation(CoreValidationError::InvalidDataIdx) => 0x4A,
            CoreError::Validation(CoreValidationError::InvalidRefIdx) => 0x4B,
            CoreError::Validation(CoreValidationError::ConstExprRequired) => 0x4C,
            CoreError::Validation(CoreValidationError::DupExportName) => 0x4D,
            CoreError::Validation(CoreValidationError::ImmutableGlobal) => 0x4E,
            CoreError::Validation(CoreValidationError::InvalidResultArity) => 0x4F,
            CoreError::Validation(CoreValidationError::MultiTables) => 0x50,
            CoreError::Validation(CoreValidationError::MultiMemories) => 0x51,
            CoreError::Validation(CoreValidationError::InvalidLimit) => 0x52,
            CoreError::Validation(CoreValidationError::InvalidMemPages) => 0x53,
            CoreError::Validation(CoreValidationError::InvalidStartFunc) => 0x54,
            CoreError::Validation(CoreValidationError::InvalidLaneIdx) => 0x55,

            // Instantiation phase
            CoreError::Instantiation(CoreInstantiationError::ModuleNameConflict) => 0x60,
            CoreError::Instantiation(CoreInstantiationError::IncompatibleImportType) => 0x61,
            CoreError::Instantiation(CoreInstantiationError::UnknownImport) => 0x62,
            CoreError::Instantiation(CoreInstantiationError::DataSegDoesNotFit) => 0x63,
            CoreError::Instantiation(CoreInstantiationError::ElemSegDoesNotFit) => 0x64,

            // Execution phase
            CoreError::Execution(CoreExecutionError::WrongInstanceAddress) => 0x80,
            CoreError::Execution(CoreExecutionError::WrongInstanceIndex) => 0x81,
            CoreError::Execution(CoreExecutionError::InstrTypeMismatch) => 0x82,
            CoreError::Execution(CoreExecutionError::FuncTypeMismatch) => 0x83,
            CoreError::Execution(CoreExecutionError::DivideByZero) => 0x84,
            CoreError::Execution(CoreExecutionError::IntegerOverflow) => 0x85,
            CoreError::Execution(CoreExecutionError::InvalidConvToInt) => 0x86,
            CoreError::Execution(CoreExecutionError::TableOutOfBounds) => 0x87,
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) => 0x88,
            CoreError::Execution(CoreExecutionError::Unreachable) => 0x89,
            CoreError::Execution(CoreExecutionError::UninitializedElement) => 0x8A,
            CoreError::Execution(CoreExecutionError::UndefinedElement) => 0x8B,
            CoreError::Execution(CoreExecutionError::IndirectCallTypeMismatch) => 0x8C,
            CoreError::Execution(CoreExecutionError::HostFuncFailed) => 0x8D,
            CoreError::Execution(CoreExecutionError::RefTypeMismatch) => 0x8E,
            CoreError::Execution(CoreExecutionError::UnalignedAtomicAccess) => 0x8F,
            CoreError::Execution(CoreExecutionError::WaitOnUnsharedMemory) => 0x90,
            CoreError::User(user_code) => {
                return unsafe {
                    ffi::WasmEdge_ResultGen(ffi::WasmEdge_ErrCategory_UserLevelError, user_code)
                }
            }
            // sync wasmedge
            CoreError::Asyncify => 0x0A,
            CoreError::Yield => 0x0B,
        };
        unsafe { ffi::WasmEdge_ResultGen(ffi::WasmEdge_ErrCategory_WASM, code) }
    }
}
