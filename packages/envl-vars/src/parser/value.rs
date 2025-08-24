use crate::{
    misc::{num::is_num, position::Position, variable::VariableValue},
    parser::{
        error::{INVALID_TYPE, MULTIPLE_CHAR},
        parser::{Parser, ParserError},
    },
};

impl Parser {
    pub fn parse_value(
        &self,
        value: &str,
        position: &Position,
    ) -> Result<VariableValue, ParserError> {
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
                Err(ParserError {
                    kind: MULTIPLE_CHAR.kind,
                    code: MULTIPLE_CHAR.code,
                    message: MULTIPLE_CHAR.message.to_string(),
                    position: position.clone(),
                })
            }
        } else if is_num(value.to_owned(), true) {
            Ok(VariableValue::Number(value.to_owned()))
        } else if let Ok(b) = value.parse::<bool>() {
            Ok(VariableValue::Bool(b))
        } else {
            Err(ParserError {
                kind: INVALID_TYPE.kind,
                code: INVALID_TYPE.code,
                message: INVALID_TYPE.message.to_string(),
                position: position.clone(),
            })
        }
    }
}
