use crate::misc::position::Position;

#[derive(Clone, PartialEq, Debug)]
pub enum VariableValue {
    String(String),
    Number(String),
    Bool(bool),
    Char(char),
    Array(Vec<VariableValue>),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: VariableValue,
    pub position: Position,
}

#[derive(Debug, PartialEq)]
pub struct VariableWithoutPosition {
    pub name: String,
    pub value: VariableValue,
}
