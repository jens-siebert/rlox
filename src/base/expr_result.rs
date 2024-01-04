use crate::base::scanner::TokenRef;
use crate::base::stmt::StmtRef;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum ExprResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Callable),
    None,
}

pub type ExprResultRef = Box<ExprResult>;

impl ExprResult {
    pub fn number(value: f64) -> Self {
        ExprResult::Number(value)
    }

    pub fn number_ref(value: f64) -> Box<Self> {
        Box::new(ExprResult::number(value))
    }

    pub fn string(value: String) -> Self {
        ExprResult::String(value)
    }

    pub fn string_ref(value: String) -> Box<Self> {
        Box::new(ExprResult::string(value))
    }

    pub fn boolean(value: bool) -> Self {
        ExprResult::Boolean(value)
    }

    pub fn boolean_ref(value: bool) -> Box<Self> {
        Box::new(ExprResult::boolean(value))
    }

    pub fn callable(value: Callable) -> Self {
        ExprResult::Callable(value)
    }

    pub fn callable_ref(value: Callable) -> Box<Self> {
        Box::new(ExprResult::callable(value))
    }

    pub fn none() -> Self {
        ExprResult::None
    }

    pub fn none_ref() -> Box<Self> {
        Box::new(ExprResult::none())
    }
}

impl Display for ExprResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match self {
            ExprResult::Number(value) => value.to_string(),
            ExprResult::String(value) => value.to_string(),
            ExprResult::Boolean(value) => value.to_string(),
            ExprResult::Callable(callable) => format!("<fn {}>", callable.name()),
            ExprResult::None => String::from("nil"),
        };

        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq)]
pub enum Callable {
    Function {
        name: TokenRef,
        params: Vec<TokenRef>,
        body: StmtRef,
    },
}

impl Callable {
    pub fn function(name: TokenRef, params: Vec<TokenRef>, body: StmtRef) -> Self {
        Callable::Function { name, params, body }
    }

    pub(crate) fn name(&self) -> String {
        match self {
            Callable::Function {
                name,
                params: _params,
                body: _body,
            } => name.lexeme.clone(),
        }
    }

    pub(crate) fn arity(&self) -> usize {
        match self {
            Callable::Function {
                name: _name,
                params,
                body: _body,
            } => params.len(),
        }
    }
}
