use crate::types::Position;

use thiserror::Error;

#[macro_export]
macro_rules! gen_error_message {
    ($msg: literal, $pos: ident) => {
        format!(
            "Error: {} (at {}:{}:{})",
            $msg, $pos.file_path, $pos.row, $pos.col
        )
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvlError {
    pub message: ErrorContext,
    pub position: Position,
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ErrorContext {
    #[error("{0} is required")]
    Required(String),

    #[error("{0} position is invalid")]
    InvalidPosition(String),

    #[error("The order must be variable name, equal sign, value, and semicolon")]
    InvalidSyntax,

    #[error("Invalid Type")]
    InvalidType,

    #[error("{0} isn't closed")]
    IsntClosed(String),

    #[error("Write {0} after the equal written")]
    AfterEqual(String),

    #[error("Item name not set")]
    ItemNotSet,

    #[error("That syntax can't be used whithin {0}")]
    InvalidUniqueSyntax(String),

    #[error("Can't input multiple characters in char")]
    MultipleChar,

    #[error("Use {0} only when closing {1}")]
    InvalidClosed(String, String),

    #[error("{0} is duplicated")]
    Duplicate(String),

    #[error("Can't use this syntax outside of the vars and settings blocks")]
    InvalidSettingsSyntax,

    #[error("To use this syntax, you must be inside a {0} block")]
    MustInBlock(String),

    #[error("Invalid {0} property")]
    InvalidProperty(String),

    #[error("Invalid syntax in {0} block")]
    InvalidSyntaxInBlock(String),

    #[error("There are invalid elements")]
    InvalidElements,

    #[error("Write within the block")]
    InBlock,
}
