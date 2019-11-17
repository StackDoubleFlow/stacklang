
pub enum Keyword {
    Fn,
    If,
    Let,
    For
}

pub enum Separator {
    OpeningParen, // (
    ClosingParen, // )
    OpeningBlock, // {
    ClosingBlock, // }
    Semicolon, // ;
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Literal {
    Boolean(bool),
    String(String),
    Int(i32),
    Long(i64)
}

pub struct Token {
    pub line: usize,
    pub col: usize,
    pub data: TokenData
}

pub enum TokenData {
    Identifier(String),
    Keyword(Keyword),
    Separator(Separator),
    Literal(Literal)
}