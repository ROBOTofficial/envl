#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        lexer::lexer::Lexer,
        misc::variable::{Variable, VariableValue, VariableWithoutPosition},
        parser::{
            error::ErrorKind,
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
    fn array_test() {
        let result = gen_vars("variable = [ \"abc\", 'a', 12345, true ];".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Array(vec![
                    VariableValue::String("abc".to_string()),
                    VariableValue::Char('a'),
                    VariableValue::Number("12345".to_string()),
                    VariableValue::Bool(true),
                ])
            }]
        );
    }

    #[test]
    fn double_array_test() {
        let result = gen_vars("variable = [ [ 123 ], [\"456\"] ];".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Array(vec![
                    VariableValue::Array(vec![VariableValue::Number("123".to_string())]),
                    VariableValue::Array(vec![VariableValue::String("456".to_string())])
                ])
            }]
        );
    }

    #[test]
    fn triple_array_test() {
        let result = gen_vars("variable = [ [ [ 123 ] ], [ [\"456\"] ] ];".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Array(vec![
                    VariableValue::Array(vec![VariableValue::Array(vec![VariableValue::Number(
                        "123".to_string()
                    )])]),
                    VariableValue::Array(vec![VariableValue::Array(vec![VariableValue::String(
                        "456".to_string()
                    )])]),
                ])
            }]
        );
    }

    #[test]
    fn struct_test() {
        let result = gen_vars("variable = { abc: 12345, efg: true };".to_string());
        assert_eq!(
            result,
            vec![VariableWithoutPosition {
                name: "variable".to_string(),
                value: VariableValue::Struct(HashMap::from([
                    (
                        "abc".to_string(),
                        VariableValue::Number("12345".to_string()),
                    ),
                    ("efg".to_string(), VariableValue::Bool(true)),
                ]))
            }]
        );
    }

    #[test]
    fn struct_and_array_test() {
        let result = gen_vars(
            "variable = { abc: { efg: [ true ] }, hij: 12345 }; variable2 = [ { abc: true }, 12345 ];".to_string()
        );
        assert_eq!(
            result,
            vec![
                VariableWithoutPosition {
                    name: "variable".to_string(),
                    value: VariableValue::Struct(HashMap::from([
                        (
                            "abc".to_string(),
                            VariableValue::Struct(HashMap::from([(
                                "efg".to_string(),
                                VariableValue::Array(vec![VariableValue::Bool(true)])
                            )])),
                        ),
                        (
                            "hij".to_string(),
                            VariableValue::Number("12345".to_string())
                        ),
                    ]))
                },
                VariableWithoutPosition {
                    name: "variable2".to_string(),
                    value: VariableValue::Array(vec![
                        VariableValue::Struct(HashMap::from([(
                            "abc".to_string(),
                            VariableValue::Bool(true)
                        )])),
                        VariableValue::Number("12345".to_string())
                    ])
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
            assert_eq!(err.kind, ErrorKind::InvalidType);
        }
    }

    #[test]
    fn duplicate_error_test() {
        let result = gen_parsed_vars("variable = 12345; variable = \"12345\";".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind, ErrorKind::DuplicateVars);
        }
    }

    #[test]
    fn invalid_type_error_test() {
        let result = gen_parsed_vars("variable = aiueo;".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind, ErrorKind::InvalidType);
        }
    }

    #[test]
    fn multiple_char_error() {
        let result = gen_parsed_vars("variable = 'char';".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind, ErrorKind::MultipleCharacters);
        }
    }
}
