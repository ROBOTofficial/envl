use std::slice::Iter;

use crate::{
    misc::{
        error::{EnvlVarsError, ErrorContext},
        token::{Token, Value},
        variable::VariableValue,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_array<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<VariableValue, EnvlVarsError> {
        let mut array_contents = Vec::new();
        let mut parser_error: Option<EnvlVarsError> = None;
        let mut comma_used = false;
        let mut array_closed = false;
        let mut last_position = None;

        'parse_array_loop: loop {
            if let Some(token) = tokens.next() {
                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftSquareBracket => match self.parse_array(tokens) {
                        Ok(v) => {
                            if !array_contents.is_empty() && !comma_used {
                                parser_error = Some(EnvlVarsError {
                                    message: ErrorContext::Required("Comma".to_string()),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(v.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::RightSquareBracket => {
                        array_closed = true;
                        break 'parse_array_loop;
                    }
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(EnvlVarsError {
                                message: ErrorContext::Required("Comma".to_string()),
                                position: token.position.clone(),
                            });
                            break 'parse_array_loop;
                        }
                        comma_used = true;
                    }
                    Value::Struct => match self.parse_struct(tokens) {
                        Ok(value) => {
                            if !array_contents.is_empty() && !comma_used {
                                parser_error = Some(EnvlVarsError {
                                    message: ErrorContext::Required("Comma".to_string()),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(value.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::Ident(value) => {
                        let value = self.parse_value(value, &token.position);
                        match value {
                            Ok(v) => {
                                if !array_contents.is_empty() && !comma_used {
                                    parser_error = Some(EnvlVarsError {
                                        message: ErrorContext::Required("Comma".to_string()),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_array_loop;
                                }
                                array_contents.push(v.clone());
                                comma_used = false;
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_array_loop;
                            }
                        }
                    }
                    Value::Comment(_) => {}
                    _ => {
                        parser_error = Some(EnvlVarsError {
                            message: ErrorContext::InvalidSyntax,
                            position: token.position.clone(),
                        });
                        break 'parse_array_loop;
                    }
                }
            } else {
                break 'parse_array_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !array_closed {
                    return Err(EnvlVarsError {
                        message: ErrorContext::IsntClosed("Array".to_string()),
                        position,
                    });
                }
            }
            Ok(VariableValue::Array(array_contents))
        }
    }
}
