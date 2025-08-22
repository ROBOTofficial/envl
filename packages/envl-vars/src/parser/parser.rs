use std::{
    collections::{HashMap, HashSet},
    slice::Iter,
};

use crate::{
    misc::{
        num::is_num,
        position::Position,
        token::{Token, Value},
        variable::{Variable, VariableValue},
    },
    parser::error::{
        duplicate_error, ErrorKind, ARRAY_AFTER_EQUAL, ARRAY_CLOSED, ARRAY_INVALID_CLOSE,
        COLON_POSITION, COLON_REQUIRED, COMMA_POSITION, COMMA_REQUIRED, DIFFERENT_ORDER,
        INVALID_ARRAY_POSITION, INVALID_TYPE, ITEM_NAME_NOT_SET, MULTIPLE_CHAR, STRUCT_AFTER_EQUAL,
        STRUCT_CLOSED, STRUCT_INVALID_CLOSE, SYNTAX_IN_ARRAY, SYNTAX_IN_STRUCT,
    },
};

#[derive(Debug)]
pub struct ParserError {
    pub code: u32,
    pub kind: ErrorKind,
    pub message: String,
    pub position: Position,
}

#[derive(Debug, Clone)]
enum ParsedIdent {
    Name(String),
    Value(VariableValue),
}

