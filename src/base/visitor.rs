use crate::base::parser::Expr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {}

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> Result<R, RuntimeError>;
}
