use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("{0}")]
    Common(CoreCommonError),
    #[error("{0}")]
    Load(CoreLoadError),
    #[error("{0}")]
    Validation(CoreValidationError),
    #[error("{0}")]
    Instantiation(CoreInstantiationError),
    #[error("{0}")]
    Execution(CoreExecutionError),
    #[error("User error: {0}")]
    User(u32),
    #[error("Asyncify error")]
    Asyncify,
    #[error("Yield")]
    Yield,
}

impl CoreError {
    pub(crate) fn runtime() -> Self {
        CoreError::Common(CoreCommonError::RuntimeError)
    }

    pub fn terminated() -> Self {
        CoreError::Common(CoreCommonError::Terminated)
    }

    pub fn is_yield(&self) -> bool {
        if let Self::Yield = self {
            true
        } else {
            false
        }
    }
}

/// The error type for the common errors from WasmEdge Core.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreCommonError {
    #[error("process terminated")]
    Terminated,
    #[error("generic runtime error")]
    RuntimeError,
    #[error("cost limit exceeded")]
    CostLimitExceeded,
    #[error("wrong VM workflow")]
    WrongVMWorkflow,
    #[error("wasm function not found")]
    FuncNotFound,
    #[error("AOT runtime is disabled in this build")]
    AOTDisabled,
    #[error("execution interrupted")]
    Interrupted,
    #[error("user defined error code")]
    UserDefError,
    #[error("wasm module hasn't passed validation yet")]
    NotValidated,
}

/// The error type for the load phase from WasmEdge Core.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreLoadError {
    #[error("Invalid file path")]
    IllegalPath,
    #[error("Read error")]
    ReadError,
    #[error("unexpected end")]
    UnexpectedEnd,
    #[error("magic header not detected")]
    MalformedMagic,
    #[error("unknown binary version")]
    MalformedVersion,
    #[error("malformed section id")]
    MalformedSection,
    #[error("section size mismatch")]
    SectionSizeMismatch,
    #[error("length out of bounds")]
    NameSizeOutOfBounds,
    #[error("unexpected content after last section")]
    JunkSection,
    #[error("function and code section have inconsistent lengths")]
    IncompatibleFuncCode,
    #[error("data count and data section have inconsistent lengths")]
    IncompatibleDataCount,
    #[error("data count section required")]
    DataCountRequired,
    #[error("malformed import kind")]
    MalformedImportKind,
    #[error("malformed export kind")]
    MalformedExportKind,
    #[error("zero byte expected")]
    ExpectedZeroByte,
    #[error("malformed mutability")]
    InvalidMut,
    #[error("too many locals")]
    TooManyLocals,
    #[error("malformed value type")]
    MalformedValType,
    #[error("malformed element type")]
    MalformedElemType,
    #[error("malformed reference type")]
    MalformedRefType,
    #[error("malformed UTF-8 encoding")]
    MalformedUTF8,
    #[error("integer too large")]
    IntegerTooLarge,
    #[error("integer representation too long")]
    IntegerTooLong,
    #[error("illegal opcode")]
    IllegalOpCode,
    #[error("invalid wasm grammar")]
    IllegalGrammar,
}

/// The error type for the validation phase from WasmEdge Core.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreValidationError {
    #[error("alignment must not be larger than natural")]
    InvalidAlignment,
    #[error("type mismatch")]
    TypeCheckFailed,
    #[error("unknown label")]
    InvalidLabelIdx,
    #[error("unknown local")]
    InvalidLocalIdx,
    #[error("unknown type")]
    InvalidFuncTypeIdx,
    #[error("unknown function")]
    InvalidFuncIdx,
    #[error("unknown table")]
    InvalidTableIdx,
    #[error("unknown memory")]
    InvalidMemoryIdx,
    #[error("unknown global")]
    InvalidGlobalIdx,
    #[error("unknown elem segment")]
    InvalidElemIdx,
    #[error("unknown data segment")]
    InvalidDataIdx,
    #[error("undeclared function reference")]
    InvalidRefIdx,
    #[error("constant expression required")]
    ConstExprRequired,
    #[error("duplicate export name")]
    DupExportName,
    #[error("global is immutable")]
    ImmutableGlobal,
    #[error("invalid result arity")]
    InvalidResultArity,
    #[error("multiple tables")]
    MultiTables,
    #[error("multiple memories")]
    MultiMemories,
    #[error("size minimum must not be greater than maximum")]
    InvalidLimit,
    #[error("memory size must be at most 65536 pages (4GiB)")]
    InvalidMemPages,
    #[error("start function")]
    InvalidStartFunc,
    #[error("invalid lane index")]
    InvalidLaneIdx,
}

/// The error type for the instantiation phase from WasmEdge Core.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreInstantiationError {
    #[error("module name conflict")]
    ModuleNameConflict,
    #[error("incompatible import type")]
    IncompatibleImportType,
    #[error("unknown import")]
    UnknownImport,
    #[error("data segment does not fit")]
    DataSegDoesNotFit,
    #[error("elements segment does not fit")]
    ElemSegDoesNotFit,
}

/// The error type for the execution phase from WasmEdge Core.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum CoreExecutionError {
    #[error("wrong instance address")]
    WrongInstanceAddress,
    #[error("wrong instance index")]
    WrongInstanceIndex,
    #[error("instruction type mismatch")]
    InstrTypeMismatch,
    #[error("function type mismatch")]
    FuncTypeMismatch,
    #[error("integer divide by zero")]
    DivideByZero,
    #[error("integer overflow")]
    IntegerOverflow,
    #[error("invalid conversion to integer")]
    InvalidConvToInt,
    #[error("out of bounds table access")]
    TableOutOfBounds,
    #[error("out of bounds memory access")]
    MemoryOutOfBounds,
    #[error("unreachable")]
    Unreachable,
    #[error("uninitialized element")]
    UninitializedElement,
    #[error("undefined element")]
    UndefinedElement,
    #[error("indirect call type mismatch")]
    IndirectCallTypeMismatch,
    #[error("host function failed")]
    HostFuncFailed,
    #[error("reference type mismatch")]
    RefTypeMismatch,
    #[error("unaligned atomic")]
    UnalignedAtomicAccess,
    #[error("wait on unshared memory")]
    WaitOnUnsharedMemory,
}

/// The error types for WasmEdge Instance.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum InstanceError {
    #[error("Fail to create Instance context")]
    Create,
    #[error("Fail to create ImportModule context")]
    CreateImportModule,
    #[error("Not found the target function ({0})")]
    NotFoundFunc(String),
    #[error("Not found the target table ({0})")]
    NotFoundTable(String),
    #[error("Not found the target memory ({0})")]
    NotFoundMem(String),
    #[error("Fail to write memory ({0})")]
    WriteMem(String),
    #[error("Not found the target global ({0})")]
    NotFoundGlobal(String),
    #[error("Not found the target mutable global ({0})")]
    NotFoundMutGlobal(String),
    #[error("Found an interior nul byte")]
    FoundNulByte(#[from] std::ffi::NulError),
}
