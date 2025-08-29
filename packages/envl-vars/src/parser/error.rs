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

pub const COMMA_REQUIRED: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 1,
    message: "Comma is required",
};
pub const COMMA_POSITION: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 2,
    message: "Comma position is invalid",
};
pub const COLON_REQUIRED: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 3,
    message: "Colon is required",
};
pub const COLON_POSITION: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 4,
    message: "Colon position is invalid",
};
pub const DIFFERENT_ORDER: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 5,
    message: "The order must be variable name, equal sign, value, and semicolon",
};
pub const INVALID_TYPE: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::InvalidType,
    code: 6,
    message: "Invalid type",
};
pub const STRUCT_CLOSED: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 7,
    message: "Struct isn't closed",
};
pub const ARRAY_CLOSED: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 8,
    message: "Array isn't closed",
};
pub const ARRAY_AFTER_EQUAL: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 9,
    message: "Write arrays after the equal written",
};
pub const ITEM_NAME_NOT_SET: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 10,
    message: "Item name not set",
};
pub const SYNTAX_IN_STRUCT: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 11,
    message: "That syntax can't be used whithin a struct",
};
pub const SYNTAX_IN_ARRAY: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 12,
    message: "That syntax can't be used whithin a array",
};
pub const MULTIPLE_CHAR: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::MultipleCharacters,
    code: 13,
    message: "Can't input multiple characters in char",
};
pub const ARRAY_INVALID_CLOSE: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 14,
    message: "Use ] only when closing an array",
};
pub const STRUCT_INVALID_CLOSE: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 15,
    message: "Use } only when closing an struct",
};
pub const STRUCT_AFTER_EQUAL: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 16,
    message: "Write structs after the equal written",
};
pub const INVALID_ARRAY_POSITION: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 17,
    message: "Can't write an array at that position",
};
pub fn duplicate_error(name: &String) -> EnvlVarsError {
    let message = format!("{} is duplicated", name);
    let message_ref: &'static str = Box::leak(message.into_boxed_str());
    EnvlVarsError {
        kind: ErrorKind::DuplicateVars,
        code: 18,
        message: message_ref,
    }
}
pub const INVALID_SYNTAX: EnvlVarsError = EnvlVarsError {
    kind: ErrorKind::SyntaxError,
    code: 19,
    message: "That syntax can't be used",
};
