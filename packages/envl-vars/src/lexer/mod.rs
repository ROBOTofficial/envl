use envl_utils::types::Position;

use crate::misc::token::{Token, Value};

pub mod current_token;
pub mod test;

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
        let mut start_quote = char::default();
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
                        row,
                        col,
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
                row,
                col,
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
                        tokens.push(Token {
                            value: Value::Ident(format!(
                                "{}{}{}",
                                start_quote,
                                current_token.clone(),
                                c
                            )),
                            position: position.clone(),
                        });
                        start_quote = char::default();
                    } else {
                        start_quote = c;
                    }
                    in_quote = !in_quote;
                    current_token.clear();
                }
                '\\' => {
                    is_escape = true;
                }
                '[' => {
                    tokens.push(Token {
                        value: Value::LeftSquareBracket,
                        position: position.clone(),
                    });
                }
                ']' => {
                    tokens.push(Token {
                        value: Value::RightSquareBracket,
                        position: position.clone(),
                    });
                }
                '{' => {
                    tokens.push(Token {
                        value: Value::LeftCurlyBracket,
                        position: position.clone(),
                    });
                }
                '}' => {
                    tokens.push(Token {
                        value: Value::RightCurlyBracket,
                        position: position.clone(),
                    });
                }
                ':' => {
                    tokens.push(Token {
                        value: Value::Colon,
                        position: position.clone(),
                    });
                }
                ',' => {
                    tokens.push(Token {
                        value: Value::Comma,
                        position: position.clone(),
                    });
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
                            let identifier = self.lex_current_token(current_token.clone());
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
                let identifier = self.lex_current_token(current_token.clone());
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
