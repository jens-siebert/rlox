use crate::base::expr::ExprRef;
use crate::base::scanner::Token;
use crate::base::visitor::{RuntimeError, Visitor};

pub enum Stmt<'a> {
    Block {
        statements: Vec<StmtRef<'a>>,
    },
    Expression {
        expression: ExprRef<'a>,
    },
    Print {
        expression: ExprRef<'a>,
    },
    Var {
        name: &'a Token,
        initializer: ExprRef<'a>,
    },
}

pub type StmtRef<'a> = Box<Stmt<'a>>;

impl<'a> Stmt<'a> {
    pub fn block(statements: Vec<StmtRef>) -> Stmt {
        Stmt::Block { statements }
    }

    pub fn block_ref(statements: Vec<StmtRef>) -> StmtRef {
        Box::new(Stmt::block(statements))
    }

    pub fn expression(expression: ExprRef) -> Stmt {
        Stmt::Expression { expression }
    }

    pub fn expression_ref(expression: ExprRef) -> StmtRef {
        Box::new(Stmt::expression(expression))
    }

    pub fn print(expression: ExprRef) -> Stmt {
        Stmt::Print { expression }
    }

    pub fn print_ref(expression: ExprRef) -> StmtRef {
        Box::new(Stmt::print(expression))
    }

    pub fn var(name: &'a Token, initializer: ExprRef<'a>) -> Stmt<'a> {
        Stmt::Var { name, initializer }
    }

    pub fn var_ref(name: &'a Token, initializer: ExprRef<'a>) -> StmtRef<'a> {
        Box::new(Stmt::var(name, initializer))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<Stmt<'a>, R>) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}