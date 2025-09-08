use std::slice::Iter;

use envl_utils::error::{EnvlError, ErrorContext};

use crate::{
    misc::token::{Token, Value},
    parser::{var::parse_struct::parse_struct, vars::option_value::ParsedValue},
};

pub fn parse_array<'a>(tokens: &mut Iter<'a, Token>) -> Result<ParsedValue, EnvlError> {
    let mut block_closed = false;
    let mut comma_used = false;
    let mut elements = Vec::new();

    let mut last_position = None;
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
            macro_rules! insert {
                ($value: expr) => {
                    if !elements.is_empty() && !comma_used {
                        error!(ErrorContext::Required("Comma".to_string()));
                    }
                    elements.push($value.clone());
                    comma_used = false;
                };
            }

            last_position = Some(token.position.to_owned());

            match &token.value {
                Value::Comma => {
                    if comma_used {
                        error!(ErrorContext::Required("Comma".to_string()));
                    }
                    comma_used = true;
                }
                Value::Array => match parse_array(tokens) {
                    Ok(v) => {
                        insert!(v);
                    }
                    Err(err) => {
                        parser_error = Some(err);
                        break 'parse_loop;
                    }
                },
                Value::LeftSquareBracket => match parse_array(tokens) {
                    Ok(v) => {
                        insert!(v);
                    }
                    Err(err) => {
                        parser_error = Some(err);
                        break 'parse_loop;
                    }
                },
                Value::RightSquareBracket => {
                    block_closed = true;
                    break 'parse_loop;
                }
                Value::Null => {
                    elements.push(ParsedValue::Null);
                }
                Value::Struct => match parse_struct(tokens) {
                    Ok(v) => {
                        insert!(v);
                    }
                    Err(err) => {
                        parser_error = Some(err);
                        break 'parse_loop;
                    }
                },
                Value::Ident(v) => {
                    elements.push(ParsedValue::Value(v.to_owned()));
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
    } else {
        if let Some(position) = last_position {
            if !block_closed {
                return Err(EnvlError {
                    message: ErrorContext::IsntClosed("Array".to_string()),
                    position,
                });
            }
        }

        Ok(ParsedValue::Array(elements))
    }
}
