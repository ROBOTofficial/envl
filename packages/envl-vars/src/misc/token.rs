use crate::misc::position::Position;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Comment(String),
    Ident(String),
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    Comma,
    Colon,
    Equal,
    Semi,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
