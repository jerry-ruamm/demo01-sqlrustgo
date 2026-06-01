pub mod error;
#[allow(clippy::module_inception)]
pub mod executor;
pub mod operators;

pub use error::ExecError;
pub use executor::Executor;
pub use operators::Operator;
