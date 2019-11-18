use crate::syntax::token::*;

use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct LexerError {
    msg: String
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.msg)
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    pub line: usize,
    pub col: usize,
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            line: 1,
            col: 0,
            source: source.chars().peekable(),
        }
    }

    fn push_token(&mut self, td: TokenData) {
        self.tokens.push(Token {
            line: self.line,
            col: self.col,
            data: td,
        });
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn next(&mut self) -> Result<char, LexerError> {
        match self.source.next() {
            Some(ch) => Ok(ch),
            None => Err(LexerError::new("Reached end of buffer")),
        }
    }

    pub fn lex(&mut self) -> Result<(), LexerError> {
        loop {
            if self.peek_next().is_none() {
                return Ok(());
            }
            let ch = self.next()?;
            self.col += 1;
            match ch {
                '\'' | '"' => {
                    let mut str = String::new();
                    loop {
                        match self.next()? {
                            '\'' if ch == '\'' => break,
                            '"' if ch == '"' => break,
                            '\n' => {
                                return Err(LexerError::new("Reached EOL while parsing string"))
                            }
                            c => str.push(c),
                        }
                    }
                    let str_len = str.len();
                    self.push_token(TokenData::Literal(Literal::String(str)));
                    self.col += str_len + 1;
                },
                '+' => self.push_token(TokenData::Operator(Operator::Add)),
                '-' => self.push_token(TokenData::Operator(Operator::Subtract)),
                '*' => self.push_token(TokenData::Operator(Operator::Multiply)),
                '/' => self.push_token(TokenData::Operator(Operator::Divide)),
                '(' => self.push_token(TokenData::Separator(Separator::OpeningParen)),
                ')' => self.push_token(TokenData::Separator(Separator::ClosingParen)),
                '{' => self.push_token(TokenData::Separator(Separator::OpeningBlock)),
                '}' => self.push_token(TokenData::Separator(Separator::ClosingBlock)),
                ';' => self.push_token(TokenData::Separator(Separator::Semicolon)),
                ',' => self.push_token(TokenData::Separator(Separator::Comma)),
                '\n' => {
                    self.line +=1;
                    self.col = 0;
                }
                _ => {}
            }
        }
    }
}
