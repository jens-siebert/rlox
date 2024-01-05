use crate::base::expr_result::ExprResultRef;
use crate::base::visitor::RuntimeError;
use std::collections::{HashMap, VecDeque};

type EnvironmentData = HashMap<String, ExprResultRef>;

pub struct Environment {
    store: VecDeque<EnvironmentData>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        let mut initial_store = VecDeque::new();
        initial_store.push_front(EnvironmentData::new());

        Environment {
            store: initial_store,
        }
    }

    pub(crate) fn push_scope(&mut self) {
        self.store.push_front(EnvironmentData::new())
    }

    pub(crate) fn pop_scope(&mut self) -> Option<EnvironmentData> {
        self.store.pop_front()
    }

    pub(crate) fn define(&mut self, name: &str, value: &ExprResultRef) {
        if let Some(environment) = self.store.front_mut() {
            environment.insert(name.to_string(), value.clone());
        }
    }

    pub(crate) fn get(&self, name: &String) -> Result<ExprResultRef, RuntimeError> {
        for environment in &self.store {
            if environment.contains_key(name) {
                if let Some(value) = environment.get(name) {
                    return Ok(value.clone());
                }
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }

    pub(crate) fn assign(
        &mut self,
        name: &String,
        value: &ExprResultRef,
    ) -> Result<(), RuntimeError> {
        for environment in &mut self.store {
            if environment.contains_key(name) {
                environment.insert(name.clone(), value.clone());
            }
        }

        Err(RuntimeError::UndefinedVariable)
    }
}
