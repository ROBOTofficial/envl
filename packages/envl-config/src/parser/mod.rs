use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
};

use crate::misc::{
    config::Config,
    token::{Token, Value},
};

pub mod settings;
pub mod value;
pub mod var;
pub mod vars;

pub struct Parser {
    pub file_path: String,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(file_path: String, tokens: Vec<Token>) -> Self {
        Self { file_path, tokens }
    }

    pub fn parse(&self) -> Result<Config, EnvlError> {
        let mut based_token = vec![];

        for token in self.tokens.iter() {
            match token.value {
                Value::Comment(_) => {
                    continue;
                }
                _ => {
                    based_token.push(token.clone());
                }
            }
        }

        let mut tokens = based_token.iter();
        let mut parser_error = None;
        let mut vars = None;
        let mut settings = None;

        'parse_loop: loop {
            macro_rules! error {
                ($err: expr) => {
                    parser_error = Some($err);
                    break 'parse_loop;
                };
            }

            if let Some(token) = tokens.next() {
                match token.value {
                    Value::Vars => match self.parse_vars(&mut tokens) {
                        Ok(result) => {
                            vars = Some(result);
                        }
                        Err(err) => {
                            error!(err);
                        }
                    },
                    Value::Settings => match self.parse_settings(&mut tokens) {
                        Ok(result) => {
                            settings = Some(result);
                        }
                        Err(err) => {
                            error!(err);
                        }
                    },
                    _ => {
                        error!(EnvlError {
                            message: ErrorContext::InvalidSettingsSyntax,
                            position: token.position.clone()
                        });
                    }
                }
            } else {
                break 'parse_loop;
            }
        }

        if let Some(err) = parser_error {
            return Err(err);
        }

        match (vars, settings) {
            (Some(vars), Some(settings)) => Ok(Config { settings, vars }),
            _ => Err(EnvlError {
                message: ErrorContext::Required("Settings and vars".to_string()),
                position: Position {
                    file_path: self.file_path.clone(),
                    row: 0,
                    col: 0,
                },
            }),
        }
    }
}
