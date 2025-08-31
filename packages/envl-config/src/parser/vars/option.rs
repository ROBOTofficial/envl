use std::slice::Iter;

use envl_utils::types::Position;

use crate::{
    misc::{
        token::{Token, Value},
        variable::Type,
    },
    parser::{
        error::{
            template_to_error, ParserError, INVALID_LEFT_SHIFT_POSITION, INVALID_OPTIONAL,
            INVALID_SYNTAX, MUST_IN_VARS_BLOCK, OPTION_CLOSED,
        },
        Parser,
    },
};

impl Parser {
    pub fn parse_option<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, ParserError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut last_position = None;
        let mut optional_type = None;

        let mut parser_error = None;

        'parse_loop: loop {
            if let Some(token) = tokens.next() {
                macro_rules! error {
                    ($err: expr) => {
                        parser_error = Some(template_to_error($err, token.position.clone()));
                        break 'parse_loop;
                    };
                }

                last_position = Some(token.position.to_owned());

                match &token.value {
                    Value::LeftShift => {
                        if in_block {
                            error!(INVALID_LEFT_SHIFT_POSITION);
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
                    error!(MUST_IN_VARS_BLOCK);
                }
                if optional_type.is_some() {
                    error!(INVALID_OPTIONAL);
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
                        error!(INVALID_SYNTAX);
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
                    return Err(template_to_error(OPTION_CLOSED, position));
                }
            }
            if let Some(t) = optional_type {
                Ok(Type::Option(Box::from(t)))
            } else {
                Err(template_to_error(
                    INVALID_OPTIONAL,
                    Position {
                        file_path: self.file_path.to_owned(),
                        col: 0,
                        row: 0,
                    },
                ))
            }
        }
    }
}
