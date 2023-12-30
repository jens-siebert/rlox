use crate::base::expr::{Expr, ExprRef};
use crate::base::literal::{LiteralValue, LiteralValueRef};
use crate::base::scanner::TokenType;
use crate::base::stmt::{Stmt, StmtRef};
use crate::base::visitor::{RuntimeError, Visitor};
use crate::interpreter::environment::{Environment, EnvironmentRef};
use std::cell::RefCell;

pub struct Interpreter {
    environment: RefCell<EnvironmentRef>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Environment::new_ref()),
        }
    }

    fn is_truthy(&self, literal_value: &LiteralValueRef) -> bool {
        match **literal_value {
            LiteralValue::Boolean(value) => value,
            LiteralValue::None => false,
            _ => true,
        }
    }

    fn evaluate(&self, expr: &ExprRef) -> Result<LiteralValueRef, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &StmtRef) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(
        &self,
        statements: &Vec<StmtRef>,
        environment: EnvironmentRef,
    ) -> Result<(), RuntimeError> {
        self.environment.replace(environment);

        for statement in statements {
            self.execute(statement)?
        }

        let enclosing = self.environment.borrow().enclosing();
        if let Some(e) = enclosing {
            self.environment.replace(e);
        }

        Ok(())
    }

    pub fn interpret(&self, statements: Vec<StmtRef>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
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
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Greater => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 > v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::GreaterEqual => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 >= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Less => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 < v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::LessEqual => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::boolean_ref(v1 <= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::BangEqual => Ok(LiteralValue::boolean_ref(*left != *right)),
                    TokenType::EqualEqual => Ok(LiteralValue::boolean_ref(*left == *right)),
                    TokenType::Minus => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 - v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Slash => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 / v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Star => match (*left, *right) {
                        (LiteralValue::Number(v1), LiteralValue::Number(v2)) => {
                            Ok(LiteralValue::number_ref(v1 * v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Plus => match (*left, *right) {
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
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_expr = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {
                    if self.is_truthy(&left_expr) {
                        return Ok(left_expr);
                    }
                } else if !self.is_truthy(&left_expr) {
                    return Ok(left_expr);
                }

                self.evaluate(right)
            }
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Minus => match *right {
                        LiteralValue::Number(value) => Ok(LiteralValue::number_ref(-value)),
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Bang => Ok(LiteralValue::boolean_ref(!self.is_truthy(&right))),
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
            Expr::Variable { name } => match self.environment.borrow().get(&name.lexeme) {
                Ok(value) => Ok(value),
                Err(_) => Err(RuntimeError::UndefinedVariable),
            },
            Expr::Assign { name, value } => {
                let v = self.evaluate(value)?;
                self.environment.borrow_mut().assign(&name.lexeme, &v)?;
                Ok(v)
            }
        }
    }
}

impl Visitor<Stmt<'_>, ()> for Interpreter {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Block { statements } => {
                let old_scope = self.environment.borrow().clone();
                self.execute_block(statements, Environment::new_scope_ref(Some(old_scope)))
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_result = self.evaluate(condition)?;

                if self.is_truthy(&condition_result) {
                    self.execute(then_branch)?
                } else {
                    match else_branch {
                        None => {}
                        Some(branch) => self.execute(branch)?,
                    }
                }

                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.borrow_mut().define(&name.lexeme, &value);
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }

                Ok(())
            }
        }
    }
}
