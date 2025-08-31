use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        config::{Var, Vars},
        token::{Token, Value},
        variable::{Type, Value as VarValue},
    },
    parser::{
        error::{
            duplicate_error, template_to_error, ParserError, COLON_POSITION, COLON_REQUIRED,
            COMMA_POSITION, COMMA_REQUIRED, ELEMENT_NAME_REQUIRED, INVALID_ELEMENTS,
            INVALID_LEFT_CURLY_POSITION, INVALID_SYNTAX, INVALID_SYNTAX_IN_VARS,
            MUST_IN_VARS_BLOCK, VARS_CLOSED,
        },
        vars::option_value::parse_parsed_value,
        Parser,
    },
};

pub mod array;
pub mod option_value;
pub mod parse_struct;

impl Parser {
    pub fn parse_vars<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Vars, ParserError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut colon_used = false;
        let mut comma_used = false;
        let mut element_name = None;
        let mut inserted_element_name = None;
        let mut last_position = None;

        let mut parser_error = None;
        let mut vars = HashMap::new();

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
                        if !vars.is_empty() && !comma_used {
                            error!(COMMA_REQUIRED);
                        }
                        if !colon_used {
                            error!(COLON_REQUIRED);
                        }
                        if vars.get(&$name).is_some() {
                            let err = duplicate_error(&$name);
                            error!(err);
                        }
                        vars.insert($name.clone(), $value);
                        element_name = None;
                        comma_used = false;
                        colon_used = false;
                        inserted_element_name = Some($name);
                    };
                }

                last_position = Some(token.position.clone());

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
                    Value::Comma => {
                        if comma_used {
                            error!(COMMA_POSITION);
                        }
                        comma_used = true;
                    }
                    Value::Colon => {
                        if colon_used {
                            error!(COLON_POSITION);
                        }
                        colon_used = true;
                    }
                    Value::Ident(v) => {
                        if element_name.is_some() {
                            error!(INVALID_ELEMENTS);
                        }
                        element_name = Some(v.clone());
                    }
                    Value::Null => {
                        if let Some(name) = element_name {
                            insert!(
                                name,
                                Var {
                                    v_type: Type::Null,
                                    default_value: VarValue::Null,
                                    actions_value: VarValue::Null,
                                    position: token.position.to_owned()
                                }
                            );
                        } else {
                            error!(ELEMENT_NAME_REQUIRED);
                        }
                    }
                    Value::Array => match self.parse_array(tokens) {
                        Ok(t) => {
                            if let Some(name) = element_name {
                                insert!(
                                    name,
                                    Var {
                                        v_type: t.clone(),
                                        default_value: VarValue::Null,
                                        actions_value: VarValue::Null,
                                        position: token.position.to_owned()
                                    }
                                );
                            } else {
                                error!(ELEMENT_NAME_REQUIRED);
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::Struct => match self.parse_struct(tokens) {
                        Ok(t) => {
                            if let Some(name) = element_name {
                                insert!(
                                    name,
                                    Var {
                                        v_type: t.clone(),
                                        default_value: VarValue::Null,
                                        actions_value: VarValue::Null,
                                        position: token.position.to_owned()
                                    }
                                );
                            } else {
                                error!(ELEMENT_NAME_REQUIRED);
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::Type(t) => {
                        if let Some(name) = element_name {
                            insert!(
                                name,
                                Var {
                                    v_type: t.clone(),
                                    default_value: VarValue::Null,
                                    actions_value: VarValue::Null,
                                    position: token.position.to_owned()
                                }
                            );
                        } else {
                            error!(ELEMENT_NAME_REQUIRED);
                        }
                    }
                    Value::LeftParentheses => match self.parse_option_value(tokens) {
                        Ok((default_value, actions_value)) => {
                            if let Some(ref name) = inserted_element_name {
                                if let Some(var) = vars.get(name) {
                                    match parse_parsed_value(
                                        default_value,
                                        var.v_type.clone(),
                                        token.position.clone(),
                                    ) {
                                        Ok(parsed_default_value) => {
                                            match parse_parsed_value(
                                                actions_value,
                                                var.v_type.clone(),
                                                token.position.clone(),
                                            ) {
                                                Ok(parsed_actions_value) => {
                                                    vars.insert(
                                                        name.clone(),
                                                        Var {
                                                            v_type: var.v_type.clone(),
                                                            default_value: parsed_default_value,
                                                            actions_value: parsed_actions_value,
                                                            position: var.position.clone(),
                                                        },
                                                    );
                                                }
                                                Err(err) => {
                                                    parser_error = Some(err);
                                                    break 'parse_loop;
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            parser_error = Some(err);
                                            break 'parse_loop;
                                        }
                                    }
                                } else {
                                    error!(INVALID_SYNTAX);
                                }
                            } else {
                                error!(ELEMENT_NAME_REQUIRED);
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    _ => {
                        error!(INVALID_SYNTAX_IN_VARS);
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
                    return Err(template_to_error(VARS_CLOSED, position));
                }
            }

            Ok(vars)
        }
    }
}
