use crate::misc::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    SyntaxError,
    DuplicateVars,
    InvalidType,
    MultipleCharacters,
}

pub struct EnvlConfigError {
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

pub fn template_to_error(template: EnvlConfigError, position: Position) -> ParserError {
    ParserError {
        code: template.code,
        kind: template.kind,
        message: template.message.to_string(),
        position,
    }
}
