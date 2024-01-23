use crate::base::expr_result::ExprResult;
use crate::base::visitor::RuntimeError;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Default, PartialEq)]
struct Scope {
    values: HashMap<String, ExprResult>,
}

impl Scope {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn define(&mut self, name: &str, value: ExprResult) {
        self.values.insert(name.to_string(), value);
    }

    fn get(&self, name: &str) -> Option<ExprResult> {
        self.values.get(name).cloned()
    }

    fn assign(&mut self, name: &str, value: &ExprResult) -> Result<(), RuntimeError> {
        match self.values.get(name) {
            None => Err(RuntimeError::UndefinedVariable),
            Some(_) => {
                self.values.insert(name.to_string(), value.to_owned());
                Ok(())
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    scopes: VecDeque<Scope>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        let mut scopes = VecDeque::new();
        scopes.push_front(Scope::new());

        Self { scopes }
    }

    pub(crate) fn push_scope(&mut self) {
        let enclosing_scope = Scope::default();
        self.scopes.push_front(enclosing_scope);
    }

    pub(crate) fn pop_scope(&mut self) {
        self.scopes.pop_front();
    }

    pub(crate) fn define(&mut self, name: &str, value: ExprResult) {
        if let Some(scope) = self.scopes.front_mut() {
            scope.define(name, value);
        }
    }

    pub(crate) fn get(&self, name: &str) -> Result<ExprResult, RuntimeError> {
        for scope in self.scopes.iter() {
            if let Some(value) = scope.get(name) {
                return Ok(value);
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn assign(&mut self, name: &str, value: &ExprResult) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut() {
            if scope.assign(name, value).is_ok() {
                return Ok(());
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
