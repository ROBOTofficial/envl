use std::collections::HashSet;

use envl_utils::error::{EnvlError, ErrorContext};

use crate::misc::{
    token::{Token, Value},
    variable::{Variable, VariableValue},
};

pub mod array;
pub mod ident;
pub mod parse_struct;
pub mod test;
pub mod value;

#[derive(Debug, Clone)]
pub enum ParsedIdent {
    Name(String),
    Value(VariableValue),
}

pub struct Var {
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

    pub fn parse(&self) -> Result<Vec<Variable>, EnvlError> {
        let mut based_token = vec![];

        for token in self.tokens.iter() {
            match token.value {
                Value::Comment(_) => {
                    continue;
                }
                _ => {
                    based_token.push(token.clone());
                }
            }
        }

        let mut tokens = based_token.iter();

        let mut vars = Vec::new();
        let mut equal_used = false;
        let mut var = Var {
            name: None,
            value: None,
        };
        let mut parser_error: Option<EnvlError> = None;

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
                parser_error = Some(EnvlError {
                    message: ErrorContext::InvalidSyntax,
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
                                parser_error = Some(EnvlError {
                                    message: ErrorContext::AfterEqual("array".to_string()),
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
                        parser_error = Some(EnvlError {
                            message: ErrorContext::InvalidSyntax,
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Struct => match self.parse_struct(&mut tokens) {
                        Ok(v) => {
                            if var.name.is_some() && var.value.is_none() && equal_used {
                                var = Var {
                                    name: var.name,
                                    value: Some(v.clone()),
                                }
                            } else {
                                parser_error = Some(EnvlError {
                                    message: ErrorContext::AfterEqual("struct".to_string()),
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
                        parser_error = Some(EnvlError {
                            message: ErrorContext::InvalidSyntax,
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Colon => {
                        parser_error = Some(EnvlError {
                            message: ErrorContext::InvalidPosition("Colon".to_string()),
                            position: position.clone(),
                        });
                        break 'parse_loop;
                    }
                    Value::Comma => {
                        parser_error = Some(EnvlError {
                            message: ErrorContext::InvalidPosition("Comma".to_string()),
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
                    _ => {
                        parser_error = Some(EnvlError {
                            message: ErrorContext::InvalidSyntax,
                            position: token.position.clone(),
                        });
                        break 'parse_loop;
                    }
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

    fn duplicate_check(&self, vars: &Vec<Variable>) -> Option<EnvlError> {
        let mut hs = HashSet::new();

        for var in vars {
            if !hs.insert(&var.name) {
                return Some(EnvlError {
                    message: ErrorContext::Duplicate(var.name.clone()),
                    position: var.position.clone(),
                });
            }
        }

        None
    }
}
