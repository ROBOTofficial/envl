use crate::misc::position::Position;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Variable(String),
    Comment,
    Equal,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
