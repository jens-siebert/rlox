use crate::base::literal::LiteralValueRef;
use crate::base::visitor::RuntimeError;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<EnvironmentRef>,
    values: HashMap<String, LiteralValueRef>,
}

pub type EnvironmentRef = Box<Environment>;

impl Environment {
    fn new() -> Self {
        Environment::new_local_scope(None)
    }

    pub(crate) fn new_ref() -> EnvironmentRef {
        Box::new(Environment::new())
    }

    fn new_local_scope(enclosing: Option<EnvironmentRef>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub(crate) fn new_local_scope_ref(enclosing: Option<EnvironmentRef>) -> EnvironmentRef {
        Box::new(Environment::new_local_scope(enclosing))
    }

    pub(crate) fn define(&mut self, name: &str, value: &LiteralValueRef) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub(crate) fn get(&self, name: &String) -> Result<LiteralValueRef, RuntimeError> {
        match self.values.get(name) {
            None => match &self.enclosing {
                None => Err(RuntimeError::UndefinedVariable),
                Some(scope) => scope.get(name),
            },
            Some(value) => Ok(value.clone()),
        }
    }

    pub(crate) fn assign(
        &mut self,
        name: &String,
        value: &LiteralValueRef,
    ) -> Result<(), RuntimeError> {
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
