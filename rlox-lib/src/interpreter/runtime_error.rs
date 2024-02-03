use crate::base::expr_result::ExprResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Output error.")]
    OutputError,
    #[error("Invalid value.")]
    InvalidValue,
    #[error("Number expected.")]
    NumberExpected,
    #[error("Number or String expected.")]
    NumberOrStringExpected,
    #[error("Undefined variable {name}.")]
    UndefinedVariable { name: String },
    #[error("Undefined callable.")]
    UndefinedCallable,
    #[error("Invalid argument.")]
    InvalidArgument,
    #[error("Block expected.")]
    BlockExpected,
    #[error("Number of arguments does not match number of parameters.")]
    NonMatchingNumberOfArguments,
    #[error("Can't read local variable in its own initializer.")]
    VariableNotDefined,
    #[error("Already a variable with this name in this scope.")]
    VariableAlreadyDefinedInScope,
    #[error("Can't return from top-level code.")]
    TopLevelReturn,
    #[error(transparent)]
    Return { ret_val: ExprResult },
}
