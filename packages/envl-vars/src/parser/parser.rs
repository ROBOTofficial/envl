use std::{collections::HashSet, slice::Iter};

use crate::{
    envl_vars_error_message,
    misc::{
        num::is_num,
        position::Position,
        token::{Token, Value},
        variable::{Variable, VariableValue},
    },
    parser::error::ErrorCode,
};

#[derive(Debug)]
pub struct ParserError {
    pub code: ErrorCode,
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
                let message = envl_vars_error_message!(
                    "The order must be variable name, equal sign, value, and semicolon.",
                    $pos
                );
                parser_error = Some(ParserError {
                    code: ErrorCode::SyntaxError,
                    message,
                    position: $pos,
                })
            };
        }

        'parse_loop: loop {
            if let Some(token) = tokens.next() {
                let value = &token.value;
                let position = token.position.clone();
                match value {
                    Value::LeftBracket => match self.parse_array(&mut tokens) {
                        Ok(v) => {
                            if var.name.is_some() && var.value.is_none() && equal_used {
                                var = Var {
                                    name: var.name,
                                    value: Some(v.clone()),
                                }
                            } else {
                                parser_error = Some(ParserError {
                                    code: ErrorCode::SyntaxError,
                                    message: format!("Write arrays after the equal written"),
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
                    Value::RightBracket => {
                        parser_error = Some(ParserError {
                            code: ErrorCode::SyntaxError,
                            message: format!("Use ] only when closing an array"),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Comma => {
                        parser_error = Some(ParserError {
                            code: ErrorCode::SyntaxError,
                            message: format!("Comma position is invalid"),
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
                let message = format!("{} is duplicated", &var.name);
                return Some(ParserError {
                    code: ErrorCode::DuplicateVars,
                    message,
                    position: var.position.clone(),
                });
            }
        }

        None
    }

    fn parse_array<'a>(&self, tokens: &mut Iter<'a, Token>) -> Result<VariableValue, ParserError> {
        let mut array_contents = Vec::new();
        let mut parser_error: Option<ParserError> = None;
        let mut comma_used = false;

        'parse_array_loop: loop {
            if let Some(token) = tokens.next() {
                match &token.value {
                    Value::LeftBracket => match self.parse_array(tokens) {
                        Ok(v) => {
                            if array_contents.len() != 0 && !comma_used {
                                parser_error = Some(ParserError {
                                    code: ErrorCode::SyntaxError,
                                    message: format!("Comma is required"),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(v.clone());
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::RightBracket => {
                        break 'parse_array_loop;
                    }
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(ParserError {
                                code: ErrorCode::SyntaxError,
                                message: format!("Comma position is invalid"),
                                position: token.position.clone(),
                            });
                            break 'parse_array_loop;
                        }
                        comma_used = true;
                    }
                    Value::Ident(value) => {
                        let value = self.parse_value(&value, &token.position);
                        match value {
                            Ok(v) => {
                                if array_contents.len() != 0 && !comma_used {
                                    parser_error = Some(ParserError {
                                        code: ErrorCode::SyntaxError,
                                        message: format!("Comma is required"),
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
                    _ => {}
                }
            } else {
                break 'parse_array_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
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
                code: ErrorCode::SyntaxError,
                message: envl_vars_error_message!(
                    "The order must be variable name, equal sign, value, and semicolon.",
                    position
                ),
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
                code: ErrorCode::SyntaxError,
                message: envl_vars_error_message!(
                    "The order must be variable name, equal sign, value, and semicolon.",
                    position
                ),
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
                    code: ErrorCode::MultipleCharacters,
                    message: "Can't input multiple characters in char".to_string(),
                    position: position.clone(),
                })
            }
        } else if is_num(value.clone()) {
            Ok(VariableValue::Number(value.clone()))
        } else if let Ok(b) = value.parse::<bool>() {
            Ok(VariableValue::Bool(b))
        } else {
            Err(ParserError {
                code: ErrorCode::InvalidType,
                message: "Invalid type".to_string(),
                position: position.clone(),
            })
        }
    }
}
