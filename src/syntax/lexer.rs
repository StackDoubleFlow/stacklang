use crate::syntax::token::*;

use std::error::Error;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

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
                '=' => self.push_token(TokenData::Operator(Operator::Assignment)),
                '(' => self.push_token(TokenData::Separator(Separator::OpeningParen)),
                ')' => self.push_token(TokenData::Separator(Separator::ClosingParen)),
                '{' => self.push_token(TokenData::Separator(Separator::OpeningBlock)),
                '}' => self.push_token(TokenData::Separator(Separator::ClosingBlock)),
                ';' => self.push_token(TokenData::Separator(Separator::Semicolon)),
                ',' => self.push_token(TokenData::Separator(Separator::Comma)),
                '.' => self.push_token(TokenData::Separator(Separator::Dot)),
                '\n' => {
                    self.line +=1;
                    self.col = 0;
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let mut text = ch.to_string();
                    while let Some(next) = self.peek_next() {
                        if next.is_alphanumeric() || next == '_' {
                            text.push(self.next()?);
                        } else {
                            break;
                        }
                    }
                    self.push_token(match text.as_str() {
                        "true" => TokenData::Literal(Literal::Boolean(true)),
                        "false" => TokenData::Literal(Literal::Boolean(false)),
                        _ => {
                            if let Ok(keyword) = Keyword::from_str(text.as_str()) {
                                TokenData::Keyword(keyword)
                            } else {
                                TokenData::Identifier(text)
                            }
                        }
                    });
                }
                _ if ch.is_numeric() => {
                    let mut text = ch.to_string();
                    while let Some(next) = self.peek_next() {
                        if next.is_numeric() {
                            text.push(self.next()?);
                        } else {
                            break;
                        }
                    }
                    let int : i64 = match text.parse() {
                        Ok(i) => i,
                        Err(_) => return Err(LexerError::new("Failed to parse number")) 
                    };
                    let text_len = text.len();
                    if int > 2_147_483_647 {
                        self.push_token(TokenData::Literal(Literal::Long(int)));
                    } else {
                        self.push_token(TokenData::Literal(Literal::Int(int as i32)));
                    }
                    self.col += text_len;
                }
                _ => {}
            }
        }
    }
}
