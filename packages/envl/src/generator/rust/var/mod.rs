use envl_config::misc::variable::Value;
use proc_macro2::TokenStream;
use std::io::Error;

use crate::{generator::rust::var::value::gen_value, VarData};

pub mod array;
pub mod gen_struct;
pub mod value;

pub fn parse_var(
    name: String,
    var: VarData,
    structs: &mut Vec<TokenStream>,
) -> Result<String, Error> {
    match var.value {
        Value::Null => {
            match gen_value(
                name,
                var.v_type.to_owned(),
                var.default_value.to_owned(),
                structs,
            ) {
                Ok(r) => Ok(r.to_string()),
                Err(err) => Err(err),
            }
        }
        _ => match gen_value(name, var.v_type.to_owned(), var.value.to_owned(), structs) {
            Ok(r) => Ok(r.to_string()),
            Err(err) => Err(err),
        },
    }
}
