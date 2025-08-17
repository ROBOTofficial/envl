use crate::misc::{
    position::Position,
    token::{Token, Value},
};

pub struct Lexer {
    file_path: String,
    code: String,
}

impl Lexer {
    pub fn new(file_path: String, code: String) -> Self {
        Self { file_path, code }
    }

    pub fn generate(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut row = 1;
        let mut col = 0;
        let mut in_quote = false;
        let mut is_comment = false;
        let mut is_escape = false;
        let mut current_token = String::new();

        'lexer_loop: for (i, c) in self.code.char_indices() {
            let is_last = self.code.len() == (i + 1);
            let mut is_others = false;

            if is_comment && (c == '\n' || is_last) {
                if c != '\n' && is_last {
                    current_token.push(c);
                }
                tokens.push(Token {
                    value: Value::Comment(current_token.clone()),
                    position: Position {
                        file_path: self.file_path.clone(),
                        row: row.clone(),
                        col: col.clone(),
                    },
                });
                current_token.clear();
                is_comment = false;
            }

            if c == '\n' {
                row += 1;
                col = 0;
                continue;
            }

            col += 1;

            let position = Position {
                file_path: self.file_path.clone(),
                row: row.clone(),
                col: col.clone(),
            };

            if is_escape {
                current_token.push(match c {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '0' => '\0',
                    _ => c,
                });
                is_escape = false;
                continue;
            }

            if (in_quote && c != '"' && c != '\'') || is_comment {
                current_token.push(c);
                continue;
            }

            match c {
                '"' | '\'' => {
                    if in_quote {
                        let quote = c.clone();
                        tokens.push(Token {
                            value: Value::Ident(format!(
                                "{}{}{}",
                                quote,
                                current_token.clone(),
                                quote
                            )),
                            position: position.clone(),
                        });
                    }
                    in_quote = !in_quote;
                    current_token.clear();
                }
                '\\' => {
                    is_escape = true;
                }
                ';' => {
                    tokens.push(Token {
                        value: Value::Semi,
                        position: position.clone(),
                    });
                }
                '=' => {
                    tokens.push(Token {
                        value: Value::Equal,
                        position: position.clone(),
                    });
                }
                '/' => {
                    if current_token == "/" {
                        is_comment = true;
                        current_token.clear();
                    } else {
                        current_token.push(c);
                        continue 'lexer_loop;
                    }
                }
                other => {
                    if other.is_whitespace() && !in_quote && !is_comment {
                        if !current_token.is_empty() {
                            let identifier = Value::Ident(current_token.clone());
                            tokens.push(Token {
                                value: identifier,
                                position: position.clone(),
                            });
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                    is_others = true;
                }
            }

            if !is_comment && !in_quote && !is_others && !current_token.is_empty() {
                let identifier = Value::Ident(current_token.clone());
                tokens.insert(
                    tokens.len() - 1,
                    Token {
                        value: identifier,
                        position,
                    },
                );
                current_token.clear();
            }
        }

        tokens
    }
}

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
