use crate::base::expr_result::ExprResult;
use crate::base::scanner::Token;
use crate::interpreter::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, ExprResult>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn define(&mut self, name: &str, value: ExprResult) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<ExprResult, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.to_owned());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::UndefinedVariable {
            line: name.line,
            name: name.lexeme.to_owned(),
        })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<ExprResult, RuntimeError> {
        self.get_at_helper(distance, 0, name)
    }

    fn get_at_helper(
        &self,
        distance: usize,
        index: usize,
        name: &Token,
    ) -> Result<ExprResult, RuntimeError> {
        if index < distance {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow().get_at_helper(distance, index + 1, name);
            }
        } else if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.to_owned());
        }

        Err(RuntimeError::UndefinedVariable {
            line: name.line,
            name: name.lexeme.to_owned(),
        })
    }

    pub fn assign(&mut self, name: &Token, value: &ExprResult) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.to_owned());

            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::UndefinedVariable {
            line: name.line,
            name: name.lexeme.to_owned(),
        })
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: &ExprResult,
    ) -> Result<(), RuntimeError> {
        self.assign_at_helper(distance, 0, name, value)
    }

    fn assign_at_helper(
        &mut self,
        distance: usize,
        index: usize,
        name: &Token,
        value: &ExprResult,
    ) -> Result<(), RuntimeError> {
        if index < distance {
            if let Some(enclosing) = &self.enclosing {
                return enclosing
                    .borrow_mut()
                    .assign_at_helper(distance, index + 1, name, value);
            }
        } else if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.to_owned());

            return Ok(());
        }

        Err(RuntimeError::UndefinedVariable {
            line: name.line,
            name: name.lexeme.to_owned(),
        })
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
