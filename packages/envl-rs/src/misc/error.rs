use std::io::Error;

use envl_utils::{error::EnvlError as EnvlVarsError, types::Position};

#[derive(Debug, Clone)]
pub struct EnvlLibError {
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    Vars(EnvlVarsError),
    Io(Error),
    Lib(EnvlLibError),
}

#[derive(Debug)]
pub struct EnvlError {
    pub message: String,
    pub position: Option<Position>,
    pub kind: ErrorKind,
}

pub fn convert_envl_vars_error(err: EnvlVarsError) -> EnvlError {
    EnvlError {
        message: err.message.to_string(),
        position: None,
        kind: ErrorKind::Vars(err),
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
