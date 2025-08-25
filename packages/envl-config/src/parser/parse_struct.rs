use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        token::{Token, Value},
        variable::Type,
    },
    parser::{
        error::{
            duplicate_error, template_to_error, ParserError, COLON_POSITION, COLON_REQUIRED,
            ELEMENT_NAME_REQUIRED, INVALID_ELEMENTS, INVALID_LEFT_CURLY_POSITION, INVALID_SYNTAX,
            MUST_IN_VARS_BLOCK, STRUCT_CLOSED,
        },
        Parser,
    },
};

impl Parser {
    pub fn parse_struct<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, ParserError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut colon_used = false;
        let mut last_position = None;
        let mut target_prop = None;
        let mut target_value = None;
        let mut elements = HashMap::new();

        let mut parser_error = None;

        'parse_loop: loop {
            if let Some(token) = tokens.next() {
                macro_rules! error {
                    ($err: expr) => {
                        parser_error = Some(template_to_error($err, token.position.clone()));
                        break 'parse_loop;
                    };
                }
                macro_rules! insert {
                    ($name: expr, $value: expr) => {
                        if !colon_used {
                            error!(COLON_REQUIRED);
                        }
                        if elements.get(&$name).is_some() {
                            let err = duplicate_error(&$name);
                            error!(err);
                        }
                        elements.insert($name, $value);
                        target_prop = None;
                        target_value = None;
                        colon_used = false;
                    };
                }

                last_position = Some(token.position.to_owned());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        if in_block {
                            error!(INVALID_LEFT_CURLY_POSITION);
                        }
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
                    error!(MUST_IN_VARS_BLOCK);
                }

                match &token.value {
                    Value::Colon => {
                        if colon_used || target_prop.is_none() {
                            error!(COLON_POSITION);
                        }
                        colon_used = true;
                    }
                    Value::Semi => match (target_prop, target_value) {
                        (Some(name), Some(value)) => {
                            insert!(name, value);
                        }
                        _ => {
                            error!(INVALID_SYNTAX);
                        }
                    },
                    Value::Ident(v) => {
                        if target_prop.is_some() {
                            error!(INVALID_ELEMENTS);
                        }
                        target_prop = Some(v.to_owned());
                    }
                    Value::Type(t) => {
                        if target_prop.is_some() {
                            if !colon_used {
                                error!(COLON_REQUIRED);
                            }
                            if target_value.is_some() {
                                error!(INVALID_SYNTAX);
                            }
                            target_value = Some(t.to_owned());
                        } else {
                            error!(ELEMENT_NAME_REQUIRED);
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

            Ok(Type::Struct(elements))
        }
    }
}
