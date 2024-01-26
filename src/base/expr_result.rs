use crate::base::scanner::Token;
use crate::base::stmt::Stmt;
use crate::base::visitor::RuntimeError;
use crate::interpreter::environment::Environment;
use crate::interpreter::interpreter::Interpreter;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum ExprResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Function),
    #[default]
    None,
}

impl ExprResult {
    pub fn number(value: f64) -> Self {
        ExprResult::Number(value)
    }

    pub fn string(value: String) -> Self {
        ExprResult::String(value)
    }

    pub fn boolean(value: bool) -> Self {
        ExprResult::Boolean(value)
    }

    pub fn callable(value: Function) -> Self {
        ExprResult::Callable(value)
    }

    pub fn none() -> Self {
        ExprResult::None
    }

    pub fn is_truthy(&self) -> bool {
        match *self {
            ExprResult::Boolean(value) => value,
            ExprResult::None => false,
            _ => true,
        }
    }
}

impl Display for ExprResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match self {
            ExprResult::Number(value) => value.to_string(),
            ExprResult::String(value) => value.to_string(),
            ExprResult::Boolean(value) => value.to_string(),
            ExprResult::Callable(callable) => format!("<fn {}>", callable.name.lexeme),
            ExprResult::None => String::from("nil"),
        };

        write!(f, "{}", result)
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<ExprResult>,
    ) -> Result<ExprResult, RuntimeError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            closure,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<ExprResult>,
    ) -> Result<ExprResult, RuntimeError> {
        let scoped_interpreter =
            interpreter.fork(Environment::new_enclosing(Rc::clone(&self.closure)));

        for (i, token) in self.params.iter().enumerate() {
            if let Some(argument) = arguments.get(i) {
                scoped_interpreter.define(token.lexeme.as_str(), argument.clone());
            } else {
                return Err(RuntimeError::InvalidArgument);
            }
        }

        scoped_interpreter.execute_block(&self.body)
    }
}
