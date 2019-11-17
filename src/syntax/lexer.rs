

use crate::syntax::token::Token;
use crate::syntax::token::TokenData;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    line: usize,
    column: usize,
    source: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            line: 0,
            column: 0,
            source
        }
    }

    fn push_token(&self, td: TokenData) {
        self.tokens.as_ref().push(Token { line: self.line, col: self.column, data: td });
    }

    pub fn lex(&self) {
        loop {

        }
    }
}
