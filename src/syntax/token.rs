
#[derive(Debug)]
pub enum Keyword {
    Fn,
    If,
    Let,
    For,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Separator {
    OpeningParen, // (
    ClosingParen, // )
    OpeningBlock, // {
    ClosingBlock, // }
    Semicolon,    // ;
    Comma,        // ,
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Int(i32),
    Long(i64),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Literal::*;
        match self {
            Boolean(val) => write!(f, "{}", val),
            String(val) => write!(f, "{}", val),
            Int(val) => write!(f, "{}", val),
            Long(val) => write!(f, "{}", val)
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub col: usize,
    pub data: TokenData,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} :: {}", self.line, self.col, self.data)
    }
}

#[derive(Debug)]
pub enum TokenData {
    Identifier(String),
    Keyword(Keyword),
    Separator(Separator),
    Literal(Literal),
    Operator(Operator)
}

impl std::fmt::Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenData::*;
        match self {
            Identifier(name) => write!(f, "(Identifier, {}", name),
            Keyword(keyword) => write!(f, "(Keyword, {}", keyword),
            Separator(separator) => write!(f, "(Keyword, {}", separator),
            Literal(lit) => write!(f, "(Literal, {}", lit),
            Operator(operator) => write!(f, "(Keyword, {}", operator),
        }
    }
}