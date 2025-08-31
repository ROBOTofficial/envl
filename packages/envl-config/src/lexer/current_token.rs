use crate::{
    lexer::Lexer,
    misc::{token::Value, variable::Type},
};

impl Lexer {
    pub fn lex_current_token(&self, current_token: String) -> Value {
        match current_token.as_str() {
            "string" => Value::Type(Type::String),
            "char" => Value::Type(Type::Char),
            "int" => Value::Type(Type::Int),
            "uint" => Value::Type(Type::Uint),
            "bool" => Value::Type(Type::Bool),
            "float" => Value::Type(Type::Float),
            "settings" => Value::Settings,
            "vars" => Value::Vars,
            "struct" => Value::Struct,
            "Array" => Value::Array,
            "Option" => Value::Option,
            "null" => Value::Null,
            other => Value::Ident(other.to_string()),
        }
    }
}
