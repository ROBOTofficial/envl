use std::slice::Iter;

use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
};

use crate::{
    misc::{
        token::{Token, Value},
        variable::Type,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_array<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, EnvlError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut last_position = None;
        let mut array_type = None;

        let mut parser_error = None;

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
                    Value::LeftShift => {
                        if in_block {
                            error!(ErrorContext::InvalidPosition("<".to_string()));
                        }
                        in_block = true;
                        continue;
                    }
                    Value::RightShift => {
                        block_closed = true;
                        break 'parse_loop;
                    }
                    _ => {}
                }

                if !in_block {
                    error!(ErrorContext::InvalidSyntaxInBlock("vars".to_string()));
                }

                match &token.value {
                    Value::Option => {
                        if array_type.is_some() {
                            error!(ErrorContext::InvalidType);
                        }
                        match self.parse_option(tokens) {
                            Ok(v) => {
                                array_type = Some(v);
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Array => {
                        if array_type.is_some() {
                            error!(ErrorContext::InvalidType);
                        }
                        match self.parse_array(tokens) {
                            Ok(v) => {
                                array_type = Some(v);
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Struct => {
                        if array_type.is_some() {
                            error!(ErrorContext::InvalidType);
                        }
                        match self.parse_struct(tokens) {
                            Ok(v) => {
                                array_type = Some(v);
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Type(t) => {
                        if array_type.is_some() {
                            error!(ErrorContext::InvalidType);
                        }
                        array_type = Some(t.to_owned());
                    }
                    _ => {
                        error!(ErrorContext::InvalidSyntaxInBlock("array".to_string()));
                    }
                }
            } else {
                break 'parse_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else if let Some(t) = array_type {
            Ok(Type::Array(Box::from(t)))
        } else {
            if let Some(position) = last_position {
                if !block_closed {
                    return Err(EnvlError {
                        message: ErrorContext::IsntClosed("Array".to_string()),
                        position,
                    });
                } else {
                    return Err(EnvlError {
                        message: ErrorContext::InvalidType,
                        position: position.clone(),
                    });
                }
            }

            Err(EnvlError {
                message: ErrorContext::InvalidType,
                position: Position {
                    file_path: self.file_path.to_owned(),
                    col: 0,
                    row: 0,
                },
            })
        }
    }
}
