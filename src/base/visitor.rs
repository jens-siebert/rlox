use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Invalid value.")]
    InvalidValue,
    #[error("Number expected.")]
    NumberExpected,
    #[error("Number or String expected.")]
    NumberOrStringExpected,
    #[error("Undefined variable.")]
    UndefinedVariable,
    #[error("Invalid argument.")]
    InvalidArgument,
    #[error("Block expected.")]
    BlockExpected,
    #[error("Number of arguments does not match number of paramters.")]
    NonMatchingNumberOfArguments,
}

pub trait Visitor<I, R> {
    fn visit(&self, input: &I) -> Result<R, RuntimeError>;
}
