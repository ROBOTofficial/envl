use crate::misc::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    SyntaxError,
    DuplicateVars,
    InvalidType,
    MultipleCharacters,
}

pub struct EnvlConfigErrorTemplate {
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

pub fn template_to_error(template: EnvlConfigErrorTemplate, position: Position) -> ParserError {
    ParserError {
        code: template.code,
        kind: template.kind,
        message: template.message.to_string(),
        position,
    }
}

pub const INVALID_LEFT_CURLY_POSITION: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 1,
    message: "Position of the { is invalid",
};
pub const VARS_CLOSED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 2,
    message: "Vars is not closed",
};
pub const SETTINGS_AND_VARS_REQUIRED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 3,
    message: "Settings and vars is required",
};
pub const MUST_IN_VARS_BLOCK: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 4,
    message: "To use this syntax, you must be inside a vars block",
};
pub const COMMA_REQUIRED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 5,
    message: "Comma is required",
};
pub const COMMA_POSITION: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 6,
    message: "Comma position is invalid",
};
pub const COLON_REQUIRED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 7,
    message: "Colon is required",
};
pub const COLON_POSITION: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 8,
    message: "Colon position is invalid",
};
pub const INVALID_SYNTAX_IN_VARS: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 9,
    message: "That syntax cannot be used within a vars block",
};
pub const INVALID_ELEMENTS: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 10,
    message: "There are invalid elements",
};
pub const ELEMENT_NAME_REQUIRED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 11,
    message: "Element name is required",
};
pub fn duplicate_error(name: &String) -> EnvlConfigErrorTemplate {
    let message = format!("{} is duplicated", name);
    let message_ref: &'static str = Box::leak(message.into_boxed_str());
    EnvlConfigErrorTemplate {
        kind: ErrorKind::DuplicateVars,
        code: 12,
        message: message_ref,
    }
}
pub const INVALID_SYNTAX_OUTSIDE: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 13,
    message: "Can't use this syntax outside of the vars and settings blocks",
};
pub const SETTINGS_CLOSED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 14,
    message: "Settings is not closed",
};
pub const INVALID_SYNTAX_IN_SETTINGS: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 15,
    message: "That syntax cannot be used within a settings block",
};
pub const INVALID_EQUAL: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 16,
    message: "Invalid equal position",
};
pub const INVALID_SEMI: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 16,
    message: "Invalid semi position",
};
pub const EQUAL_REQUIRED: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 17,
    message: "Equal is required",
};
pub const INVALID_SETTING: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 18,
    message: "Invalid setting property",
};
pub const INVALID_TYPE: EnvlConfigErrorTemplate = EnvlConfigErrorTemplate {
    kind: ErrorKind::SyntaxError,
    code: 18,
    message: "Invalid type",
};
