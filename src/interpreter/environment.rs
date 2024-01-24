use crate::base::expr_result::ExprResult;
use crate::base::visitor::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, ExprResult>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub(crate) fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub(crate) fn define(&mut self, name: &str, value: ExprResult) {
        self.values.insert(name.to_string(), value);
    }

    pub(crate) fn get(&self, name: &str) -> Result<ExprResult, RuntimeError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.to_owned());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn assign(&mut self, name: &str, value: &ExprResult) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.to_owned());

            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::UndefinedVariable)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
