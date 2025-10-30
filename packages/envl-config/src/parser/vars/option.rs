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
    pub fn parse_option<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, EnvlError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut last_position = None;
        let mut optional_type = None;

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

                last_position = Some(token.position.to_owned());

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
                    error!(ErrorContext::MustInBlock("vars".to_string()));
                }
                if optional_type.is_some() {
                    error!(ErrorContext::InvalidType);
                }

                match &token.value {
                    Value::Type(t) => {
                        optional_type = Some(t.clone());
                    }
                    Value::Array => match self.parse_array(tokens) {
                        Ok(t) => {
                            optional_type = Some(t);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::Struct => match self.parse_struct(tokens) {
                        Ok(t) => {
                            optional_type = Some(t);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    _ => {
                        error!(ErrorContext::InvalidSyntaxInBlock("option".to_string()));
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
                        message: ErrorContext::IsntClosed("Option".to_string()),
                        position,
                    });
                }
            }
            if let Some(t) = optional_type {
                Ok(Type::Option(Box::from(t)))
            } else {
                Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position: Position {
                        file_path: self.file_path.to_owned(),
                        row: 0,
                        col: 0,
                    },
                })
            }
        }
    }
}
