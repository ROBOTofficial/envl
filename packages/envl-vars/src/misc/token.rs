use crate::misc::position::Position;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(String),
    Bool(bool),
    Variable(String),
    Comment(String),
    Equal,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
