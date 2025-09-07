use envl_utils::types::Position;

use crate::{
    misc::error::{EnvlVarsError, ErrorContext},
    parser::{ParsedIdent, Parser, Var},
};

impl Parser {
    pub fn parse_ident(
        &self,
        value: String,
        var: &Var,
        position: &Position,
        equal_used: &bool,
    ) -> Result<ParsedIdent, EnvlVarsError> {
        if var.name.is_some() && var.value.is_some() {
            return Err(EnvlVarsError {
                message: ErrorContext::InvalidSyntax,
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
            Err(EnvlVarsError {
                message: ErrorContext::InvalidSyntax,
                position: position.clone(),
            })
        }
    }
}
