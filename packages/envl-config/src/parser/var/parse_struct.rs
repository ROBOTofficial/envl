use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::token::{Token, Value},
    parser::{
        error::{
            template_to_error, ParserError, COLON_POSITION, COLON_REQUIRED, ELEMENT_NAME_REQUIRED,
            INVALID_ELEMENTS, INVALID_SYNTAX, IN_BLOCK, STRUCT_CLOSED,
        },
        var::array::parse_array,
        vars::option::ParsedValue,
    },
};

pub fn parse_struct<'a>(tokens: &mut Iter<'a, Token>) -> Result<ParsedValue, ParserError> {
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
                ($err: expr) => {
                    parser_error = Some(template_to_error($err, token.position.clone()));
                    break 'parse_loop;
                };
            }
            macro_rules! set_element_value {
                ($value: expr) => {
                    if !colon_used {
                        error!(COLON_REQUIRED);
                    }
                    if element_name.is_none() {
                        error!(ELEMENT_NAME_REQUIRED);
                    }
                    if element_value.is_some() {
                        error!(INVALID_SYNTAX);
                    }
                    element_value = Some($value);
                };
            }
            macro_rules! insert {
                () => {
                    if !colon_used {
                        error!(COLON_REQUIRED);
                    }
                    if let Some(ref name) = element_name {
                        if elements.get(name).is_some() {
                            error!(INVALID_ELEMENTS);
                        }
                        if let Some(ref value) = element_value {
                            elements.insert(name.clone(), value.clone());
                        } else {
                            error!(INVALID_ELEMENTS);
                        }
                    } else {
                        error!(ELEMENT_NAME_REQUIRED);
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
                error!(IN_BLOCK);
            }

            match &token.value {
                Value::Semi => {
                    insert!();
                }
                Value::Colon => {
                    if colon_used {
                        error!(COLON_POSITION);
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
                return Err(template_to_error(STRUCT_CLOSED, position));
            }
        }

        Ok(ParsedValue::Struct(elements))
    }
}
