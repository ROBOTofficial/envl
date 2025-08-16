use crate::misc::{position::Position, variable::VariableValue};

#[derive(Debug, PartialEq)]
pub enum Value {
    VariableName(String),
    VariableValue(VariableValue),
    Comment(String),
    Equal,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub value: Value,
    pub position: Position,
}
