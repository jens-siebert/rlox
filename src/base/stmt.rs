use crate::base::expr::ExprRef;
use crate::base::scanner::TokenRef;
use crate::base::visitor::{RuntimeError, Visitor};

#[derive(Clone, PartialEq)]
pub enum Stmt {
    Block {
        statements: Vec<StmtRef>,
    },
    Expression {
        expression: ExprRef,
    },
    Function {
        name: TokenRef,
        params: Vec<TokenRef>,
        body: Vec<StmtRef>,
    },
    If {
        condition: ExprRef,
        then_branch: StmtRef,
        else_branch: Option<StmtRef>,
    },
    Print {
        expression: ExprRef,
    },
    Return {
        value: Option<ExprRef>,
    },
    Var {
        name: TokenRef,
        initializer: ExprRef,
    },
    While {
        condition: ExprRef,
        body: StmtRef,
    },
}

pub type StmtRef = Box<Stmt>;

impl Stmt {
    pub fn block(statements: Vec<StmtRef>) -> Self {
        Stmt::Block { statements }
    }

    pub fn block_ref(statements: Vec<StmtRef>) -> Box<Self> {
        Box::new(Stmt::block(statements))
    }

    pub fn function(name: TokenRef, params: Vec<TokenRef>, body: Vec<StmtRef>) -> Self {
        Stmt::Function { name, params, body }
    }

    pub fn function_ref(name: TokenRef, params: Vec<TokenRef>, body: Vec<StmtRef>) -> Box<Self> {
        Box::new(Stmt::function(name, params, body))
    }

    pub fn expression(expression: ExprRef) -> Self {
        Stmt::Expression { expression }
    }

    pub fn expression_ref(expression: ExprRef) -> Box<Self> {
        Box::new(Stmt::expression(expression))
    }

    pub fn if_stmt(condition: ExprRef, then_branch: StmtRef, else_branch: Option<StmtRef>) -> Self {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn if_stmt_ref(
        condition: ExprRef,
        then_branch: StmtRef,
        else_branch: Option<StmtRef>,
    ) -> Box<Self> {
        Box::new(Stmt::if_stmt(condition, then_branch, else_branch))
    }

    pub fn print(expression: ExprRef) -> Self {
        Stmt::Print { expression }
    }

    pub fn print_ref(expression: ExprRef) -> Box<Self> {
        Box::new(Stmt::print(expression))
    }

    pub fn return_stmt(value: Option<ExprRef>) -> Self {
        Stmt::Return { value }
    }

    pub fn return_stmt_ref(value: Option<ExprRef>) -> Box<Self> {
        Box::new(Stmt::return_stmt(value))
    }

    pub fn var(name: TokenRef, initializer: ExprRef) -> Self {
        Stmt::Var { name, initializer }
    }

    pub fn var_ref(name: TokenRef, initializer: ExprRef) -> Box<Self> {
        Box::new(Stmt::var(name, initializer))
    }

    pub fn while_stmt(condition: ExprRef, body: StmtRef) -> Self {
        Stmt::While { condition, body }
    }

    pub fn while_stmt_ref(condition: ExprRef, body: StmtRef) -> Box<Self> {
        Box::new(Stmt::while_stmt(condition, body))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<Stmt, R>) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}
