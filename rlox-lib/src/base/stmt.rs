use crate::base::expr::Expr;
use crate::base::scanner::Token;
use crate::base::visitor::Visitor;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Box<Token>,
        superclass: Box<Option<Expr>>,
        methods: Vec<Stmt>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Function {
        name: Box<Token>,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Box<Token>,
        value: Box<Option<Expr>>,
    },
    Var {
        name: Box<Token>,
        initializer: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block { statements }
    }

    pub fn class(name: Token, superclass: Option<Expr>, methods: Vec<Stmt>) -> Self {
        Stmt::Class {
            name: Box::new(name),
            superclass: Box::new(superclass),
            methods,
        }
    }

    pub fn function(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Stmt::Function {
            name: Box::new(name),
            params,
            body,
        }
    }

    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression {
            expression: Box::new(expression),
        }
    }

    pub fn if_stmt(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Stmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print {
            expression: Box::new(expression),
        }
    }

    pub fn return_stmt(keyword: Token, value: Option<Expr>) -> Self {
        Stmt::Return {
            keyword: Box::new(keyword),
            value: Box::new(value),
        }
    }

    pub fn var(name: Token, initializer: Expr) -> Self {
        Stmt::Var {
            name: Box::new(name),
            initializer: Box::new(initializer),
        }
    }

    pub fn while_stmt(condition: Expr, body: Stmt) -> Self {
        Stmt::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    pub fn accept<R, E>(&self, visitor: &dyn Visitor<Stmt, R, E>) -> Result<R, E> {
        visitor.visit(self)
    }
}
