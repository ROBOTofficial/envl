use crate::{
    envl_vars_error_message,
    misc::{
        position::Position,
        token::{Token, Value},
        variable::{Variable, VariableValue},
    },
};

#[derive(Debug)]
pub struct ParserError {
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
                parser_error = Some(ParserError { message, $pos })
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
                Value::VariableName(name) => {
                    if equal_used {
                        error!(position);
                        break 'parse_loop;
                    }
                    match (&var.name, &var.value) {
                        (None, None) => {
                            var.name = Some(name.clone());
                        }
                        _ => {
                            error!(position);
                            break 'parse_loop;
                        }
                    }
                }
                Value::VariableValue(value) => match value {
                    VariableValue::String(value) => {
                        if !equal_used {
                            error!(position);
                            break 'parse_loop;
                        }
                        match (&var.name, &var.value) {
                            (Some(_), None) => {
                                var.value = Some(VariableValue::String(value.clone()));
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                    VariableValue::Number(value) => {
                        if !equal_used {
                            error!(position);
                            break 'parse_loop;
                        }
                        match (&var.name, &var.value) {
                            (Some(_), None) => {
                                var.value = Some(VariableValue::Number(value.clone()));
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                    VariableValue::Bool(value) => {
                        if !equal_used {
                            error!(position);
                            break 'parse_loop;
                        }
                        match (&var.name, &var.value) {
                            (Some(_), None) => {
                                var.value = Some(VariableValue::Bool(value.clone()));
                            }
                            _ => {
                                error!(position);
                                break 'parse_loop;
                            }
                        }
                    }
                },
                _ => {}
            }
        }

        if let Some(err) = parser_error {
            return Err(err);
        }

        Ok(vars)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::lexer::Lexer,
        misc::variable::{VariableValue, VariableWithoutPosition},
        parser::parser::Parser,
    };

    #[test]
    pub fn parse_test() {
        let lex = Lexer::new("test.envl".to_string(), "variable = 12345;".to_string());
        let tokens = lex.generate();
        let parser = Parser::new(tokens);
        let result = parser
            .parse()
            .unwrap()
            .iter()
            .map(|v| VariableWithoutPosition {
                name: v.name.clone(),
                value: v.value.clone(),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Number("12345".to_string())
            }]
        );
    }
}
