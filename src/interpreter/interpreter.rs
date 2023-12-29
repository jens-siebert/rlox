use crate::base::parser::{Expr, ExprRef, LiteralValue, LiteralValueRef, Stmt, StmtRef};
use crate::base::scanner::TokenType;
use crate::base::visitor::{RuntimeError, Visitor};

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: &ExprRef) -> Result<LiteralValueRef, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &StmtRef) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    pub fn interpret(&self, statements: Vec<StmtRef>) {
        for statement in statements {
            if let Err(error) = self.execute(&statement) {
                eprintln!("{}", error);
            }
        }
    }
}

impl Visitor<Expr<'_>, LiteralValueRef> for Interpreter {
    fn visit(&self, input: &Expr<'_>) -> Result<LiteralValueRef, RuntimeError> {
        match input {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                match &operator.token_type {
                    TokenType::Greater => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 > v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::GreaterEqual => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 >= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Less => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 < v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::LessEqual => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 <= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::BangEqual => Ok(LiteralValue::boolean_ref(*left? != *right?)),
                    TokenType::EqualEqual => Ok(LiteralValue::boolean_ref(*left? == *right?)),
                    TokenType::Minus => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 - v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Slash => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 / v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Star => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 * v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Plus => match (*left?, *right?) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 + v2))
                        }
                        (LiteralValue::String(v1), LiteralValue::String(v2)) => {
                            Ok(LiteralValue::string_ref(v1.clone() + v2.clone().as_str()))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => match value {
                LiteralValue::Number(value) => Ok(LiteralValue::number_ref(*value)),
                LiteralValue::String(value) => Ok(LiteralValue::string_ref(value.clone())),
                LiteralValue::Boolean(value) => Ok(LiteralValue::boolean_ref(*value)),
                LiteralValue::None => Ok(LiteralValue::none_ref()),
            },
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

                match &operator.token_type {
                    TokenType::Minus => match *right? {
                        LiteralValue::Number(value) => Ok(LiteralValue::number_ref(-value)),
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Bang => match *right? {
                        LiteralValue::Boolean(value) => Ok(LiteralValue::boolean_ref(!value)),
                        LiteralValue::None => Ok(LiteralValue::boolean_ref(true)),
                        _ => Ok(LiteralValue::boolean_ref(false)),
                    },
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
        }
    }
}

impl Visitor<Stmt<'_>, ()> for Interpreter {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
        }
    }
}
