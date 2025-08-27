use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        position::Position,
        token::{Token, Value},
        variable::{Type, Value as ConfigValue},
    },
    parser::{
        error::{
            template_to_error, ParserError, COLON_POSITION, COLON_REQUIRED, COMMA_POSITION,
            ELEMENT_NAME_REQUIRED, INVALID_ELEMENTS, INVALID_LEFT_PARENTHESES, INVALID_SYNTAX,
            INVALID_TYPE, OPTION_CLOSED,
        },
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
) -> Result<ConfigValue, ParserError> {
    match v {
        ParsedValue::Null => Ok(ConfigValue::Null),
        ParsedValue::Value(value) => match parse_value(t, value) {
            Ok(result) => Ok(result),
            Err(err) => Err(template_to_error(err, position)),
        },
        ParsedValue::Struct(values) => match t {
            Type::Struct(t) => {
                let mut elements = HashMap::new();

                for (name, value) in values {
                    if let Some(value_type) = t.get(&name) {
                        match parse_parsed_value(value, value_type.clone(), position.clone()) {
                            Ok(result) => {
                                if elements.contains_key(&name) {
                                    return Err(template_to_error(INVALID_ELEMENTS, position));
                                }
                                elements.insert(name.clone(), result);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return Err(template_to_error(INVALID_TYPE, position));
                    }
                }

                Ok(ConfigValue::Struct(elements))
            }
            _ => Err(template_to_error(INVALID_TYPE, position)),
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
            _ => Err(template_to_error(INVALID_TYPE, position)),
        },
    }
}

impl Parser {
    pub fn parse_option<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<(ParsedValue, ParsedValue), ParserError> {
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
                    ($err: expr) => {
                        parser_error = Some(template_to_error($err, token.position.clone()));
                        break 'parse_loop;
                    };
                }
                macro_rules! insert {
                    ($value: expr) => {
                        if !colon_used {
                            error!(COLON_REQUIRED);
                        }
                        if !comma_used && inserted_count != 0 {
                            error!(COMMA_POSITION);
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
                                error!(INVALID_SYNTAX);
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
                        error!(INVALID_LEFT_PARENTHESES);
                    }
                    Value::RightParentheses => {
                        block_closed = true;
                        break 'parse_loop;
                    }
                    Value::Colon => {
                        if colon_used || element_name.is_none() {
                            error!(COLON_POSITION);
                        }
                        colon_used = true;
                    }
                    Value::Comma => {
                        if comma_used || element_name.is_some() {
                            error!(COMMA_POSITION);
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
                            error!(ELEMENT_NAME_REQUIRED);
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

            Ok((default_value, actions_value))
        }
    }
}
