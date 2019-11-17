

use crate::syntax::token::*;

use std::iter::Peekable;
use std::str::Chars;

pub struct LexerError {
    msg: String
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string()
        }
    }
}

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    line: usize,
    col: usize,
    source: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            line: 0,
            col: 0,
            source: source.chars().peekable()
        }
    }

    fn push_token(&mut self, td: TokenData) {
        self.tokens.push(Token { line: self.line, col: self.col, data: td });
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn next(&mut self) -> Result<char, LexerError> {
        match self.source.next() {
            Some(ch) => Ok(ch),
            None => Err(LexerError::new("Reached end of file"))
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
                        self.col += 1;
                        match self.next()? {
                            '\'' if ch == '\'' => break,
                            '"' if ch == '"' => break,
                            '\n' => return Err(LexerError::new("Reached EOL while parsing string")),
                            c => str.push(c)
                        }
                    }
                    self.push_token(TokenData::Literal(Literal::String(str)));
                }
                _ => {

                }
            }
            
        }
    }
}
