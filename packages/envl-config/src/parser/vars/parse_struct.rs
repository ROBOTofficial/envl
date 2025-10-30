use std::{collections::HashMap, slice::Iter};

use envl_utils::{
    error::{EnvlError, ErrorContext},
    name::is_valid_variable_name,
};

use crate::{
    misc::{
        token::{Token, Value},
        variable::Type,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_struct<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<Type, EnvlError> {
        let mut in_block = false;
        let mut block_closed = false;
        let mut colon_used = false;
        let mut last_position = None;
        let mut target_prop: Option<String> = None;
        let mut target_value = None;
        let mut elements = HashMap::new();

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
                    ($name: expr, $value: expr) => {
                        if !colon_used {
                            error!(ErrorContext::Required("Colon".to_string()));
                        }
                        if elements.get(&$name).is_some() {
                            error!(ErrorContext::Duplicate($name.to_string()));
                        }
                        elements.insert($name, $value);
                        target_prop = None;
                        target_value = None;
                        colon_used = false;
                    };
                }
                macro_rules! insert_target_value {
                    ($value: expr) => {
                        if target_prop.is_some() {
                            if !colon_used {
                                error!(ErrorContext::Required("Colon".to_string()));
                            }
                            if target_value.is_some() {
                                error!(ErrorContext::InvalidSyntaxInBlock("struct".to_string()));
                            }
                            target_value = Some($value.to_owned());
                        } else {
                            error!(ErrorContext::Required("Element name".to_string()));
                        }
                    };
                }

                last_position = Some(token.position.to_owned());

                match &token.value {
                    Value::LeftCurlyBracket => {
                        if in_block {
                            error!(ErrorContext::InvalidPosition("{".to_string()));
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
                    error!(ErrorContext::MustInBlock("vars".to_string()));
                }

                match &token.value {
                    Value::Colon => {
                        if colon_used || target_prop.is_none() {
                            error!(ErrorContext::Required("Colon".to_string()));
                        }
                        colon_used = true;
                    }
                    Value::Semi => match (target_prop, target_value) {
                        (Some(name), Some(value)) => {
                            insert!(name, value);
                        }
                        _ => {
                            error!(ErrorContext::InvalidSyntaxInBlock("struct".to_string()));
                        }
                    },
                    Value::Ident(v) => {
                        if target_prop.is_some() {
                            error!(ErrorContext::InvalidElements);
                        }
                        if !is_valid_variable_name(v) {
                            error!(ErrorContext::InvalidName(v.to_string()));
                        }
                        target_prop = Some(v.to_owned());
                    }
                    Value::Type(t) => {
                        insert_target_value!(t);
                    }
                    Value::Option => match self.parse_option(tokens) {
                        Ok(v) => {
                            insert_target_value!(v);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::Array => match self.parse_array(tokens) {
                        Ok(v) => {
                            insert_target_value!(v);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::Struct => match self.parse_struct(tokens) {
                        Ok(t) => {
                            insert_target_value!(t);
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
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
                        message: ErrorContext::IsntClosed("struct".to_string()),
                        position,
                    });
                }
            }

            Ok(Type::Struct(elements))
        }
    }
}
