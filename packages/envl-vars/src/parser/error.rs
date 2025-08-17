#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    SyntaxError,
    DuplicateVars,
    InvalidType,
    MultipleCharacters,
}
