mod core;
pub mod error;
mod sdk;
#[cfg(feature = "embedded")]
pub use crate::core::ast_module;
pub use crate::core::config;
pub use crate::core::executor;
pub use crate::core::instance::memory;
pub use crate::core::types;
pub use sdk::*;
mod utils;
