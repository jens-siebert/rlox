use crate::base::parser::Expr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Invalid value.")]
    InvalidValue,
    #[error("Number expected.")]
    NumberExpected,
    #[error("Number or String expected.")]
    NumberOrStringExpected,
}

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> Result<R, RuntimeError>;
}
