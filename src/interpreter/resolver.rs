use crate::base::expr::{Expr, LiteralValue};
use crate::base::scanner::Token;
use crate::base::stmt::Stmt;
use crate::base::visitor::{RuntimeError, Visitor};
use crate::interpreter::interpreter::Interpreter;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: RefCell<VecDeque<HashMap<String, bool>>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(VecDeque::new()),
        }
    }

    fn resolve_stmts(&self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.resolve_stmt(statement)?
        }

        Ok(())
    }

    fn resolve_stmt(&self, statement: Stmt) -> Result<(), RuntimeError> {
        statement.accept(self)
    }

    fn resolve_expr(&self, expression: Expr) -> Result<(), RuntimeError> {
        expression.accept(self)
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push_front(HashMap::new())
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop_front();
    }

    fn declare(&self, name: &Token) {
        if let Some(scope) = self.scopes.borrow_mut().front_mut() {
            scope.insert(name.lexeme.to_owned(), false);
        }
    }

    fn define(&self, name: &Token) {
        if let Some(scope) = self.scopes.borrow_mut().front_mut() {
            scope.insert(name.lexeme.to_owned(), true);
        }
    }
}

impl Visitor<Stmt, ()> for Resolver {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements.to_owned())?;
                self.end_scope()
            }
            Stmt::Expression { .. } => {}
            Stmt::Function { .. } => {}
            Stmt::If { .. } => {}
            Stmt::Print { .. } => {}
            Stmt::Return { .. } => {}
            Stmt::Var { name, initializer } => {
                self.declare(name);

                if let Expr::Literal { value } = *initializer.to_owned() {
                    if value != LiteralValue::None {
                        self.resolve_expr(*initializer.to_owned())?;
                    }
                }

                self.define(name);
            }
            Stmt::While { .. } => {}
        }

        Ok(())
    }
}

impl Visitor<Expr, ()> for Resolver {
    fn visit(&self, input: &Expr) -> Result<(), RuntimeError> {
        match input {
            Expr::Binary { .. } => {}
            Expr::Call { .. } => {}
            Expr::Grouping { .. } => {}
            Expr::Literal { .. } => {}
            Expr::Logical { .. } => {}
            Expr::Unary { .. } => {}
            Expr::Variable { .. } => {}
            Expr::Assign { .. } => {}
        }

        Ok(())
    }
}
