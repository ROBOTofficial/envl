use crate::misc::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    SyntaxError,
    DuplicateVars,
    InvalidType,
    MultipleCharacters,
}

pub struct EnvlVarsError {
    pub kind: ErrorKind,
    pub code: u32,
    pub message: &'static str,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub code: u32,
    pub kind: ErrorKind,
    pub message: String,
    pub position: Position,
}