struct Var {
    pub name: Option<String>,
    pub value: Option<VariableValue>,
}

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Vec<Variable>, ParserError> {
        let mut tokens = self.tokens.iter();
        let mut vars = Vec::new();
        let mut equal_used = false;
        let mut var = Var {
            name: None,
            value: None,
        };
        let mut parser_error: Option<ParserError> = None;

        macro_rules! clear {
            () => {{
                var = Var {
                    name: None,
                    value: None,
                };
                equal_used = false;
            }};
        }

        macro_rules! error {
            ($pos: ident) => {
                parser_error = Some(ParserError {
                    kind: DIFFERENT_ORDER.kind,
                    code: DIFFERENT_ORDER.code,
                    message: DIFFERENT_ORDER.message.to_string(),
                    position: $pos,
                })
            };
        }

        'parse_loop: loop {
            if let Some(token) = tokens.next() {
                let value = &token.value;
                let position = token.position.clone();
                match value {
                    Value::LeftSquareBracket => match self.parse_array(&mut tokens) {
                        Ok(v) => {
                            if var.name.is_some() && var.value.is_none() && equal_used {
                                var = Var {
                                    name: var.name,
                                    value: Some(v.clone()),
                                }
                            } else {
                                parser_error = Some(ParserError {
                                    kind: ARRAY_AFTER_EQUAL.kind,
                                    code: ARRAY_AFTER_EQUAL.code,
                                    message: ARRAY_AFTER_EQUAL.message.to_string(),
                                    position: position.clone(),
                                });
                                break 'parse_loop;
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::RightSquareBracket => {
                        parser_error = Some(ParserError {
                            kind: ARRAY_INVALID_CLOSE.kind,
                            code: ARRAY_INVALID_CLOSE.code,
                            message: ARRAY_INVALID_CLOSE.message.to_string(),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::LeftCurlyBracket => match self.parse_struct(&mut tokens) {
                        Ok(v) => {
                            if var.name.is_some() && var.value.is_none() && equal_used {
                                var = Var {
                                    name: var.name,
                                    value: Some(v.clone()),
                                }
                            } else {
                                parser_error = Some(ParserError {
                                    kind: STRUCT_AFTER_EQUAL.kind,
                                    code: STRUCT_AFTER_EQUAL.code,
                                    message: STRUCT_AFTER_EQUAL.message.to_string(),
                                    position: position.clone(),
                                });
                                break 'parse_loop;
                            }
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_loop;
                        }
                    },
                    Value::RightCurlyBracket => {
                        parser_error = Some(ParserError {
                            kind: STRUCT_INVALID_CLOSE.kind,
                            code: STRUCT_INVALID_CLOSE.code,
                            message: STRUCT_INVALID_CLOSE.message.to_string(),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Colon => {
                        parser_error = Some(ParserError {
                            kind: COLON_POSITION.kind,
                            code: COLON_POSITION.code,
                            message: COLON_POSITION.message.to_string(),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Comma => {
                        parser_error = Some(ParserError {
                            kind: COMMA_POSITION.kind,
                            code: COMMA_POSITION.code,
                            message: COMMA_POSITION.message.to_string(),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Equal => {
                        if equal_used {
                            error!(position);
                            break 'parse_loop;
                        }
                        match (&var.name, &var.value) {
                            (Some(_), None) => {
                                equal_used = true;
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Semi => {
                        if !equal_used {
                            error!(position);
                            break 'parse_loop;
                        }
                        match (&var.name, &var.value) {
                            (Some(name), Some(value)) => {
                                vars.push(Variable {
                                    name: name.clone(),
                                    value: value.clone(),
                                    position: position.clone(),
                                });
                                clear!();
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Ident(value) => {
                        match self.parse_ident(value.clone(), &var, &position, &equal_used) {
                            Ok(ident) => match ident {
                                ParsedIdent::Name(name) => {
                                    var = Var {
                                        name: Some(name.clone()),
                                        value: None,
                                    };
                                }
                                ParsedIdent::Value(value) => {
                                    var = Var {
                                        name: var.name,
                                        value: Some(value.clone()),
                                    };
                                }
                            },
                            Err(e) => {
                                parser_error = Some(e);
                                break 'parse_loop;
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                break 'parse_loop;
            }
        }

        if let Some(err) = parser_error {
            return Err(err);
        }

        if let Some(err) = self.duplicate_check(&vars) {
            return Err(err);
        }

        Ok(vars)
    }

    fn duplicate_check(&self, vars: &Vec<Variable>) -> Option<ParserError> {
        let mut hs = HashSet::new();

        for var in vars {
            if !hs.insert(var.name.clone()) {
                let err = duplicate_error(&var.name);
                return Some(ParserError {
                    kind: err.kind,
                    code: err.code,
                    message: err.message.to_string(),
                    position: var.position.clone(),
                });
            }
        }

        None
    }

    fn parse_struct<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<VariableValue, ParserError> {
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
                                if hm.len() != 0 && !comma_used {
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
                                if hm.len() != 0 && !comma_used {
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
                            if hm.len() != 0 && !comma_used {
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

    fn parse_array<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<VariableValue, ParserError> {
        let mut array_contents = Vec::new();
        let mut parser_error: Option<ParserError> = None;
        let mut comma_used = false;
        let mut array_closed = false;
        let mut last_position = None;

        'parse_array_loop: loop {
            if let Some(token) = tokens.next() {
                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftSquareBracket => match self.parse_array(tokens) {
                        Ok(v) => {
                            if array_contents.len() != 0 && !comma_used {
                                parser_error = Some(ParserError {
                                    kind: COMMA_REQUIRED.kind,
                                    code: COMMA_REQUIRED.code,
                                    message: COMMA_REQUIRED.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(v.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::RightSquareBracket => {
                        array_closed = true;
                        break 'parse_array_loop;
                    }
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(ParserError {
                                kind: COMMA_POSITION.kind,
                                code: COMMA_POSITION.code,
                                message: COMMA_POSITION.message.to_string(),
                                position: token.position.clone(),
                            });
                            break 'parse_array_loop;
                        }
                        comma_used = true;
                    }
                    Value::LeftCurlyBracket => match self.parse_struct(tokens) {
                        Ok(value) => {
                            if array_contents.len() != 0 && !comma_used {
                                parser_error = Some(ParserError {
                                    kind: COMMA_REQUIRED.kind,
                                    code: COMMA_REQUIRED.code,
                                    message: COMMA_REQUIRED.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(value.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::Ident(value) => {
                        let value = self.parse_value(&value, &token.position);
                        match value {
                            Ok(v) => {
                                if array_contents.len() != 0 && !comma_used {
                                    parser_error = Some(ParserError {
                                        kind: COMMA_REQUIRED.kind,
                                        code: COMMA_REQUIRED.code,
                                        message: COMMA_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_array_loop;
                                }
                                array_contents.push(v.clone());
                                comma_used = false;
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_array_loop;
                            }
                        }
                    }
                    Value::Comment(_) => {}
                    _ => {
                        parser_error = Some(ParserError {
                            kind: SYNTAX_IN_ARRAY.kind,
                            code: SYNTAX_IN_ARRAY.code,
                            message: SYNTAX_IN_ARRAY.message.to_string(),
                            position: token.position.clone(),
                        });
                        break 'parse_array_loop;
                    }
                }
            } else {
                break 'parse_array_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !array_closed {
                    return Err(ParserError {
                        kind: ARRAY_CLOSED.kind,
                        code: ARRAY_CLOSED.code,
                        message: ARRAY_CLOSED.message.to_string(),
                        position,
                    });
                }
            }
            Ok(VariableValue::Array(array_contents))
        }
    }

    fn parse_ident(
        &self,
        value: String,
        var: &Var,
        position: &Position,
        equal_used: &bool,
    ) -> Result<ParsedIdent, ParserError> {
        if var.name.is_some() && var.value.is_some() {
            return Err(ParserError {
                kind: DIFFERENT_ORDER.kind,
                code: DIFFERENT_ORDER.code,
                message: DIFFERENT_ORDER.message.to_string(),
                position: position.clone(),
            });
        }
        if var.name.is_none() && !equal_used {
            Ok(ParsedIdent::Name(value.clone()))
        } else if var.value.is_none() && *equal_used {
            let var_value = self.parse_value(&value, &position);
            match var_value {
                Ok(var_value) => Ok(ParsedIdent::Value(var_value)),
                Err(err) => Err(err),
            }
        } else {
            Err(ParserError {
                kind: DIFFERENT_ORDER.kind,
                code: DIFFERENT_ORDER.code,
                message: DIFFERENT_ORDER.message.to_string(),
                position: position.clone(),
            })
        }
    }

    fn parse_value(
        &self,
        value: &String,
        position: &Position,
    ) -> Result<VariableValue, ParserError> {
        if value.starts_with('"') && value.ends_with('"') {
            let mut str_value = value.clone();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            Ok(VariableValue::String(str_value))
        } else if value.starts_with('\'') && value.ends_with('\'') {
            let mut str_value = value.clone();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            if let Ok(c) = str_value.parse::<char>() {
                Ok(VariableValue::Char(c))
            } else {
                Err(ParserError {
                    kind: MULTIPLE_CHAR.kind,
                    code: MULTIPLE_CHAR.code,
                    message: MULTIPLE_CHAR.message.to_string(),
                    position: position.clone(),
                })
            }
        } else if is_num(value.clone()) {
            Ok(VariableValue::Number(value.clone()))
        } else if let Ok(b) = value.parse::<bool>() {
            Ok(VariableValue::Bool(b))
        } else {
            Err(ParserError {
                kind: INVALID_TYPE.kind,
                code: INVALID_TYPE.code,
                message: INVALID_TYPE.message.to_string(),
                position: position.clone(),
            })
        }
    }
}
