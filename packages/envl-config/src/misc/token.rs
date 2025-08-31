use envl_utils::types::Position;

use crate::misc::variable::Type;

#[derive(Debug, PartialEq, Clone)]
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
    RightShift,
    LeftShift,
    Settings,
    Struct,
    Array,
    Comma,
    Colon,
    Equal,
    Null,
    Vars,
    Semi,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
