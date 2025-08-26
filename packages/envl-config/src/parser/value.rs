use crate::{
    misc::variable::{Type, Value},
    parser::error::{EnvlConfigErrorTemplate, INVALID_TYPE, MULTIPLE_CHAR},
};

pub fn parse_value(t: Type, ident: String) -> Result<Value, EnvlConfigErrorTemplate> {
    match t {
        Type::Null => Ok(Value::Null),
        Type::String => {
            if ident.starts_with('"') && ident.ends_with('"') {
                let mut str_value = ident.to_owned();
                str_value.remove(ident.len() - 1);
                str_value.remove(0);
                Ok(Value::String(str_value))
            } else {
                Err(INVALID_TYPE)
            }
        }
        Type::Char => {
            if ident.starts_with('\'') && ident.ends_with('\'') {
                let mut str_value = ident.to_owned();
                str_value.remove(ident.len() - 1);
                str_value.remove(0);
                if let Ok(c) = str_value.parse::<char>() {
                    Ok(Value::Char(c))
                } else {
                    Err(MULTIPLE_CHAR)
                }
            } else {
                Err(INVALID_TYPE)
            }
        }
        Type::Float => {
            if let Ok(n) = ident.parse::<f64>() {
                Ok(Value::Float(n))
            } else {
                Err(INVALID_TYPE)
            }
        }
        Type::Int => {
            if let Ok(n) = ident.parse::<i64>() {
                Ok(Value::Int(n))
            } else {
                Err(INVALID_TYPE)
            }
        }
        Type::Uint => {
            if let Ok(n) = ident.parse::<u64>() {
                Ok(Value::Uint(n))
            } else {
                Err(INVALID_TYPE)
            }
        }
        Type::Bool => {
            if let Ok(b) = ident.parse::<bool>() {
                Ok(Value::Bool(b))
            } else {
                Err(INVALID_TYPE)
            }
        }
        _ => Err(INVALID_TYPE),
    }
}
