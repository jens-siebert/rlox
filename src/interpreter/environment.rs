use crate::base::expr_result::ExprResult;
use crate::base::visitor::RuntimeError;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Default, PartialEq)]
struct Scope {
    values: HashMap<String, ExprResult>,
    return_value: Box<ExprResult>,
}

impl Scope {
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

#[derive(Clone, Default, PartialEq)]
pub struct Environment {
    current_scope: Scope,
    parent_scopes: VecDeque<Scope>,
}

impl Environment {
    pub(crate) fn push_scope(&mut self) {
        let enclosing_scope = std::mem::take(&mut self.current_scope);
        self.parent_scopes.push_front(enclosing_scope);
    }

    pub(crate) fn pop_scope(&mut self) {
        let parent_scope = self.parent_scopes.pop_front().unwrap();
        self.current_scope = parent_scope;
    }

    pub(crate) fn define(&mut self, name: &str, value: ExprResult) {
        self.current_scope.define(name, value);
    }

    pub(crate) fn get(&self, name: &str) -> Result<ExprResult, RuntimeError> {
        if let Some(value) = self.current_scope.get(name) {
            return Ok(value);
        }

        for scope in self.parent_scopes.iter() {
            if let Some(value) = scope.get(name) {
                return Ok(value);
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn assign(&mut self, name: &str, value: &ExprResult) -> Result<(), RuntimeError> {
        if self.current_scope.assign(name, value).is_ok() {
            return Ok(());
        }

        for scope in self.parent_scopes.iter_mut() {
            if scope.assign(name, value).is_ok() {
                return Ok(());
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }
}
