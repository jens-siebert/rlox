use crate::base::expr::{Expr, LiteralValue};
use crate::base::expr_result::ExprResult;
use crate::base::expr_result::{Callable, Function};
use crate::base::scanner::TokenType;
use crate::base::stmt::Stmt;
use crate::base::visitor::{RuntimeError, Visitor};
use crate::interpreter::environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let env = Rc::clone(&globals);
        Self {
            globals,
            environment: env,
        }
    }

    pub fn fork(&self, environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            globals: Rc::clone(&self.globals),
            environment,
        }
    }

    pub fn interpret(&self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute_block(&self, statements: &Vec<Stmt>) -> Result<ExprResult, RuntimeError> {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                return match e {
                    RuntimeError::Return { ret_val } => Ok(ret_val),
                    _ => Err(e),
                };
            }
        }

        Ok(ExprResult::none())
    }

    pub fn define(&self, name: &str, value: ExprResult) {
        self.environment.borrow_mut().define(name, value);
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&self, expr: &Expr) -> Result<ExprResult, RuntimeError> {
        expr.accept(self)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}

impl Visitor<Expr, ExprResult> for Interpreter {
    fn visit(&self, input: &Expr) -> Result<ExprResult, RuntimeError> {
        match input {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Greater => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 > v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 >= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Less => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 < v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 <= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::BangEqual => Ok(ExprResult::boolean(left != right)),
                    TokenType::EqualEqual => Ok(ExprResult::boolean(left == right)),
                    TokenType::Minus => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 - v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Slash => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 / v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Star => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 * v2))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Plus => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 + v2))
                        }
                        (ExprResult::String(v1), ExprResult::String(v2)) => {
                            Ok(ExprResult::string(v1.clone() + v2.clone().as_str()))
                        }
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
            Expr::Call { callee, arguments } => {
                let call = self.evaluate(callee)?;

                if let ExprResult::Callable(callable) = call {
                    if arguments.len() != callable.arity() {
                        return Err(RuntimeError::NonMatchingNumberOfArguments);
                    }

                    let mut args = vec![];
                    for argument in arguments {
                        args.push(self.evaluate(argument)?);
                    }

                    callable.call(self, &args)
                } else {
                    Err(RuntimeError::UndefinedCallable)
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => match value {
                LiteralValue::Number(value) => Ok(ExprResult::number(value.into_inner())),
                LiteralValue::String(value) => Ok(ExprResult::string(value.clone())),
                LiteralValue::Boolean(value) => Ok(ExprResult::boolean(*value)),
                LiteralValue::None => Ok(ExprResult::none()),
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
                    TokenType::Minus => match right {
                        ExprResult::Number(value) => Ok(ExprResult::number(-value)),
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Bang => Ok(ExprResult::boolean(!right.is_truthy())),
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
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
            }
            Stmt::Function { name, params, body } => {
                let callable = Function::new(
                    *name.to_owned(),
                    params.to_owned(),
                    body.to_owned(),
                    Environment::new_enclosing(Rc::clone(&self.environment)),
                );

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.as_str(), ExprResult::callable(callable.clone()));
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
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
            }
            Stmt::Return { value } => {
                if let Some(expr) = *value.to_owned() {
                    let ret_val = self.evaluate(&expr)?;
                    return Err(RuntimeError::Return { ret_val });
                }
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.borrow_mut().define(&name.lexeme, value);
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
        }

        Ok(())
    }
}
