#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, misc::token::Value};

    fn generate_tokens(code: String) -> Vec<Value> {
        let lex = Lexer::new("test.envl".to_string(), code);
        lex.generate()
            .into_iter()
            .map(|t| t.value)
            .collect::<Vec<_>>()
    }

    #[test]
    fn default_file_test() {
        let tokens = generate_tokens("settings {} vars {}".to_string());
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
}
