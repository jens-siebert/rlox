use crate::base::scanner::Token;
use crate::base::stmt::Stmt;
use crate::interpreter::environment::Environment;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum ExprResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(LoxFunction),
    Class(LoxClass),
    Instance(LoxInstance),
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

    pub fn function(function: LoxFunction) -> Self {
        ExprResult::Function(function)
    }

    pub fn class(class: LoxClass) -> Self {
        ExprResult::Class(class)
    }

    pub fn instance(instance: LoxInstance) -> Self {
        ExprResult::Instance(instance)
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
            ExprResult::Function(function) => format!("<fn {}>", function.name.lexeme),
            ExprResult::Class(class) => class.name.lexeme.to_string(),
            ExprResult::Instance(instance) => format!("{} instance", instance.class.name.lexeme),
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
        arguments: &[ExprResult],
    ) -> Result<ExprResult, RuntimeError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
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

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &[ExprResult],
    ) -> Result<ExprResult, RuntimeError> {
        let scoped_interpreter =
            interpreter.fork(Environment::new_enclosing(Rc::clone(&self.closure)));

        for (i, token) in self.params.iter().enumerate() {
            if let Some(argument) = arguments.get(i) {
                scoped_interpreter.define(token, argument.clone());
            } else {
                return Err(RuntimeError::InvalidArgument { line: token.line });
            }
        }

        scoped_interpreter.execute_block(&self.body)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    name: Token,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: Token, methods: HashMap<String, LoxFunction>) -> Self {
        Self { name, methods }
    }

    pub fn find_method(&self, name: &Token) -> Option<&LoxFunction> {
        self.methods.get(&name.lexeme)
    }
}

impl Callable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: &[ExprResult],
    ) -> Result<ExprResult, RuntimeError> {
        Ok(ExprResult::instance(LoxInstance::new(self.to_owned())))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxInstance {
    class: LoxClass,
    fields: Rc<RefCell<HashMap<String, ExprResult>>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: &Token) -> Result<ExprResult, RuntimeError> {
        if let Some(value) = self.fields.borrow().get(&name.lexeme) {
            Ok(value.to_owned())
        } else if let Some(method) = self.class.find_method(name) {
            Ok(ExprResult::function(method.to_owned()))
        } else {
            Err(RuntimeError::UndefinedProperty { line: name.line })
        }
    }

    pub fn set(&self, name: &Token, value: ExprResult) {
        self.fields
            .borrow_mut()
            .insert(name.lexeme.to_owned(), value);
    }
}
