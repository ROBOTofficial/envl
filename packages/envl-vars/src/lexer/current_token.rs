use crate::{lexer::Lexer, misc::token::Value};

impl Lexer {
    pub fn lex_current_token(&self, token: String) -> Value {
        match token.as_str() {
            "struct" => Value::Struct,
            other => Value::Ident(other.to_string()),
        }
    }
}
