use envl_utils::{
    error::{EnvlError, ErrorContext},
    name::is_valid_variable_name,
    types::Position,
};

use crate::parser::{ParsedIdent, Parser, Var};

impl Parser {
    pub fn parse_ident(
        &self,
        value: String,
        var: &Var,
        position: &Position,
        equal_used: &bool,
    ) -> Result<ParsedIdent, EnvlError> {
        if var.name.is_some() && var.value.is_some() {
            return Err(EnvlError {
                message: ErrorContext::InvalidSyntax,
                position: position.clone(),
            });
        }
        if var.name.is_none() && !equal_used {
            if !is_valid_variable_name(&value) {
                Err(EnvlError {
                    message: ErrorContext::InvalidName(value.to_string()),
                    position: position.clone(),
                })
            } else {
                Ok(ParsedIdent::Name(value.clone()))
            }
        } else if var.value.is_none() && *equal_used {
            let var_value = self.parse_value(&value, position);
            match var_value {
                Ok(var_value) => Ok(ParsedIdent::Value(var_value)),
                Err(err) => Err(err),
            }
        } else {
            Err(EnvlError {
                message: ErrorContext::InvalidSyntax,
                position: position.clone(),
            })
        }
    }
}
