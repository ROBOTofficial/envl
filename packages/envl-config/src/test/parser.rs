#[cfg(test)]
pub mod parser_test {
    use std::collections::HashMap;

    use envl_utils::error::EnvlError;

    use crate::{
        lexer::Lexer,
        misc::{
            config::{
                remove_position_prop, Config, ConfigWithoutPosition, SettingWithoutPotision,
                SettingsWithoutPosition, VarWithoutPosition,
            },
            variable::{Type, Value},
        },
        parser::Parser,
    };

    fn gen_obj(code: String) -> Result<Config, EnvlError> {
        let lex = Lexer::new("test.envl".to_string(), code);
        let tokens = lex.generate();
        let parser = Parser::new("test.envl".to_string(), tokens);
        parser.parse()
    }

    fn gen_parsed_obj(code: String) -> ConfigWithoutPosition {
        remove_position_prop(gen_obj(code).unwrap())
    }

    #[test]
    fn default_file_test() {
        let config = gen_parsed_obj(include_str!("./files/default_file.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: None
                },
                vars: HashMap::new()
            }
        );
    }

    #[test]
    fn settings_test() {
        let config = gen_parsed_obj(include_str!("./files/settings.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: Some(SettingWithoutPotision {
                        value: String::from("test.envl")
                    })
                },
                vars: HashMap::new()
            }
        );
    }

    #[test]
    fn vars_test() {
        let config = gen_parsed_obj(include_str!("./files/vars.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: None
                },
                vars: HashMap::from([
                    (
                        "a".to_string(),
                        VarWithoutPosition {
                            v_type: Type::String,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "b".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Char,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "c".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Float,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "d".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Int,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "e".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Uint,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "f".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Bool,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "g".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Array(Box::from(Type::Int)),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "h".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Struct(HashMap::from([
                                ("a".to_string(), Type::Bool),
                                ("b".to_string(), Type::Int)
                            ])),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "i".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Null,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "j".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Array(Box::from(Type::Array(Box::from(Type::Struct(
                                HashMap::from([("a".to_string(), Type::Int)])
                            ))))),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "k".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Struct(HashMap::from([
                                (
                                    "a".to_string(),
                                    Type::Struct(HashMap::from([("b".to_string(), Type::Bool)]))
                                ),
                                ("b".to_string(), Type::Array(Box::from(Type::Int)))
                            ])),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    )
                ])
            }
        );
    }

    #[test]
    fn option_value_test() {
        let config = gen_parsed_obj(include_str!("./files/option_value.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: None
                },
                vars: HashMap::from([
                    (
                        "a".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Int,
                            default_value: Value::Int(123),
                            actions_value: Value::Int(456)
                        }
                    ),
                    (
                        "b".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Bool,
                            default_value: Value::Bool(false),
                            actions_value: Value::Bool(true)
                        }
                    ),
                    (
                        "c".to_string(),
                        VarWithoutPosition {
                            v_type: Type::String,
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "d".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Array(Box::from(Type::Int)),
                            default_value: Value::Array(vec![Value::Int(123), Value::Int(456)]),
                            actions_value: Value::Array(vec![])
                        }
                    ),
                    (
                        "e".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Array(Box::from(Type::Array(Box::from(Type::Int)))),
                            default_value: Value::Array(vec![
                                Value::Array(vec![Value::Int(123)]),
                                Value::Array(vec![Value::Int(456)])
                            ]),
                            actions_value: Value::Array(vec![Value::Array(vec![])])
                        }
                    ),
                    (
                        "f".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Struct(HashMap::from([("a".to_string(), Type::Bool)])),
                            default_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Bool(true)
                            )])),
                            actions_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Bool(false)
                            )]))
                        }
                    ),
                    (
                        "g".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Struct(HashMap::from([(
                                "a".to_string(),
                                Type::Struct(HashMap::from([("b".to_string(), Type::Bool)]))
                            )])),
                            default_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Struct(HashMap::from([(
                                    "b".to_string(),
                                    Value::Bool(true)
                                )]))
                            )])),
                            actions_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Struct(HashMap::from([(
                                    "b".to_string(),
                                    Value::Bool(false)
                                )]))
                            )]))
                        }
                    )
                ])
            }
        );
    }

    #[test]
    fn uncommon_option_value_test() {
        let config =
            gen_parsed_obj(include_str!("./files/uncommon_option_value.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: None
                },
                vars: HashMap::from([
                    (
                        "a".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Array(Box::from(Type::Struct(HashMap::from([(
                                "a".to_string(),
                                Type::Int
                            )])))),
                            default_value: Value::Array(vec![
                                Value::Struct(HashMap::from([("a".to_string(), Value::Int(123))])),
                                Value::Struct(HashMap::from([("a".to_string(), Value::Int(456))]))
                            ]),
                            actions_value: Value::Array(vec![])
                        }
                    ),
                    (
                        "b".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Struct(HashMap::from([(
                                "a".to_string(),
                                Type::Array(Box::from(Type::Int))
                            )])),
                            default_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Array(vec![Value::Int(123), Value::Int(456)])
                            )])),
                            actions_value: Value::Struct(HashMap::from([(
                                "a".to_string(),
                                Value::Array(vec![])
                            )]))
                        }
                    )
                ])
            }
        )
    }

    #[test]
    fn optional_test() {
        let config = gen_parsed_obj(include_str!("./files/optional.test.envl").to_string());
        assert_eq!(
            config,
            ConfigWithoutPosition {
                settings: SettingsWithoutPosition {
                    envl_file_path: None
                },
                vars: HashMap::from([
                    (
                        "a".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Option(Box::from(Type::Bool)),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "b".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Option(Box::from(Type::Array(Box::from(Type::Int)))),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "c".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Option(Box::from(Type::Struct(HashMap::from([
                                ("a".to_string(), Type::Bool),
                                ("b".to_string(), Type::Option(Box::from(Type::Int)))
                            ])))),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    ),
                    (
                        "d".to_string(),
                        VarWithoutPosition {
                            v_type: Type::Option(Box::from(Type::Array(Box::from(Type::Option(
                                Box::from(Type::String)
                            ))))),
                            default_value: Value::Null,
                            actions_value: Value::Null
                        }
                    )
                ])
            }
        )
    }
}
