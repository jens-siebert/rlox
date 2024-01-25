use crate::base::expr::{Expr, LiteralValue};
use crate::base::scanner::{Token, TokenType};
use crate::base::stmt::Stmt;
use std::cell::RefCell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Error while reading token.")]
    TokenReadError,
    #[error("Unknown token detected.")]
    MissingExpression,
    #[error("Expect '(' after function name.")]
    MissingLeftParenthesisAfterFunctionName,
    #[error("Expect '(' after 'if' statement.")]
    MissingLeftParenthesisAfterIfStatement,
    #[error("Expect '(' after 'while' statement.")]
    MissingLeftParenthesisAfterWhileStatement,
    #[error("Expect '(' after 'for' statement.")]
    MissingLeftParenthesisAfterForStatement,
    #[error("Expect '{{' before function body.")]
    MissingLeftBraceBeforeFunctionBody,
    #[error("Expect ')' after expression.")]
    MissingRightParenthesisAfterExpression,
    #[error("Expect ')' after condition.")]
    MissingRightParenthesisAfterCondition,
    #[error("Expect ')' after 'for' statement.")]
    MissingRightParenthesisAfterForStatement,
    #[error("Expect ')' after parameters.")]
    MissingRightParenthesisAfterParameters,
    #[error("Expect ')' after arguments.")]
    MissingRightParenthesisAfterArguments,
    #[error("Expect '}}' after block.")]
    MissingRightBraceAfterBlock,
    #[error("Expect ';' after value.")]
    MissingSemicolonAfterValue,
    #[error("Expect ';' after expression.")]
    MissingSemicolonAfterExpression,
    #[error("Expect ';' after variable declaration.")]
    MissingSemicolonAfterVariableDeclaration,
    #[error("Expect ';' after loop condition.")]
    MissingSemicolonAfterLoopCondition,
    #[error("Expect variable name.")]
    MissingVariableName,
    #[error("Expect function name.")]
    MissingFunctionName,
    #[error("Expect function name.")]
    MissingParameterName,
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,
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
        if self.match_token_types(&[TokenType::Fun])? {
            self.function()
        } else if self.match_token_types(&[TokenType::Var])? {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn function(&self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, ParserError::MissingFunctionName)?;
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterFunctionName,
        )?;

        let mut parameters = vec![];

        if !self.check(TokenType::RightParen)? {
            loop {
                let parameter =
                    self.consume(TokenType::Identifier, ParserError::MissingParameterName)?;

                parameters.push(parameter);

                if !self.match_token_types(&[TokenType::Comma])? {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterParameters,
        )?;
        self.consume(
            TokenType::LeftBrace,
            ParserError::MissingLeftBraceBeforeFunctionBody,
        )?;

        let body = self.block()?;

        Ok(Stmt::function(name, parameters, body))
    }

    fn variable_declaration(&self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, ParserError::MissingVariableName)?;
        let initializer = if self.match_token_types(&[TokenType::Equal])? {
            self.expression()?
        } else {
            Expr::literal(LiteralValue::None)
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterVariableDeclaration,
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
            ParserError::MissingLeftParenthesisAfterForStatement,
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
            ParserError::MissingSemicolonAfterLoopCondition,
        )?;

        let increment = if !self.check(TokenType::RightParen)? {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterForStatement,
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
            ParserError::MissingLeftParenthesisAfterIfStatement,
        )?;

        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterCondition,
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
            ParserError::MissingSemicolonAfterValue,
        )?;
        Ok(Stmt::print(value))
    }

    fn return_statement(&self) -> Result<Stmt, ParserError> {
        let expr = if !self.check(TokenType::Semicolon)? {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterExpression,
        )?;

        Ok(Stmt::return_stmt(expr))
    }

    fn while_statement(&self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            ParserError::MissingLeftParenthesisAfterWhileStatement,
        )?;

        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParserError::MissingRightParenthesisAfterCondition,
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
            ParserError::MissingRightBraceAfterBlock,
        )?;

        Ok(statements)
    }

    fn expression_statement(&self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterExpression,
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
                Expr::Variable { name } => Ok(Expr::assign(*name, value)),
                _ => Err(ParserError::InvalidAssignmentTarget),
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

                self.consume(
                    TokenType::RightParen,
                    ParserError::MissingRightParenthesisAfterArguments,
                )?;

                expr = Expr::call(expr, arguments);
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
                return Ok(Expr::literal(LiteralValue::Number(*value)));
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
                ParserError::MissingRightParenthesisAfterExpression,
            )?;
            return Ok(Expr::grouping(expr));
        }

        Err(ParserError::MissingExpression)
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
