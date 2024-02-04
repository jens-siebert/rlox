use crate::base::expr::{Expr, LiteralValue};
use crate::base::scanner::{Token, TokenType};
use crate::base::stmt::Stmt;
use std::cell::RefCell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Error while reading token.")]
    TokenReadError,
    #[error("{line:?}: Unknown token detected.")]
    MissingExpression { line: usize },
    #[error("{line:?}: Expect '(' after function name.")]
    MissingLeftParenthesisAfterFunctionName { line: usize },
    #[error("{line:?}: Expect '(' after 'if' statement.")]
    MissingLeftParenthesisAfterIfStatement { line: usize },
    #[error("{line:?}: Expect '(' after 'while' statement.")]
    MissingLeftParenthesisAfterWhileStatement { line: usize },
    #[error("{line:?}: Expect '(' after 'for' statement.")]
    MissingLeftParenthesisAfterForStatement { line: usize },
    #[error("{line:?}: Expect '{{' before function body.")]
    MissingLeftBraceBeforeFunctionBody { line: usize },
    #[error("{line:?}: Expect '{{' before class body.")]
    MissingLeftBraceBeforeClassBody { line: usize },
    #[error("{line:?}: Expect ')' after expression.")]
    MissingRightParenthesisAfterExpression { line: usize },
    #[error("{line:?}: Expect ')' after condition.")]
    MissingRightParenthesisAfterCondition { line: usize },
    #[error("{line:?}: Expect ')' after 'for' statement.")]
    MissingRightParenthesisAfterForStatement { line: usize },
    #[error("{line:?}: Expect ')' after parameters.")]
    MissingRightParenthesisAfterParameters { line: usize },
    #[error("{line:?}: Expect ')' after arguments.")]
    MissingRightParenthesisAfterArguments { line: usize },
    #[error("{line:?}: Expect '}}' after block.")]
    MissingRightBraceAfterBlock { line: usize },
    #[error("{line:?}: Expect '}}' after class body.")]
    MissingRightBraceAfterClassBody { line: usize },
    #[error("{line:?}: Expect ';' after value.")]
    MissingSemicolonAfterValue { line: usize },
    #[error("{line:?}: Expect ';' after expression.")]
    MissingSemicolonAfterExpression { line: usize },
    #[error("{line:?}: Expect ';' after variable declaration.")]
    MissingSemicolonAfterVariableDeclaration { line: usize },
    #[error("{line:?}: Expect ';' after loop condition.")]
    MissingSemicolonAfterLoopCondition { line: usize },
    #[error("{line:?}: Expect variable name.")]
    MissingVariableName { line: usize },
    #[error("{line:?}: Expect function name.")]
    MissingFunctionName { line: usize },
    #[error("{line:?}: Expect class name.")]
    MissingClassName { line: usize },
    #[error("{line:?}: Expect function name.")]
    MissingParameterName { line: usize },
    #[error("{line:?}: Invalid assignment target.")]
    InvalidAssignmentTarget { line: usize },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::new(0),
        }
    }
    pub fn parse(&self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = vec![];

        while !self.is_at_end()? {
            statements.push(self.declaration()?)
        }

        Ok(statements)
    }

    fn declaration(&self) -> Result<Stmt, ParserError> {
        if self.match_token_types(&[TokenType::Class])? {
            self.class_declaration()
        } else if self.match_token_types(&[TokenType::Fun])? {
            self.function()
        } else if self.match_token_types(&[TokenType::Var])? {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&self) -> Result<Stmt, ParserError> {
        let name = self.consume(
            TokenType::Identifier,
            ParserError::MissingClassName {
                line: self.peek().unwrap().line,
            },
        )?;

        self.consume(
            TokenType::LeftBrace,
            ParserError::MissingLeftBraceBeforeClassBody {
                line: self.peek().unwrap().line,
            },
        )?;

        let mut methods = vec![];
        while !self.check(TokenType::RightBrace)? && !self.is_at_end()? {
            methods.push(self.function()?);
        }

        self.consume(
            TokenType::RightBrace,
            ParserError::MissingRightBraceAfterClassBody {
                line: self.peek().unwrap().line,
            },
        )?;

        Ok(Stmt::class(name, methods))
    }

    fn function(&self) -> Result<Stmt, ParserError> {
        let name = self.consume(
            TokenType::Identifier,
            ParserError::MissingFunctionName {
                line: self.peek().unwrap().line,
            },
        )?;
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterFunctionName {
                line: self.peek().unwrap().line,
            },
        )?;

        let mut parameters = vec![];

        if !self.check(TokenType::RightParen)? {
            loop {
                let parameter = self.consume(
                    TokenType::Identifier,
                    ParserError::MissingParameterName {
                        line: self.peek().unwrap().line,
                    },
                )?;

                parameters.push(parameter);

                if !self.match_token_types(&[TokenType::Comma])? {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterParameters {
                line: self.peek().unwrap().line,
            },
        )?;
        self.consume(
            TokenType::LeftBrace,
            ParserError::MissingLeftBraceBeforeFunctionBody {
                line: self.peek().unwrap().line,
            },
        )?;

        let body = self.block()?;

        Ok(Stmt::function(name, parameters, body))
    }

    fn variable_declaration(&self) -> Result<Stmt, ParserError> {
        let name = self.consume(
            TokenType::Identifier,
            ParserError::MissingVariableName {
                line: self.peek().unwrap().line,
            },
        )?;
        let initializer = if self.match_token_types(&[TokenType::Equal])? {
            self.expression()?
        } else {
            Expr::literal(LiteralValue::None)
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterVariableDeclaration {
                line: self.peek().unwrap().line,
            },
        )?;

        Ok(Stmt::var(name, initializer))
    }

    fn statement(&self) -> Result<Stmt, ParserError> {
        if self.match_token_types(&[TokenType::For])? {
            self.for_statement()
        } else if self.match_token_types(&[TokenType::If])? {
            self.if_statement()
        } else if self.match_token_types(&[TokenType::Print])? {
            self.print_statement()
        } else if self.match_token_types(&[TokenType::Return])? {
            self.return_statement()
        } else if self.match_token_types(&[TokenType::While])? {
            self.while_statement()
        } else if self.match_token_types(&[TokenType::LeftBrace])? {
            let block = self.block()?;
            Ok(Stmt::block(block))
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterForStatement {
                line: self.peek().unwrap().line,
            },
        )?;

        let initializer = if self.match_token_types(&[TokenType::Semicolon])? {
            None
        } else if self.match_token_types(&[TokenType::Var])? {
            Some(self.variable_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon)? {
            self.expression()?
        } else {
            Expr::literal(LiteralValue::Boolean(true))
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterLoopCondition {
                line: self.peek().unwrap().line,
            },
        )?;

        let increment = if !self.check(TokenType::RightParen)? {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterForStatement {
                line: self.peek().unwrap().line,
            },
        )?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::block(vec![body, Stmt::expression(inc)])
        }

        body = Stmt::while_stmt(condition, body);

        if let Some(init) = initializer {
            body = Stmt::block(vec![init, body])
        }

        Ok(body)
    }

    fn if_statement(&self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterIfStatement {
                line: self.peek().unwrap().line,
            },
        )?;

        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterCondition {
                line: self.peek().unwrap().line,
            },
        )?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token_types(&[TokenType::Else])? {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::if_stmt(condition, then_branch, else_branch))
    }

    fn print_statement(&self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterValue {
                line: self.peek().unwrap().line,
            },
        )?;
        Ok(Stmt::print(value))
    }

    fn return_statement(&self) -> Result<Stmt, ParserError> {
        let keyword = self.previous()?;
        let expr = if !self.check(TokenType::Semicolon)? {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterExpression {
                line: self.peek().unwrap().line,
            },
        )?;

        Ok(Stmt::return_stmt(keyword, expr))
    }

    fn while_statement(&self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterWhileStatement {
                line: self.peek().unwrap().line,
            },
        )?;

        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterCondition {
                line: self.peek().unwrap().line,
            },
        )?;

        let body = self.statement()?;

        Ok(Stmt::while_stmt(condition, body))
    }

    fn block(&self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace)? && !self.is_at_end()? {
            statements.push(self.declaration()?)
        }

        self.consume(
            TokenType::RightBrace,
            ParserError::MissingRightBraceAfterBlock {
                line: self.peek().unwrap().line,
            },
        )?;

        Ok(statements)
    }

    fn expression_statement(&self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterExpression {
                line: self.peek().unwrap().line,
            },
        )?;
        Ok(Stmt::expression(value))
    }

    fn expression(&self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, ParserError> {
        let expr = self.or()?;

        if self.match_token_types(&[TokenType::Equal])? {
            let value = self.assignment()?;

            return match expr {
                Expr::Variable { uuid: _uuid, name } => Ok(Expr::assign(*name, value)),
                _ => Err(ParserError::InvalidAssignmentTarget {
                    line: self.peek().unwrap().line,
                }),
            };
        }

        Ok(expr)
    }

    fn or(&self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.match_token_types(&[TokenType::Or])? {
            let operator = self.previous()?;
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn and(&self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token_types(&[TokenType::And])? {
            let operator = self.previous()?;
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn equality(&self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[TokenType::Slash, TokenType::Star])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, ParserError> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.call()
    }

    fn call(&self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token_types(&[TokenType::LeftParen])? {
                let mut arguments = vec![];
                if !self.check(TokenType::RightParen)? {
                    loop {
                        arguments.push(self.expression()?);

                        if !self.match_token_types(&[TokenType::Comma])? {
                            break;
                        }
                    }
                }

                let paren = self.consume(
                    TokenType::RightParen,
                    ParserError::MissingRightParenthesisAfterArguments {
                        line: self.peek().unwrap().line,
                    },
                )?;

                expr = Expr::call(paren, expr, arguments);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&self) -> Result<Expr, ParserError> {
        if self.match_token_types(&[TokenType::False])? {
            return Ok(Expr::literal(LiteralValue::Boolean(false)));
        }
        if self.match_token_types(&[TokenType::True])? {
            return Ok(Expr::literal(LiteralValue::Boolean(true)));
        }
        if self.match_token_types(&[TokenType::Nil])? {
            return Ok(Expr::literal(LiteralValue::None));
        }

        match &self.peek()?.token_type {
            TokenType::Number { value } => {
                self.advance()?;
                return Ok(Expr::literal(LiteralValue::Number(value.to_owned())));
            }
            TokenType::String { value } => {
                self.advance()?;
                return Ok(Expr::literal(LiteralValue::String(value.clone())));
            }
            _ => {}
        }

        if self.match_token_types(&[TokenType::Identifier])? {
            return Ok(Expr::variable(self.previous()?));
        }

        if self.match_token_types(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                ParserError::MissingRightParenthesisAfterExpression {
                    line: self.peek().unwrap().line,
                },
            )?;
            return Ok(Expr::grouping(expr));
        }

        Err(ParserError::MissingExpression {
            line: self.peek().unwrap().line,
        })
    }

    fn peek(&self) -> Result<Token, ParserError> {
        match self.tokens.get(*self.current.borrow()) {
            None => Err(ParserError::TokenReadError),
            Some(token) => Ok((*token).clone()),
        }
    }

    fn previous(&self) -> Result<Token, ParserError> {
        match self.tokens.get(*self.current.borrow() - 1) {
            None => Err(ParserError::TokenReadError),
            Some(token) => Ok((*token).clone()),
        }
    }

    fn advance(&self) -> Result<Token, ParserError> {
        if !self.is_at_end()? {
            *self.current.borrow_mut() += 1
        }
        self.previous()
    }

    fn consume(&self, token_type: TokenType, error: ParserError) -> Result<Token, ParserError> {
        if self.check(token_type)? {
            self.advance()
        } else {
            Err(error)
        }
    }

    fn is_at_end(&self) -> Result<bool, ParserError> {
        match self.peek() {
            Ok(token) => Ok(token.token_type == TokenType::Eof),
            Err(err) => Err(err),
        }
    }

    fn check(&self, token_type: TokenType) -> Result<bool, ParserError> {
        if self.is_at_end()? {
            Ok(false)
        } else {
            match self.peek() {
                Ok(token) => Ok(token.token_type == token_type),
                Err(err) => Err(err),
            }
        }
    }

    fn match_token_types(&self, token_types: &[TokenType]) -> Result<bool, ParserError> {
        for token_type in token_types {
            if self.check(token_type.clone())? {
                self.advance()?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
