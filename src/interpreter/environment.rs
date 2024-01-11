use crate::base::expr_result::ExprResultRef;
use crate::base::visitor::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, PartialEq)]
struct Scope {
    values: HashMap<String, Rc<RefCell<ExprResultRef>>>,
    return_value: ExprResultRef,
}

impl Scope {
    fn define(&mut self, name: &str, value: ExprResultRef) {
        self.values
            .insert(name.to_string(), Rc::new(RefCell::new(value)));
    }

    fn get(&self, name: &str) -> Option<ExprResultRef> {
        self.values.get(name).map(|v| v.borrow().clone())
    }

    fn assign(&self, name: &str, value: &ExprResultRef) -> Result<(), RuntimeError> {
        match self.values.get(name) {
            None => Err(RuntimeError::UndefinedVariable),
            Some(slot) => {
                *slot.borrow_mut() = value.to_owned();
                Ok(())
            }
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub(crate) struct Environment {
    current_scope: Scope,
    parent_scopes: Vec<Scope>,
}

impl Environment {
    pub(crate) fn push_scope(&mut self) {
        let enclosing_scope = std::mem::take(&mut self.current_scope);
        self.parent_scopes.push(enclosing_scope);
    }

    pub(crate) fn pop_scope(&mut self) {
        let parent_scope = self.parent_scopes.pop().unwrap();
        self.current_scope = parent_scope;
    }

    pub(crate) fn define(&mut self, name: &str, value: ExprResultRef) {
        self.current_scope.define(name, value);
    }

    pub(crate) fn get(&self, name: &str) -> Result<ExprResultRef, RuntimeError> {
        if let Some(value) = self.current_scope.get(name) {
            return Ok(value);
        }

        for scope in self.parent_scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value);
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn assign(&mut self, name: &str, value: &ExprResultRef) -> Result<(), RuntimeError> {
        if self.current_scope.assign(name, value).is_ok() {
            return Ok(());
        }
        for scope in self.parent_scopes.iter_mut().rev() {
            if scope.assign(name, value).is_ok() {
                return Ok(());
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn get_return_value(&self) -> ExprResultRef {
        self.current_scope.return_value.clone()
    }

    pub(crate) fn set_return_value(&mut self, value: ExprResultRef) {
        self.current_scope.return_value = value
    }
}
