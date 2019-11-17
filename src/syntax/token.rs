
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
}

pub enum Operator {

}

pub struct Token {
    pub line: usize,
    pub col: usize,
    pub data: TokenData
}

pub enum TokenData {
    Identifier(String),
    Keyword(Keyword),
    Separator(Separator)
}