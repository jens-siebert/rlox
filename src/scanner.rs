use thiserror::Error;

enum TokenType {
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
    String,
    Number,

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

    Eof
}

struct Token  {
    token_type: TokenType
}

impl Token {
    fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unknown symbol detected!")]
    UnknownSymbol
}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    current_pos: usize,
    current_line: usize
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Scanner { source: input.chars().collect(), tokens: vec![], current_pos: 0, current_line: 0 }
    }

    pub fn scan_tokens(&mut self) -> Result<(), ScannerError> {
        while !self.is_at_end() {
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof));

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        match self.advance() {
            '(' => self.tokens.push(Token::new(TokenType::LeftParen)),
            ')' => self.tokens.push(Token::new(TokenType::RightParen)),
            '{' => self.tokens.push(Token::new(TokenType::LeftBrace)),
            '}' => self.tokens.push(Token::new(TokenType::RightBrace)),
            ',' => self.tokens.push(Token::new(TokenType::Comma)),
            '.' => self.tokens.push(Token::new(TokenType::Dot)),
            '-' => self.tokens.push(Token::new(TokenType::Minus)),
            '+' => self.tokens.push(Token::new(TokenType::Plus)),
            ';' => self.tokens.push(Token::new(TokenType::Semicolon)),
            '*' => self.tokens.push(Token::new(TokenType::Star)),
            '!' => {
                let t = if self.match_expected('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.tokens.push(Token::new(t));
            },
            '=' => {
                let t = if self.match_expected('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.tokens.push(Token::new(t));
            },
            '<' => {
                let t = if self.match_expected('=') { TokenType::LessEqual } else { TokenType::Less };
                self.tokens.push(Token::new(t));
            },
            '>' => {
                let t = if self.match_expected('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.tokens.push(Token::new(t));
            },
            '/' => {
                if self.match_expected('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::new(TokenType::Slash))
                }
            }
            _ => return Err(ScannerError::UnknownSymbol)
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current_pos];
        self.current_pos += 1;

        c
    }

    fn match_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current_pos] != expected {
            return false;
        }

        self.current_pos += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current_pos]
        }
    }
}