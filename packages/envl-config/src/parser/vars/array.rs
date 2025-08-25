use std::slice::Iter;

use crate::{
    misc::{
        position::Position,
        token::{Token, Value},
        variable::Type,
    },
    parser::{
        error::{
            template_to_error, ParserError, ARRAY_CLOSED, INVALID_LEFT_SHIFT_POSITION,
            INVALID_SYNTAX, INVALID_TYPE, MUST_IN_VARS_BLOCK,
        },
        Parser,
    },
};

impl Parser {
    pub fn parse_array<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, ParserError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut last_position = None;
        let mut array_type = None;

        let mut parser_error = None;

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

                match &token.value {
                    Value::Array => {
                        if array_type.is_some() {
                            error!(INVALID_TYPE);
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
                    Value::Type(t) => {
                        if array_type.is_some() {
                            error!(INVALID_TYPE);
                        }
                        array_type = Some(t.to_owned());
                    }
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
        } else if let Some(t) = array_type {
            Ok(Type::Array(Box::from(t)))
        } else {
            if let Some(position) = last_position {
                if !block_closed {
                    return Err(template_to_error(ARRAY_CLOSED, position));
                } else {
                    return Err(template_to_error(INVALID_TYPE, position.clone()));
                }
            }

            Err(template_to_error(
                INVALID_TYPE,
                Position {
                    file_path: self.file_path.to_owned(),
                    col: 0,
                    row: 0,
                },
            ))
        }
    }
}
