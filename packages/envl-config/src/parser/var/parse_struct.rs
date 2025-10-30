use std::{collections::HashMap, slice::Iter};

use envl_utils::error::{EnvlError, ErrorContext};

use crate::{
    misc::token::{Token, Value},
    parser::{var::array::parse_array, vars::option_value::ParsedValue},
};

pub fn parse_struct<'a>(tokens: &mut Iter<'a, Token>) -> Result<ParsedValue, EnvlError> {
    let mut in_block = false;
    let mut block_closed = false;
    let mut colon_used = false;
    let mut element_name: Option<String> = None;
    let mut element_value: Option<ParsedValue> = None;

    let mut elements = HashMap::new();
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
            macro_rules! set_element_value {
                ($value: expr) => {
                    if !colon_used {
                        error!(ErrorContext::Required("Colon".to_string()));
                    }
                    if element_name.is_none() {
                        error!(ErrorContext::Required("Element name".to_string()));
                    }
                    if element_value.is_some() {
                        error!(ErrorContext::InvalidSyntaxInBlock("struct".to_string()));
                    }
                    element_value = Some($value);
                };
            }
            macro_rules! insert {
                () => {
                    if !colon_used {
                        error!(ErrorContext::Required("Colon".to_string()));
                    }
                    if let Some(ref name) = element_name {
                        if elements.get(name).is_some() {
                            error!(ErrorContext::InvalidElements);
                        }
                        if let Some(ref value) = element_value {
                            elements.insert(name.clone(), value.clone());
                        } else {
                            error!(ErrorContext::InvalidElements);
                        }
                    } else {
                        error!(ErrorContext::Required("Element name".to_string()));
                    }
                };
            }

            last_position = Some(token.position.to_owned());

            match &token.value {
                Value::LeftCurlyBracket => {
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
                error!(ErrorContext::InBlock);
            }

            match &token.value {
                Value::Semi => {
                    insert!();
                }
                Value::Colon => {
                    if colon_used {
                        error!(ErrorContext::InvalidPosition("Colon".to_string()));
                    }
                    colon_used = true;
                }
                Value::Null => {
                    set_element_value!(ParsedValue::Null);
                }
                Value::Struct => match parse_struct(tokens) {
                    Ok(v) => {
                        set_element_value!(v);
                    }
                    Err(err) => {
                        parser_error = Some(err);
                        break 'parse_loop;
                    }
                },
                Value::LeftSquareBracket => match parse_array(tokens) {
                    Ok(v) => {
                        set_element_value!(v);
                    }
                    Err(err) => {
                        parser_error = Some(err);
                        break 'parse_loop;
                    }
                },
                Value::Ident(v) => {
                    if element_name.is_some() {
                        set_element_value!(ParsedValue::Value(v.clone()));
                    } else {
                        element_name = Some(v.to_owned());
                    }
                }
                _ => {
                    error!(ErrorContext::InvalidSyntaxInBlock("struct".to_string()));
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
                    message: ErrorContext::IsntClosed("Struct".to_string()),
                    position,
                });
            }
        }

        Ok(ParsedValue::Struct(elements))
    }
}
