use crate::misc::{
    num::is_num,
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
        let mut tokens: Vec<Token> = vec![];
        let mut row = 1;
        let mut col = 0;
        let mut in_quote = false;
        let mut is_escape = false;
        let mut current_token = String::new();

        for c in self.code.chars() {
            let mut is_others = false;

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

            if in_quote && c != '"' {
                current_token.push(c);
                continue;
            }

            match c {
                '"' => {
                    if in_quote {
                        tokens.push(Token {
                            value: Value::String(current_token.clone()),
                            position: position.clone(),
                        });
                    }
                    in_quote = !in_quote
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
                other => {
                    if other.is_whitespace() && !in_quote {
                        if !current_token.is_empty() {
                            let identifier = self.get_consume_identifier(current_token.clone());
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

            if !in_quote && !is_others && !current_token.is_empty() {
                let identifier = self.get_consume_identifier(current_token.clone());
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

    fn get_consume_identifier(&self, token: String) -> Value {
        if is_num(token.clone()) {
            Value::Number(token.clone())
        } else if let Ok(b) = token.parse::<bool>() {
            Value::Bool(b)
        } else {
            Value::Variable(token)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, misc::token::Value};

    #[test]
    fn lexer_test() {
        let lex = Lexer::new("test.envl".to_string(), "variable = 12345;".to_string());
        let tokens = lex
            .generate()
            .into_iter()
            .map(|t| t.value)
            .collect::<Vec<_>>();
        let expect_arr = vec![
            Value::Variable("variable".to_string()),
            Value::Equal,
            Value::Number("12345".to_string()),
            Value::Semi,
        ];
        assert_eq!(tokens, expect_arr);
    }
}
