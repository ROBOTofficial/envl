use crate::misc::{position::Position, variable::Type};

#[derive(Debug, PartialEq)]
pub enum Value {
    Comment(String),
    Ident(String),
    Type(Type),
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    RightParentheses,
    LeftParentheses,
    Settings,
    Vars,
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
