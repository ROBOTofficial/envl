use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        error::{EnvlVarsError, ErrorContext},
        token::{Token, Value},
        variable::VariableValue,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_struct<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<VariableValue, EnvlVarsError> {
        let mut in_block = false;
        let mut hm = HashMap::new();
        let mut parser_error = None;
        let mut comma_used = false;
        let mut colon_used = false;
        let mut struct_closed = false;
        let mut last_position = None;
        let mut element_name: Option<String> = None;

        macro_rules! clean {
            () => {
                comma_used = false;
                colon_used = false;
                element_name = None;
            };
        }

        'parse_struct_loop: loop {
            if let Some(token) = tokens.next() {
                macro_rules! insert {
                    ($name: expr, $value: expr) => {
                        if hm.get(&$name).is_some() {
                            parser_error = Some(EnvlVarsError {
                                message: ErrorContext::Duplicate($name.to_string()),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        hm.insert($name, $value);
                    };
                }

                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        in_block = true;
                        continue 'parse_struct_loop;
                    }
                    Value::RightCurlyBracket => {
                        struct_closed = true;
                        break 'parse_struct_loop;
                    }
                    _ => {}
                }

                if !in_block {
                    parser_error = Some(EnvlVarsError {
                        message: ErrorContext::InvalidSyntax,
                        position: token.position.clone(),
                    });
                    break 'parse_struct_loop;
                }

                match &token.value {
                    Value::Struct => match self.parse_struct(tokens) {
                        Ok(value) => match element_name {
                            Some(name) => {
                                if !colon_used {
                                    parser_error = Some(EnvlVarsError {
                                        message: ErrorContext::InvalidPosition("Colon".to_string()),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                if !hm.is_empty() && !comma_used {
                                    parser_error = Some(EnvlVarsError {
                                        message: ErrorContext::InvalidPosition("Comma".to_string()),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                insert!(name, value);
                                clean!();
                            }
                            None => {
                                parser_error = Some(EnvlVarsError {
                                    message: ErrorContext::ItemNotSet,
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                        },
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_struct_loop;
                        }
                    },
                    Value::LeftSquareBracket => match self.parse_array(tokens) {
                        Ok(value) => {
                            if let Some(name) = element_name {
                                if !colon_used {
                                    parser_error = Some(EnvlVarsError {
                                        message: ErrorContext::Required("Colon".to_string()),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                if !hm.is_empty() && !comma_used {
                                    parser_error = Some(EnvlVarsError {
                                        message: ErrorContext::Required("Comma".to_string()),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                insert!(name, value);
                                clean!();
                            } else {
                                parser_error = Some(EnvlVarsError {
                                    message: ErrorContext::AfterEqual("array".to_string()),
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_struct_loop;
                        }
                    },
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(EnvlVarsError {
                                message: ErrorContext::InvalidPosition("Comma".to_string()),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        comma_used = true;
                    }
                    Value::Colon => {
                        if colon_used {
                            parser_error = Some(EnvlVarsError {
                                message: ErrorContext::InvalidPosition("Colon".to_string()),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        colon_used = true;
                    }
                    Value::Ident(v) => match element_name.clone() {
                        None => {
                            element_name = Some(v.clone());
                        }
                        Some(name) if colon_used => {
                            if !hm.is_empty() && !comma_used {
                                parser_error = Some(EnvlVarsError {
                                    message: ErrorContext::Required("Comma".to_string()),
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                            match self.parse_value(v, &token.position.clone()) {
                                Ok(value) => {
                                    insert!(name, value);
                                    clean!();
                                }
                                Err(err) => {
                                    parser_error = Some(err);
                                    break 'parse_struct_loop;
                                }
                            }
                        }
                        _ => {
                            let message = if !colon_used {
                                ErrorContext::Required("colon".to_string())
                            } else {
                                ErrorContext::ItemNotSet
                            };
                            parser_error = Some(EnvlVarsError {
                                message,
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                    },
                    _ => {
                        parser_error = Some(EnvlVarsError {
                            message: ErrorContext::AfterEqual("struct".to_string()),
                            position: token.position.clone(),
                        });
                        break 'parse_struct_loop;
                    }
                }
            } else {
                break 'parse_struct_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !struct_closed {
                    return Err(EnvlVarsError {
                        message: ErrorContext::AfterEqual("struct".to_string()),
                        position,
                    });
                }
            }
            Ok(VariableValue::Struct(hm))
        }
    }
}
