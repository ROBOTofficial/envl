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

        'parse_loop: for token in self.tokens.iter() {
            let value = &token.value;
            let position = token.position.clone();
            match value {
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
                    if var.name.is_none() {
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
                                    message: "Can't input multiple characters in char".to_string(),
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
                        var = Var {
                            name: var.name,
                            value: Some(var_value),
                        };
                    } else {
                        error!(position);
                        break 'parse_loop;
                    }
                }
                _ => {}
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

    pub fn duplicate_check(&self, vars: &Vec<Variable>) -> Option<ParserError> {
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

#[cfg(test)]
mod test {
    use crate::{
        lexer::lexer::Lexer,
        misc::variable::{Variable, VariableValue, VariableWithoutPosition},
        parser::{
            error::ErrorCode,
            parser::{Parser, ParserError},
        },
    };

    fn gen_parsed_vars(code: String) -> Result<Vec<Variable>, ParserError> {
        let lex = Lexer::new("test.envl".to_string(), code);
        let tokens = lex.generate();
        let parser = Parser::new(tokens);
        parser.parse()
    }

    fn gen_vars(code: String) -> Vec<VariableWithoutPosition> {
        gen_parsed_vars(code)
            .unwrap()
            .iter()
            .map(|v| VariableWithoutPosition {
                name: v.name.clone(),
                value: v.value.clone(),
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn number_test() {
        let result = gen_vars("variable = 12345;".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Number("12345".to_string())
            }]
        );
    }

    #[test]
    fn string_test() {
        let result = gen_vars("variable = \"12345\";".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::String("12345".to_string())
            }]
        );
    }

    #[test]
    fn char_test() {
        let result = gen_vars("variable = 'a';".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Char('a')
            }]
        );
    }

    #[test]
    fn bool_test() {
        let result = gen_vars("variable = true; variable2 = false;".to_string());
        assert_eq!(
            result,
            vec![
                VariableWithoutPosition {
                    name: "variable".to_string(),
                    value: VariableValue::Bool(true)
                },
                VariableWithoutPosition {
                    name: "variable2".to_string(),
                    value: VariableValue::Bool(false)
                }
            ]
        );
    }

    #[test]
    fn comment_test() {
        let result = gen_vars("variable = 12345; //this is a comment".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Number("12345".to_string())
            }]
        );
    }

    #[test]
    fn syntax_error_test() {
        let result = gen_parsed_vars("variable = \"aiueo';".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.code, ErrorCode::InvalidType);
        }
    }

    #[test]
    fn duplicate_error_test() {
        let result = gen_parsed_vars("variable = 12345; variable = \"12345\";".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.code, ErrorCode::DuplicateVars);
        }
    }

    #[test]
    fn invalid_type_error_test() {
        let result = gen_parsed_vars("variable = aiueo;".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.code, ErrorCode::InvalidType);
        }
    }

    #[test]
    fn multiple_char_error() {
        let result = gen_parsed_vars("variable = 'char';".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.code, ErrorCode::MultipleCharacters);
        }
    }
}
