use crate::base::expr::{Expr, ExprRef, LiteralValue};
use crate::base::expr_result::Callable;
use crate::base::expr_result::{ExprResult, ExprResultRef};
use crate::base::scanner::TokenType;
use crate::base::stmt::{Stmt, StmtRef};
use crate::base::visitor::{RuntimeError, Visitor};
use crate::interpreter::environment::Environment;
use std::cell::RefCell;

pub struct Interpreter {
    environment: RefCell<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Environment::new()),
        }
    }

    fn is_truthy(&self, literal_value: &ExprResultRef) -> bool {
        match **literal_value {
            ExprResult::Boolean(value) => value,
            ExprResult::None => false,
            _ => true,
        }
    }

    fn evaluate(&self, expr: &ExprRef) -> Result<ExprResultRef, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &StmtRef) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    pub(crate) fn execute_block(&self, statements: &Vec<StmtRef>) -> Result<(), RuntimeError> {
        self.environment.borrow_mut().push_scope();

        for statement in statements {
            self.execute(statement)?
        }

        self.environment.borrow_mut().pop_scope();

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

                    callable.call(self, args)?;
                }

                Ok(ExprResult::none_ref())
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
                        ExprResult::Number(value) => Ok(ExprResult::number_ref(-value)),
                        _ => Err(RuntimeError::NumberExpected),
                    },
                    TokenType::Bang => Ok(ExprResult::boolean_ref(!self.is_truthy(&right))),
                    _ => Err(RuntimeError::InvalidValue),
                }
            }
            Expr::Variable { name } => {
                if let Ok(value) = self.environment.borrow().get(&name.lexeme) {
                    Ok(value)
                } else {
                    Err(RuntimeError::UndefinedVariable)
                }
            }
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
            Stmt::Block { statements } => self.execute_block(statements),
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let callable = Callable::function(name.clone(), params.clone(), body.clone());
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.as_str(), &ExprResult::callable_ref(callable));

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

impl Callable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<ExprResultRef>,
    ) -> Result<(), RuntimeError> {
        interpreter.environment.borrow_mut().push_scope();

        match self {
            Callable::Function {
                name: _name,
                params,
                body,
            } => {
                for (i, token) in params.iter().enumerate() {
                    if let Some(argument) = arguments.get(i) {
                        interpreter
                            .environment
                            .borrow_mut()
                            .define(token.lexeme.as_str(), argument);
                    } else {
                        return Err(RuntimeError::InvalidArgument);
                    }
                }

                if let Stmt::Block { statements } = *body.clone() {
                    interpreter.execute_block(&statements)
                } else {
                    Err(RuntimeError::BlockExpected)
                }
            }
        }
    }
}
