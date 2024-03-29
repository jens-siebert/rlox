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
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            name,
            params,
            body,
            closure,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: &LoxInstance) -> ExprResult {
        let environment = Environment::new_enclosing(Rc::clone(&self.closure));

        environment
            .borrow_mut()
            .define("this", ExprResult::instance(instance.to_owned()));

        ExprResult::function(LoxFunction::new(
            self.name.to_owned(),
            self.params.to_owned(),
            self.body.to_owned(),
            environment,
            self.is_initializer,
        ))
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

        if let Err(e) = scoped_interpreter.execute_block(&self.body) {
            return match e {
                RuntimeError::Return { ret_val } => {
                    if self.is_initializer {
                        Ok(self.closure.borrow().get_at(0, "this").unwrap())
                    } else {
                        Ok(ret_val)
                    }
                }
                _ => Err(e),
            };
        }

        Ok(ExprResult::None)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    name: Token,
    superclass: Box<Option<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: Token,
        superclass: Option<LoxClass>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        Self {
            name,
            superclass: Box::new(superclass),
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction> {
        if self.methods.contains_key(name) {
            self.methods.get(&name.to_string())
        } else if let Some(sc) = self.superclass.as_ref() {
            sc.find_method(name)
        } else {
            None
        }
    }
}

impl Callable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &[ExprResult],
    ) -> Result<ExprResult, RuntimeError> {
        let instance = LoxInstance::new(self.to_owned());

        if let Some(initializer) = self.find_method("init") {
            if let ExprResult::Function(function) = initializer.bind(&instance) {
                function.call(interpreter, arguments)?;
            }
        }

        Ok(ExprResult::instance(instance))
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
        } else if let Some(method) = self.class.find_method(&name.lexeme) {
            Ok(method.bind(self))
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
