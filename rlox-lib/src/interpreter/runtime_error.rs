use crate::base::expr_result::ExprResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Output error.")]
    OutputError,
    #[error("{line:?}: Invalid value!")]
    InvalidValue { line: usize },
    #[error("{line:?}: Number expected!")]
    NumberExpected { line: usize },
    #[error("{line:?}: Number or String expected!")]
    NumberOrStringExpected { line: usize },
    #[error("{line:?}: Undefined variable {name:?}!")]
    UndefinedVariable { line: usize, name: String },
    #[error("{line:?}: Undefined callable!")]
    UndefinedCallable { line: usize },
    #[error("{line:?}: Invalid argument!")]
    InvalidArgument { line: usize },
    #[error("{line:?}: Block expected!")]
    BlockExpected { line: usize },
    #[error("{line:?}: Number of arguments does not match number of parameters!")]
    NonMatchingNumberOfArguments { line: usize },
    #[error("{line:?}: Can't read local variable in its own initializer!")]
    VariableNotDefined { line: usize },
    #[error("{line:?}: Already a variable with this name in this scope!")]
    VariableAlreadyDefinedInScope { line: usize },
    #[error("{line:?}: Can't return from top-level code!")]
    TopLevelReturn { line: usize },
    #[error("{line:?}: Undefined property!")]
    UndefinedProperty { line: usize },
    #[error("{line:?}: Only instances have properties!")]
    InvalidPropertyAccess { line: usize },
    #[error("{line:?}: Only instances have fields!")]
    InvalidFieldAccess { line: usize },
    #[error("{line:?}: Can't use 'this' outside of a class!")]
    ThisOutsideClass { line: usize },
    #[error("{line:?}: Can't return a value from an initializer!")]
    ReturnValueFromInitializer { line: usize },
    #[error("{line:?}: A class can't inherit from itself!")]
    SuperclassSelfInheritance { line: usize },
    #[error("{line:?}: Superclass must be a class!")]
    SuperclassInvalidType { line: usize },
    #[error("{line:?}: Can't use 'super' outside of a class!")]
    SuperOutsideClass { line: usize },
    #[error("{line:?}: Can't use 'super' in a class with no superclass!")]
    SuperWithoutSuperclass { line: usize },
    #[error(transparent)]
    Return { ret_val: ExprResult },
}
