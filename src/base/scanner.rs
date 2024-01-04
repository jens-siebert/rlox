use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String { value: String },
    Number { value: f64 },

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub type TokenRef = Rc<Token>;

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn new_ref(token_type: TokenType, lexeme: String, line: usize) -> Rc<Self> {
        Rc::new(Token::new(token_type, lexeme, line))
    }
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unknown symbol {symbol:?} detected in line {line:?}!")]
    UnknownSymbol { symbol: char, line: usize },
    #[error("Unterminated string in line {line:?}!")]
    UnterminatedString { line: usize },
    #[error("Error while parsing number {number_string:?} in line {line:?}!")]
    NumberParsingError { number_string: String, line: usize },
}

pub struct Scanner {
    source: Vec<char>,
    tokens: RefCell<Vec<TokenRef>>,
    start_pos: usize,
    current_pos: usize,
    current_line: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Scanner {
            source: input.chars().collect(),
            tokens: RefCell::new(vec![]),
            start_pos: 0,
            current_pos: 0,
            current_line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Rc<Vec<TokenRef>>, ScannerError> {
        while !self.is_at_end() {
            self.start_pos = self.current_pos;
            self.scan_token()?;
        }

        self.tokens.borrow_mut().push(Token::new_ref(
            TokenType::Eof,
            String::from(""),
            self.current_line,
        ));

        Ok(Rc::new(self.tokens.borrow().clone()))
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let t = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t)
            }
            '=' => {
                let t = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t)
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t)
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.match_string(),
            '\n' => {
                self.current_line += 1;
                Ok(())
            }
            ' ' | '\r' | '\t' => {
                /* ignore whitespaces. */
                Ok(())
            }
            c => {
                if c.is_ascii_digit() {
                    self.match_number()
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.match_identifier()
                } else {
                    Err(ScannerError::UnknownSymbol {
                        symbol: c,
                        line: self.current_line,
                    })
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), ScannerError> {
        let token_string: String = self.source[self.start_pos..self.current_pos]
            .iter()
            .collect();
        self.tokens
            .borrow_mut()
            .push(Token::new_ref(token_type, token_string, self.current_line));

        Ok(())
    }

    fn add_string_token(&mut self, value: String) -> Result<(), ScannerError> {
        let token_string: String = self.source[self.start_pos..self.current_pos]
            .iter()
            .collect();
        self.tokens.borrow_mut().push(Token::new_ref(
            TokenType::String { value },
            token_string,
            self.current_line,
        ));

        Ok(())
    }

    fn add_number_token(&mut self, value: f64) -> Result<(), ScannerError> {
        let token_string: String = self.source[self.start_pos..self.current_pos]
            .iter()
            .collect();
        self.tokens.borrow_mut().push(Token::new_ref(
            TokenType::Number { value },
            token_string,
            self.current_line,
        ));

        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current_pos] != expected {
            return false;
        }

        self.current_pos += 1;

        true
    }

    fn match_string(&mut self) -> Result<(), ScannerError> {
        let start_line = self.current_line;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.current_line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { line: start_line });
        }

        self.advance();
        self.add_string_token(
            self.source[self.start_pos + 1..self.current_pos - 1]
                .iter()
                .collect(),
        )
    }

    fn match_number(&mut self) -> Result<(), ScannerError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let number_string: String = self.source[self.start_pos..self.current_pos]
            .iter()
            .collect();
        let number = f64::from_str(number_string.as_str()).map_err(|_| {
            ScannerError::NumberParsingError {
                number_string,
                line: self.current_line,
            }
        })?;

        self.add_number_token(number)
    }

    fn match_identifier(&mut self) -> Result<(), ScannerError> {
        loop {
            let c = self.peek();
            if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier_string: String = self.source[self.start_pos..self.current_pos]
            .iter()
            .collect();

        let t = match identifier_string.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(t)
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current_pos];
        self.current_pos += 1;

        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current_pos]
        }
    }

    fn peek_next(&self) -> char {
        if self.current_pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current_pos + 1]
        }
    }
}
