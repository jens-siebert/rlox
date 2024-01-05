use crate::base::expr_result::{ExprResult, ExprResultRef};
use crate::base::visitor::RuntimeError;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<EnvironmentRef>,
    values: HashMap<String, ExprResultRef>,
    return_value: ExprResultRef,
}

pub type EnvironmentRef = Box<Environment>;

impl Environment {
    fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
            return_value: ExprResult::none_ref(),
        }
    }

    pub(crate) fn new_ref() -> Box<Self> {
        Box::new(Environment::new())
    }

    fn new_scope(enclosing: EnvironmentRef) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
            return_value: ExprResult::none_ref(),
        }
    }

    pub(crate) fn new_scope_ref(enclosing: EnvironmentRef) -> EnvironmentRef {
        Box::new(Environment::new_scope(enclosing))
    }

    pub(crate) fn enclosing(&self) -> Option<EnvironmentRef> {
        self.enclosing.clone()
    }

    pub(crate) fn return_value(&self) -> ExprResultRef {
        self.return_value.clone()
    }

    pub(crate) fn define(&mut self, name: &str, value: &ExprResultRef) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub(crate) fn get(&self, name: &String) -> Result<ExprResultRef, RuntimeError> {
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
        value: &ExprResultRef,
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
