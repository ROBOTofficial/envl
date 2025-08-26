use crate::{
    misc::{
        config::Config,
        position::Position,
        token::{Token, Value},
    },
    parser::error::{
        template_to_error, ParserError, INVALID_SYNTAX_OUTSIDE, SETTINGS_AND_VARS_REQUIRED,
    },
};

pub mod error;
pub mod settings;
pub mod value;
pub mod vars;

pub struct Parser {
    pub file_path: String,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(file_path: String, tokens: Vec<Token>) -> Self {
        Self { file_path, tokens }
    }

    pub fn parse(&self) -> Result<Config, ParserError> {
        let mut tokens = self.tokens.iter();
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
                    Value::Comment(_) => {}
                    _ => {
                        error!(template_to_error(
                            INVALID_SYNTAX_OUTSIDE,
                            token.position.clone()
                        ));
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
            _ => Err(template_to_error(
                SETTINGS_AND_VARS_REQUIRED,
                Position {
                    file_path: self.file_path.clone(),
                    row: 0,
                    col: 0,
                },
            )),
        }
    }
}
