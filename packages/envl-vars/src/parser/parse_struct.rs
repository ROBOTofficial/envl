use std::{collections::HashMap, slice::Iter};

use crate::{
    misc::{
        token::{Token, Value},
        variable::VariableValue,
    },
    parser::{
        error::{
            duplicate_error, COLON_POSITION, COLON_REQUIRED, COMMA_POSITION, COMMA_REQUIRED,
            INVALID_ARRAY_POSITION, ITEM_NAME_NOT_SET, STRUCT_CLOSED, SYNTAX_IN_STRUCT,
        },
        Parser, ParserError,
    },
};

impl Parser {
    pub fn parse_struct<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<VariableValue, ParserError> {
        let mut hm = HashMap::new();
        let mut parser_error = None;
        let mut comma_used = false;
        let mut colon_used = false;
        let mut struct_closed = false;
        let mut last_position = None;
        let mut element_name = None;

        macro_rules! clean {
            () => {
                comma_used = false;
                colon_used = false;
                element_name = None;
            };
        }

        'parse_struct_loop: loop {
            if let Some(token) = tokens.next() {
                macro_rules! insert {
                    ($name: expr, $value: expr) => {
                        if hm.get(&$name).is_some() {
                            let err = duplicate_error(&$name);
                            parser_error = Some(ParserError {
                                kind: err.kind,
                                code: err.code,
                                message: err.message.to_string(),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        hm.insert($name, $value);
                    };
                }

                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftCurlyBracket => match self.parse_struct(tokens) {
                        Ok(value) => match element_name {
                            Some(name) => {
                                if !colon_used {
                                    parser_error = Some(ParserError {
                                        kind: COLON_REQUIRED.kind,
                                        code: COLON_REQUIRED.code,
                                        message: COLON_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                if !hm.is_empty() && !comma_used {
                                    parser_error = Some(ParserError {
                                        kind: COMMA_REQUIRED.kind,
                                        code: COMMA_REQUIRED.code,
                                        message: COMMA_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                insert!(name, value);
                                clean!();
                            }
                            None => {
                                parser_error = Some(ParserError {
                                    kind: ITEM_NAME_NOT_SET.kind,
                                    code: ITEM_NAME_NOT_SET.code,
                                    message: ITEM_NAME_NOT_SET.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                        },
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_struct_loop;
                        }
                    },
                    Value::RightCurlyBracket => {
                        struct_closed = true;
                        break 'parse_struct_loop;
                    }
                    Value::LeftSquareBracket => match self.parse_array(tokens) {
                        Ok(value) => {
                            if let Some(name) = element_name {
                                if !colon_used {
                                    parser_error = Some(ParserError {
                                        kind: COLON_REQUIRED.kind,
                                        code: COLON_REQUIRED.code,
                                        message: COLON_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                if !hm.is_empty() && !comma_used {
                                    parser_error = Some(ParserError {
                                        kind: COMMA_REQUIRED.kind,
                                        code: COMMA_REQUIRED.code,
                                        message: COMMA_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_struct_loop;
                                }
                                insert!(name, value);
                                clean!();
                            } else {
                                parser_error = Some(ParserError {
                                    kind: INVALID_ARRAY_POSITION.kind,
                                    code: INVALID_ARRAY_POSITION.code,
                                    message: INVALID_ARRAY_POSITION.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_struct_loop;
                        }
                    },
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(ParserError {
                                kind: COMMA_POSITION.kind,
                                code: COMMA_POSITION.code,
                                message: COMMA_POSITION.message.to_string(),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        comma_used = true;
                    }
                    Value::Colon => {
                        if colon_used {
                            parser_error = Some(ParserError {
                                kind: COLON_POSITION.kind,
                                code: COLON_POSITION.code,
                                message: COLON_POSITION.message.to_string(),
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                        colon_used = true;
                    }
                    Value::Ident(v) => match element_name.clone() {
                        None => {
                            element_name = Some(v.clone());
                        }
                        Some(name) if colon_used => {
                            if !hm.is_empty() && !comma_used {
                                parser_error = Some(ParserError {
                                    kind: COMMA_REQUIRED.kind,
                                    code: COMMA_REQUIRED.code,
                                    message: COMMA_REQUIRED.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_struct_loop;
                            }
                            match self.parse_value(v, &token.position.clone()) {
                                Ok(value) => {
                                    insert!(name, value);
                                    clean!();
                                }
                                Err(err) => {
                                    parser_error = Some(err);
                                    break 'parse_struct_loop;
                                }
                            }
                        }
                        _ => {
                            let (code, kind, message) = if !colon_used {
                                (
                                    COLON_REQUIRED.code,
                                    COLON_REQUIRED.kind,
                                    COLON_REQUIRED.message.to_string(),
                                )
                            } else {
                                (
                                    ITEM_NAME_NOT_SET.code,
                                    ITEM_NAME_NOT_SET.kind,
                                    ITEM_NAME_NOT_SET.message.to_string(),
                                )
                            };
                            parser_error = Some(ParserError {
                                code,
                                kind,
                                message,
                                position: token.position.clone(),
                            });
                            break 'parse_struct_loop;
                        }
                    },
                    Value::Comment(_) => {}
                    _ => {
                        parser_error = Some(ParserError {
                            kind: SYNTAX_IN_STRUCT.kind,
                            code: SYNTAX_IN_STRUCT.code,
                            message: SYNTAX_IN_STRUCT.message.to_string(),
                            position: token.position.clone(),
                        });
                        break 'parse_struct_loop;
                    }
                }
            } else {
                break 'parse_struct_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !struct_closed {
                    return Err(ParserError {
                        kind: STRUCT_CLOSED.kind,
                        code: STRUCT_CLOSED.code,
                        message: STRUCT_CLOSED.message.to_string(),
                        position,
                    });
                }
            }
            Ok(VariableValue::Struct(hm))
        }
    }
}
