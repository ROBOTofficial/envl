#[cfg(test)]
mod lexer_test {
    use crate::{lexer::lexer::Lexer, misc::token::Value};

    fn generate_tokens(code: String) -> Vec<Value> {
        let lex = Lexer::new("test.envl".to_string(), code);
        lex.generate()
            .into_iter()
            .map(|t| t.value)
            .collect::<Vec<_>>()
    }

    #[test]
    fn number_test() {
        let tokens = generate_tokens("variable = 12345;".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::Ident("12345".to_string()),
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn string_test() {
        let tokens = generate_tokens("variable = \"12345\";".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::Ident("\"12345\"".to_string()),
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn char_test() {
        let tokens = generate_tokens("variable = 'a';".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::Ident("'a'".to_string()),
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn bool_test() {
        let tokens = generate_tokens("variable = true; variable2 = false;".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::Ident("true".to_string()),
            Value::Semi,
            Value::Ident("variable2".to_string()),
            Value::Equal,
            Value::Ident("false".to_string()),
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn array_test() {
        let tokens = generate_tokens("variable = [ \"abc\", 'a', 12345, true ];".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::LeftSquareBracket,
            Value::Ident("\"abc\"".to_string()),
            Value::Comma,
            Value::Ident("'a'".to_string()),
            Value::Comma,
            Value::Ident("12345".to_string()),
            Value::Comma,
            Value::Ident("true".to_string()),
            Value::RightSquareBracket,
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn double_array() {
        let tokens = generate_tokens("variable = [ [ 123 ], [\"456\"] ];".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::LeftSquareBracket,
            Value::LeftSquareBracket,
            Value::Ident("123".to_string()),
            Value::RightSquareBracket,
            Value::Comma,
            Value::LeftSquareBracket,
            Value::Ident("\"456\"".to_string()),
            Value::RightSquareBracket,
            Value::RightSquareBracket,
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn triple_array_test() {
        let tokens = generate_tokens("variable = [ [ [ 123 ] ], [ [\"456\"] ] ];".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::LeftSquareBracket,
            Value::LeftSquareBracket,
            Value::LeftSquareBracket,
            Value::Ident("123".to_string()),
            Value::RightSquareBracket,
            Value::RightSquareBracket,
            Value::Comma,
            Value::LeftSquareBracket,
            Value::LeftSquareBracket,
            Value::Ident("\"456\"".to_string()),
            Value::RightSquareBracket,
            Value::RightSquareBracket,
            Value::RightSquareBracket,
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn struct_test() {
        let tokens = generate_tokens("variable = { abc: 12345, efg: true };".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::LeftCurlyBracket,
            Value::Ident("abc".to_string()),
            Value::Colon,
            Value::Ident("12345".to_string()),
            Value::Comma,
            Value::Ident("efg".to_string()),
            Value::Colon,
            Value::Ident("true".to_string()),
            Value::RightCurlyBracket,
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }

    #[test]
    fn comment_test() {
        let tokens = generate_tokens("variable = 12345; //this is a comment".to_string());
        let expect_arr = vec![
            Value::Ident("variable".to_string()),
            Value::Equal,
            Value::Ident("12345".to_string()),
            Value::Semi,
            Value::Comment("this is a comment".to_string()),
        ];
        assert_eq!(tokens, expect_arr);
    }
}
