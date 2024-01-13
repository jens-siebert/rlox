use crate::base::expr::{Expr, LiteralValue};
use crate::base::expr_result::{Callable, Function};
use crate::base::expr_result::{ExprResult, ExprResultRef};
use crate::base::scanner::TokenType;
use crate::base::stmt::Stmt;
use crate::base::visitor::{RuntimeError, Visitor};
use crate::interpreter::environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub(crate) environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    fn evaluate(&self, expr: &Expr) -> Result<ExprResultRef, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    pub(crate) fn execute_block(
        &self,
        statements: &Vec<Stmt>,
    ) -> Result<ExprResultRef, RuntimeError> {
        let mut return_value = ExprResult::none_ref();
        self.environment.borrow_mut().push_scope();

        for statement in statements {
            self.execute(statement)?;

            if let Stmt::Return { .. } = *statement {
                return_value = self.environment.borrow().get_return_value();
                break;
            }
        }

        self.environment.borrow_mut().pop_scope();

        Ok(return_value)
    }

    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new(Rc::new(RefCell::new(Environment::default())))
    }
}

impl Visitor<Expr, ExprResultRef> for Interpreter {
    fn visit(&self, input: &Expr) -> Result<ExprResultRef, RuntimeError> {
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
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean_ref(v1 > v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::GreaterEqual => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean_ref(v1 >= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Less => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean_ref(v1 < v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::LessEqual => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean_ref(v1 <= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::BangEqual => Ok(ExprResult::boolean_ref(*left != *right)),
                    TokenType::EqualEqual => Ok(ExprResult::boolean_ref(*left == *right)),
                    TokenType::Minus => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number_ref(v1 - v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Slash => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number_ref(v1 / v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Star => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number_ref(v1 * v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Plus => match (*left, *right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number_ref(v1 + v2))
                        }
                        (ExprResult::String(v1), ExprResult::String(v2)) => {
                            Ok(ExprResult::string_ref(v1.clone() + v2.clone().as_str()))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
            Expr::Call { callee, arguments } => {
                let call = self.evaluate(callee)?;

                if let ExprResult::Callable(callable) = *call {
                    if arguments.len() != callable.arity() {
                        return Err(RuntimeError::NonMatchingNumberOfArguments);
                    }

                    let mut args = vec![];
                    for argument in arguments {
                        args.push(self.evaluate(argument)?);
                    }

                    callable.call(self, args)
                } else {
                    Err(RuntimeError::UndefinedCallable)
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => match value {
                LiteralValue::Number(value) => Ok(ExprResult::number_ref(*value)),
                LiteralValue::String(value) => Ok(ExprResult::string_ref(value.clone())),
                LiteralValue::Boolean(value) => Ok(ExprResult::boolean_ref(*value)),
                LiteralValue::None => Ok(ExprResult::none_ref()),
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_expr = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {
                    if left_expr.is_truthy() {
                        return Ok(left_expr);
                    }
                } else if !left_expr.is_truthy() {
                    return Ok(left_expr);
                }

                self.evaluate(right)
            }
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Minus => match *right {
                        ExprResult::Number(value) => Ok(ExprResult::number_ref(-value)),
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Bang => Ok(ExprResult::boolean_ref(!right.is_truthy())),
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

impl Visitor<Stmt, ()> for Interpreter {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Block { statements } => {
                self.execute_block(statements)?;

                Ok(())
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let callable = Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };

                self.environment.borrow_mut().define(
                    name.lexeme.as_str(),
                    ExprResult::callable_ref(callable.clone()),
                );

                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_result = self.evaluate(condition)?;

                if condition_result.is_truthy() {
                    self.execute(then_branch)?
                } else if let Some(branch) = *else_branch.to_owned() {
                    self.execute(&branch)?
                }

                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);

                Ok(())
            }
            Stmt::Return { value } => {
                if let Some(expr) = *value.to_owned() {
                    let result = self.evaluate(&expr)?;
                    self.environment.borrow_mut().set_return_value(result);
                }

                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.borrow_mut().define(&name.lexeme, value);

                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }

                Ok(())
            }
        }
    }
}
