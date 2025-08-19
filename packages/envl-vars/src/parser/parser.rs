use std::collections::HashSet;

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
        let mut comma_used = false;
        let mut in_array = false;
        let mut current_array = Vec::new();
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
                    Value::LeftBracket => {
                        if var.name.is_some() && var.value.is_none() && equal_used && !in_array {
                            in_array = true;
                        } else {
                            parser_error = Some(ParserError {
                                code: ErrorCode::SyntaxError,
                                message: format!("Write arrays after the equal written"),
                                position: position.clone(),
                            });
                            break 'parse_loop;
                        }
                    }
                    Value::RightBracket => {
                        if !in_array {
                            parser_error = Some(ParserError {
                                code: ErrorCode::SyntaxError,
                                message: format!("Use ] only when closing an array"),
                                position: position.clone(),
                            });
                            break 'parse_loop;
                        }

                        match (var.name.clone(), var.value.clone()) {
                            (Some(_), None) if equal_used => {
                                var.value = Some(VariableValue::Array(current_array.clone()));
                                current_array.clear();
                                comma_used = false;
                                in_array = false;
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                    Value::Comma => {
                        if !(in_array && !comma_used && current_array.len() != 0) {
                            parser_error = Some(ParserError {
                                code: ErrorCode::SyntaxError,
                                message: format!("Comma position is invalid"),
                                position: position.clone(),
                            });
                            break 'parse_loop;
                        }
                        comma_used = true;
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
                        if var.name.is_some() && var.value.is_some() {
                            error!(position);
                            break 'parse_loop;
                        }
                        if var.name.is_none() && !equal_used {
                            var = Var {
                                name: Some(value.clone()),
                                value: None,
                            };
                        } else if var.value.is_none() && equal_used {
                            let var_value: VariableValue;
                            if value.starts_with('"') && value.ends_with('"') {
                                let mut str_value = value.clone();
                                str_value.remove(value.len() - 1);
                                str_value.remove(0);
                                var_value = VariableValue::String(str_value);
                            } else if value.starts_with('\'') && value.ends_with('\'') {
                                let mut str_value = value.clone();
                                str_value.remove(value.len() - 1);
                                str_value.remove(0);
                                if let Ok(c) = str_value.parse::<char>() {
                                    var_value = VariableValue::Char(c);
                                } else {
                                    parser_error = Some(ParserError {
                                        code: ErrorCode::MultipleCharacters,
                                        message: "Can't input multiple characters in char"
                                            .to_string(),
                                        position,
                                    });
                                    break 'parse_loop;
                                }
                            } else if is_num(value.clone()) {
                                var_value = VariableValue::Number(value.clone());
                            } else if let Ok(b) = value.parse::<bool>() {
                                var_value = VariableValue::Bool(b);
                            } else {
                                parser_error = Some(ParserError {
                                    code: ErrorCode::InvalidType,
                                    message: "Invalid type".to_string(),
                                    position,
                                });
                                break 'parse_loop;
                            }

                            if in_array {
                                if current_array.len() != 0 && !comma_used {
                                    parser_error = Some(ParserError {
                                        code: ErrorCode::SyntaxError,
                                        message: format!("Comma is required"),
                                        position: position.clone(),
                                    });
                                    break 'parse_loop;
                                }
                                comma_used = false;
                                current_array.push(var_value);
                            } else {
                                var = Var {
                                    name: var.name,
                                    value: Some(var_value),
                                };
                            }
                        } else {
                            error!(position);
                            break 'parse_loop;
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
}
