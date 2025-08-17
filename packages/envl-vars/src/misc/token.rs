use crate::misc::position::Position;

#[derive(Debug, PartialEq)]
pub enum Value {
    Ident(String),
    Comment(String),
    RightBracket,
    LeftBracket,
    Comma,
    Equal,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
