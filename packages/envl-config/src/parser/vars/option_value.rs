use std::{collections::HashMap, slice::Iter};

use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
};

use crate::{
    misc::{
        token::{Token, Value},
        variable::{Type, Value as ConfigValue},
    },
    parser::{
        value::parse_value,
        var::{array::parse_array, parse_struct::parse_struct},
        Parser,
    },
};

#[derive(Debug, Clone)]
pub enum ParsedValue {
    Array(Vec<ParsedValue>),
    Struct(HashMap<String, ParsedValue>),
    Value(String),
    Null,
}

pub fn parse_parsed_value(
    v: ParsedValue,
    t: Type,
    position: Position,
) -> Result<ConfigValue, EnvlError> {
    match v {
        ParsedValue::Null => Ok(ConfigValue::Null),
        ParsedValue::Value(value) => match parse_value(t, value) {
            Ok(result) => Ok(result),
            Err(err) => Err(EnvlError {
                message: err,
                position,
            }),
        },
        ParsedValue::Struct(values) => match t {
            Type::Struct(t) => {
                let mut elements = HashMap::new();

                for (name, value) in values {
                    if let Some(value_type) = t.get(&name) {
                        match parse_parsed_value(value, value_type.clone(), position.clone()) {
                            Ok(result) => {
                                if elements.contains_key(&name) {
                                    return Err(EnvlError {
                                        message: ErrorContext::InvalidElements,
                                        position,
                                    });
                                }
                                elements.insert(name.clone(), result);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return Err(EnvlError {
                            message: ErrorContext::InvalidType,
                            position,
                        });
                    }
                }

                Ok(ConfigValue::Struct(elements))
            }
            _ => Err(EnvlError {
                message: ErrorContext::InvalidType,
                position,
            }),
        },
        ParsedValue::Array(values) => match t {
            Type::Array(boxed_type) => {
                let t = *boxed_type;
                let mut elements = Vec::new();

                for value in values {
                    match parse_parsed_value(value, t.to_owned(), position.to_owned()) {
                        Ok(element) => {
                            elements.push(element);
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                Ok(ConfigValue::Array(elements))
            }
            _ => Err(EnvlError {
                message: ErrorContext::InvalidType,
                position,
            }),
        },
    }
}

impl Parser {
    pub fn parse_option_value<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<(ParsedValue, ParsedValue), EnvlError> {
        let mut block_closed = false;
        let mut comma_used = false;
        let mut colon_used = false;
        let mut element_name = None;
        let mut last_position = None;
        let mut inserted_count = 0;

        let mut default_value = ParsedValue::Null;
        let mut actions_value = ParsedValue::Null;
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
                        if !colon_used {
                            error!(ErrorContext::Required("Colon".to_string()));
                        }
                        if !comma_used && inserted_count != 0 {
                            error!(ErrorContext::Required("Comma".to_string()));
                        } else {
                            comma_used = false;
                        }
                        match &element_name {
                            Some(v) if v == "default" => {
                                default_value = $value;
                            }
                            Some(v) if v == "actions" => {
                                actions_value = $value;
                            }
                            _ => {
                                error!(ErrorContext::InvalidSyntaxInBlock(
                                    "option value".to_string()
                                ));
                            }
                        }
                        element_name = None;
                        inserted_count += 1;
                        colon_used = false;
                    };
                }

                last_position = Some(token.position.to_owned());

                match &token.value {
                    Value::LeftParentheses => {
                        error!(ErrorContext::InvalidPosition("(".to_string()));
                    }
                    Value::RightParentheses => {
                        block_closed = true;
                        break 'parse_loop;
                    }
                    Value::Colon => {
                        if colon_used || element_name.is_none() {
                            error!(ErrorContext::InvalidPosition("Colon".to_string()));
                        }
                        colon_used = true;
                    }
                    Value::Comma => {
                        if comma_used || element_name.is_some() {
                            error!(ErrorContext::InvalidPosition("Comma".to_string()));
                        }
                        comma_used = true;
                    }
                    Value::Ident(v) => {
                        if element_name.is_some() {
                            insert!(ParsedValue::Value(v.clone()));
                        } else {
                            element_name = Some(v.to_owned());
                        }
                    }
                    Value::Null => {
                        if element_name.is_some() {
                            insert!(ParsedValue::Null);
                        } else {
                            error!(ErrorContext::Required("Element name".to_string()));
                        }
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
                    Value::LeftSquareBracket => match parse_array(tokens) {
                        Ok(v) => {
                            insert!(v);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    _ => {
                        error!(ErrorContext::InvalidSyntaxInBlock(
                            "option value".to_string()
                        ));
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
                        message: ErrorContext::IsntClosed("option".to_string()),
                        position,
                    });
                }
            }

            Ok((default_value, actions_value))
        }
    }
}
