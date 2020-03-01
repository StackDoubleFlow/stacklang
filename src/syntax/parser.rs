use crate::syntax::token::Token;

#[derive(Debug, Clone)]
struct ParseError {

}

pub struct Parser {
    tokens: Vec<Token>,
    i: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            i: 0
        }
    }
    
    pub fn parse(&mut self) {
        
    }
}