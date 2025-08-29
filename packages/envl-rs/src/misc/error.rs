use std::io::Error;

use envl_config::{misc::position::Position, parser::error::ParserError};
use envl_vars::parser::ParserError as VarsParserError;

#[derive(Debug, Clone)]
pub struct EnvlLibError {
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    Vars(VarsParserError),
    Config(ParserError),
    Io(Error),
    Lib(EnvlLibError),
}

#[derive(Debug)]
pub struct EnvlError {
    pub message: String,
    pub position: Option<Position>,
    pub kind: ErrorKind,
}

pub fn convert_envl_vars_error(err: VarsParserError) -> EnvlError {
    EnvlError {
        message: err.message.to_string(),
        position: None,
        kind: ErrorKind::Vars(err),
    }
}

pub fn convert_envl_config_error(err: ParserError) -> EnvlError {
    EnvlError {
        message: err.message.to_string(),
        position: Some(err.position.clone()),
        kind: ErrorKind::Config(err),
    }
}

pub fn convert_io_error(err: Error) -> EnvlError {
    EnvlError {
        message: err.to_string().clone(),
        position: None,
        kind: ErrorKind::Io(err),
    }
}

pub fn convert_envl_lib_error(err: EnvlLibError) -> EnvlError {
    EnvlError {
        message: err.message.to_string(),
        position: None,
        kind: ErrorKind::Lib(err),
    }
}
