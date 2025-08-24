use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        config::{Var, Vars},
        token::{Token, Value},
        variable::Value as VarValue,
    },
    parser::{
        error::{
            duplicate_error, template_to_error, ParserError, COLON_POSITION, COLON_REQUIRED,
            COMMA_POSITION, COMMA_REQUIRED, ELEMENT_NAME_REQUIRED, INVALID_ELEMENTS,
            INVALID_LEFT_CURLY_POSITION, INVALID_SYNTAX_IN_VARS, MUST_IN_VARS_BLOCK, VARS_CLOSED,
        },
        parser::Parser,
    },
};

impl Parser {
    pub fn parse_vars<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Vars, ParserError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut colon_used = false;
        let mut comma_used = false;
        let mut element_name = None;
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
                        if vars.get(&$name).is_some() {
                            let err = duplicate_error(&$name);
                            error!(err);
                        }
                        vars.insert($name, $value);
                    };
                }

                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        if in_block {
                            error!(INVALID_LEFT_CURLY_POSITION);
                        }
                        in_block = true;
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
                    Value::Type(t) => {
                        if !vars.is_empty() && !comma_used {
                            error!(COMMA_REQUIRED);
                        }
                        if let Some(name) = element_name {
                            if !colon_used {
                                error!(COLON_REQUIRED);
                            }
                            insert!(
                                name,
                                Var {
                                    v_type: t.clone(),
                                    default_value: VarValue::Null,
                                    actions_value: VarValue::Null
                                }
                            );
                            element_name = None;
                            comma_used = false;
                            colon_used = false;
                        } else {
                            error!(ELEMENT_NAME_REQUIRED);
                        }
                    }
                    Value::Comment(_) => {}
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
                    return Err(ParserError {
                        kind: VARS_CLOSED.kind,
                        code: VARS_CLOSED.code,
                        message: VARS_CLOSED.message.to_string(),
                        position,
                    });
                }
            }

            Ok(vars)
        }
    }
}
