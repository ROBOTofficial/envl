use std::slice::Iter;

use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
};

use crate::{
    misc::{
        config::{Setting, Settings},
        token::{Token, Value},
    },
    parser::Parser,
};

impl Parser {
    fn parse_string(&self, value: &str, position: &Position) -> Result<String, EnvlError> {
        if value.starts_with('"') && value.ends_with('"') {
            let mut str_value = value.to_owned();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            Ok(str_value)
        } else {
            Err(EnvlError {
                message: ErrorContext::InvalidType,
                position: position.clone(),
            })
        }
    }

    pub fn parse_settings<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Settings, EnvlError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut equal_used = false;
        let mut last_position = None;
        let mut target_prop = None;
        let mut target_value = None;

        let mut parser_error = None;
        let mut settings = Settings {
            envl_file_path: None,
        };

        'parse_loop: loop {
            if let Some(token) = tokens.next() {
                macro_rules! error {
                    ($msg: expr) => {
                        parser_error = Some(EnvlError {
                            message: $msg,
                            position: token.position.clone(),
                        });
                        break 'parse_loop;
                    };
                }

                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        if in_block {
                            error!(ErrorContext::InvalidPosition("{".to_string()));
                        }
                        in_block = true;
                        continue;
                    }
                    Value::RightCurlyBracket => {
                        block_closed = true;
                        break 'parse_loop;
                    }
                    _ => {}
                }

                if !in_block {
                    error!(ErrorContext::MustInBlock("vars".to_string()));
                }

                match &token.value {
                    Value::Equal => {
                        if equal_used || target_prop.is_none() {
                            error!(ErrorContext::InvalidPosition("Equal".to_string()));
                        }
                        equal_used = true;
                    }
                    Value::Semi => {
                        if let (Some(prop), Some(value)) = (target_prop, target_value) {
                            if !equal_used {
                                error!(ErrorContext::Required("Equal".to_string()));
                            }
                            match prop {
                                "envl_file_path" => match self.parse_string(value, &token.position)
                                {
                                    Ok(value) => {
                                        settings.envl_file_path = Some(Setting {
                                            value,
                                            position: token.position.clone(),
                                        });
                                    }
                                    Err(err) => {
                                        parser_error = Some(err);
                                        break 'parse_loop;
                                    }
                                },
                                _ => {
                                    error!(ErrorContext::InvalidProperty("settings".to_string()));
                                }
                            }
                        }
                    }
                    Value::Ident(v) => {
                        if target_prop.is_some() {
                            target_value = Some(v);
                        } else {
                            target_prop = Some(v);
                        }
                    }
                    _ => {
                        error!(ErrorContext::InvalidSyntaxInBlock("settings".to_string()));
                    }
                }
            } else {
                break 'parse_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !block_closed {
                    return Err(EnvlError {
                        message: ErrorContext::IsntClosed("Settings".to_string()),
                        position,
                    });
                }
            }

            Ok(settings)
        }
    }
}
