use envl_utils::error::ErrorContext;

use crate::misc::variable::{Type, Value};

pub fn parse_value(t: Type, ident: String) -> Result<Value, ErrorContext> {
    match t {
        Type::Null => Ok(Value::Null),
        Type::String => {
            if ident.starts_with('"') && ident.ends_with('"') {
                let mut str_value = ident.to_owned();
                str_value.remove(ident.len() - 1);
                str_value.remove(0);
                Ok(Value::String(str_value))
            } else {
                Err(ErrorContext::InvalidType)
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
                    Err(ErrorContext::MultipleChar)
                }
            } else {
                Err(ErrorContext::InvalidType)
            }
        }
        Type::Float => {
            if let Ok(n) = ident.parse::<f64>() {
                Ok(Value::Float(n))
            } else {
                Err(ErrorContext::InvalidType)
            }
        }
        Type::Int => {
            if let Ok(n) = ident.parse::<i64>() {
                Ok(Value::Int(n))
            } else {
                Err(ErrorContext::InvalidType)
            }
        }
        Type::Uint => {
            if let Ok(n) = ident.parse::<u64>() {
                Ok(Value::Uint(n))
            } else {
                Err(ErrorContext::InvalidType)
            }
        }
        Type::Bool => {
            if let Ok(b) = ident.parse::<bool>() {
                Ok(Value::Bool(b))
            } else {
                Err(ErrorContext::InvalidType)
            }
        }
        _ => Err(ErrorContext::InvalidType),
    }
}
