use envl_utils::types::Position;

use crate::{
    misc::{
        error::{EnvlVarsError, ErrorContext},
        num::is_num,
        variable::VariableValue,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_value(
        &self,
        value: &str,
        position: &Position,
    ) -> Result<VariableValue, EnvlVarsError> {
        if value.starts_with('"') && value.ends_with('"') {
            let mut str_value = value.to_owned();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            Ok(VariableValue::String(str_value))
        } else if value.starts_with('\'') && value.ends_with('\'') {
            let mut str_value = value.to_owned();
            str_value.remove(value.len() - 1);
            str_value.remove(0);
            if let Ok(c) = str_value.parse::<char>() {
                Ok(VariableValue::Char(c))
            } else {
                Err(EnvlVarsError {
                    message: ErrorContext::MultipleChar,
                    position: position.clone(),
                })
            }
        } else if is_num(value.to_owned(), true) {
            Ok(VariableValue::Number(value.to_owned()))
        } else if let Ok(b) = value.parse::<bool>() {
            Ok(VariableValue::Bool(b))
        } else {
            Err(EnvlVarsError {
                message: ErrorContext::InvalidType,
                position: position.clone(),
            })
        }
    }
}
