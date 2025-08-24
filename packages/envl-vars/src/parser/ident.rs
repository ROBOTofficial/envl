use crate::{
    misc::position::Position,
    parser::{
        error::DIFFERENT_ORDER,
        parser::{ParsedIdent, Parser, ParserError, Var},
    },
};

impl Parser {
    pub fn parse_ident(
        &self,
        value: String,
        var: &Var,
        position: &Position,
        equal_used: &bool,
    ) -> Result<ParsedIdent, ParserError> {
        if var.name.is_some() && var.value.is_some() {
            return Err(ParserError {
                kind: DIFFERENT_ORDER.kind,
                code: DIFFERENT_ORDER.code,
                message: DIFFERENT_ORDER.message.to_string(),
                position: position.clone(),
            });
        }
        if var.name.is_none() && !equal_used {
            Ok(ParsedIdent::Name(value.clone()))
        } else if var.value.is_none() && *equal_used {
            let var_value = self.parse_value(&value, position);
            match var_value {
                Ok(var_value) => Ok(ParsedIdent::Value(var_value)),
                Err(err) => Err(err),
            }
        } else {
            Err(ParserError {
                kind: DIFFERENT_ORDER.kind,
                code: DIFFERENT_ORDER.code,
                message: DIFFERENT_ORDER.message.to_string(),
                position: position.clone(),
            })
        }
    }
}
