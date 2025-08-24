use std::slice::Iter;

use crate::{
    misc::{
        config::Settings,
        position::Position,
        token::{Token, Value},
    },
    parser::{
        error::{
            template_to_error, ParserError, EQUAL_REQUIRED, INVALID_EQUAL,
            INVALID_LEFT_CURLY_POSITION, INVALID_SEMI, INVALID_SETTING, INVALID_SYNTAX_IN_SETTINGS,
            INVALID_TYPE, MUST_IN_VARS_BLOCK, SETTINGS_CLOSED,
        },
        parser::Parser,
    },
};

impl Parser {
    fn parse_string(&self, value: &String, position: &Position) -> Result<String, ParserError> {
        if value.starts_with('"') && value.ends_with('"') {
            let mut str_value = value.clone();
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
        let mut semi_used = false;
        let mut after_setting = false;
        let mut last_position = None;
        let mut target_prop = None;

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
                        equal_used = false;
                    }
                    Value::Semi => {
                        if semi_used || !after_setting {
                            error!(INVALID_SEMI);
                        }
                        semi_used = false;
                    }
                    Value::Ident(v) => {
                        if let Some(name) = target_prop {
                            if !equal_used {
                                error!(EQUAL_REQUIRED);
                            }
                            match name {
                                "envl_file_path" => match self.parse_string(v, &token.position) {
                                    Ok(value) => {
                                        settings.envl_file_path = Some(value);
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
                            equal_used = false;
                            after_setting = true;
                        } else {
                            target_prop = Some(v);
                        }
                    }
                    Value::Comment(_) => {}
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
