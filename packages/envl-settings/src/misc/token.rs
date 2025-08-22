use crate::misc::position::Position;

#[derive(Debug, PartialEq)]
pub enum Value {
    Comment(String),
    Ident(String),
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    RightParentheses,
    LeftParentheses,
    Comma,
    Colon,
    Equal,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
