use crate::misc::position::Position;

pub enum Type {}

pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Comment,
}

pub struct Token {
    pub value: Value,
    pub position: Position,
}
