use crate::base::expr::{Expr, ExprRef};
use crate::base::literal::{LiteralValue, LiteralValueRef};
use crate::base::scanner::TokenType;
use crate::base::stmt::{Stmt, StmtRef};
use crate::base::visitor::{RuntimeError, Visitor};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
struct Environment {
    enclosing: Option<EnvironmentRef>,
    values: HashMap<String, LiteralValueRef>,
}

type EnvironmentRef = Box<Environment>;

impl Environment {
    fn new() -> Self {
        Environment::new_local_scope(None)
    }

    fn new_ref() -> EnvironmentRef {
        Box::new(Environment::new())
    }

    fn new_local_scope(enclosing: Option<EnvironmentRef>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    fn new_local_scope_ref(enclosing: Option<EnvironmentRef>) -> EnvironmentRef {
        Box::new(Environment::new_local_scope(enclosing))
    }

    fn define(&mut self, name: &str, value: &LiteralValueRef) {
        self.values.insert(name.to_string(), value.clone());
    }

    fn get(&self, name: &String) -> Result<LiteralValueRef, RuntimeError> {
        match self.values.get(name) {
            None => match &self.enclosing {
                None => Err(RuntimeError::UndefinedVariable),
                Some(scope) => scope.get(name),
            },
            Some(value) => Ok(value.clone()),
        }
    }

    fn assign(&mut self, name: &String, value: &LiteralValueRef) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.clone(), value.clone());
            Ok(())
        } else {
            match &mut self.enclosing {
                None => Err(RuntimeError::UndefinedVariable),
                Some(scope) => scope.assign(name, value),
            }
        }
    }
}

pub struct Interpreter {
    environment: RefCell<EnvironmentRef>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Environment::new_ref()),
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
        let previous = self.environment.replace(environment);

        for statement in statements {
            self.execute(statement)?
        }

        self.environment.replace(previous);

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
                self.execute_block(
                    statements,
                    Environment::new_local_scope_ref(Some(old_scope)),
                )
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
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
        }
    }
}
