use std::slice::Iter;

use crate::{
    misc::{
        config::{Setting, Settings},
        position::Position,
        token::{Token, Value},
    },
    parser::{
        error::{
            template_to_error, ParserError, EQUAL_REQUIRED, INVALID_EQUAL,
            INVALID_LEFT_CURLY_POSITION, INVALID_SETTING, INVALID_SYNTAX_IN_SETTINGS, INVALID_TYPE,
            MUST_IN_VARS_BLOCK, SETTINGS_CLOSED,
        },
        Parser,
    },
};

impl Parser {
    fn parse_string(&self, value: &str, position: &Position) -> Result<String, ParserError> {
        if value.starts_with('"') && value.ends_with('"') {
            let mut str_value = value.to_owned();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            Ok(str_value)
        } else {
            Err(template_to_error(INVALID_TYPE, position.clone()))
        }
    }

    pub fn parse_settings<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<Settings, ParserError> {
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
                    ($err: expr) => {
                        parser_error = Some(template_to_error($err, token.position.clone()));
                        break 'parse_loop;
                    };
                }

                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        if in_block {
                            error!(INVALID_LEFT_CURLY_POSITION);
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
                    error!(MUST_IN_VARS_BLOCK);
                }

                match &token.value {
                    Value::Equal => {
                        if equal_used || target_prop.is_none() {
                            error!(INVALID_EQUAL);
                        }
                        equal_used = true;
                    }
                    Value::Semi => {
                        if let (Some(prop), Some(value)) = (target_prop, target_value) {
                            if !equal_used {
                                error!(EQUAL_REQUIRED);
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
                                    error!(INVALID_SETTING);
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
                        error!(INVALID_SYNTAX_IN_SETTINGS);
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
                    return Err(template_to_error(SETTINGS_CLOSED, position));
                }
            }

            Ok(settings)
        }
    }
}
