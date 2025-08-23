#[cfg(test)]
mod test {
    use crate::{
        lexer::lexer::Lexer,
        misc::{token::Value, variable::Type},
    };

    fn generate_tokens(code: String) -> Vec<Value> {
        let lex = Lexer::new("test.envl".to_string(), code);
        lex.generate()
            .into_iter()
            .map(|t| t.value)
            .collect::<Vec<_>>()
    }

    #[test]
    fn default_file_test() {
        let tokens = generate_tokens(include_str!("./files/default_file.test.envl").to_string());
        assert_eq!(
            tokens,
            vec![
                Value::Settings,
                Value::LeftCurlyBracket,
                Value::RightCurlyBracket,
                Value::Vars,
                Value::LeftCurlyBracket,
                Value::RightCurlyBracket,
            ]
        );
    }

    #[test]
    fn settings_test() {
        let tokens = generate_tokens(include_str!("./files/settings.test.envl").to_string());
        assert_eq!(
            tokens,
            vec![
                Value::Settings,
                Value::LeftCurlyBracket,
                Value::Ident(String::from("envl_file_path")),
                Value::Equal,
                Value::Ident(String::from("\"test.envl\"")),
                Value::Semi,
                Value::RightCurlyBracket,
                Value::Vars,
                Value::LeftCurlyBracket,
                Value::RightCurlyBracket,
            ]
        );
    }

    #[test]
    fn vars_test() {
        let tokens = generate_tokens(include_str!("./files/vars.test.envl").to_string());
        assert_eq!(
            tokens,
            vec![
                Value::Settings,
                Value::LeftCurlyBracket,
                Value::RightCurlyBracket,
                Value::Vars,
                Value::LeftCurlyBracket,
                Value::Ident("a".to_string()),
                Value::Colon,
                Value::Type(Type::String),
                Value::Comma,
                Value::Ident("b".to_string()),
                Value::Colon,
                Value::Type(Type::Char),
                Value::Comma,
                Value::Ident("c".to_string()),
                Value::Colon,
                Value::Type(Type::Float),
                Value::Comma,
                Value::Ident("d".to_string()),
                Value::Colon,
                Value::Type(Type::Int),
                Value::Comma,
                Value::Ident("e".to_string()),
                Value::Colon,
                Value::Type(Type::Uint),
                Value::Comma,
                Value::Ident("f".to_string()),
                Value::Colon,
                Value::Type(Type::Bool),
                Value::RightCurlyBracket,
            ]
        );
    }
}
