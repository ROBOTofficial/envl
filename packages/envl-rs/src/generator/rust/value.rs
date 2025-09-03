use std::io::{Error, ErrorKind};

use envl_config::misc::variable::{Type, Value};
use quote::quote;

use crate::generator::rust::{array::gen_array, gen_struct::gen_struct};

pub fn gen_value(
    name: String,
    t: Type,
    v: Value,
    structs: &mut Vec<String>,
) -> Result<String, Error> {
    macro_rules! is_option {
        ($value: expr) => {
            match &t {
                Type::Option(_) => format!("Some({})", $value),
                _ => $value,
            }
        };
    }

    match &v {
        Value::Null => Ok(quote! {None}.to_string()),
        Value::String(_s) => Ok(is_option!(stringify!(s).to_string())),
        Value::Char(_c) => Ok(is_option!(stringify!(c).to_string())),
        Value::Float(_f) => Ok(is_option!(stringify!(f).to_string())),
        Value::Int(_i) => Ok(is_option!(stringify!(i).to_string())),
        Value::Uint(_u) => Ok(is_option!(stringify!(u).to_string())),
        Value::Bool(_b) => Ok(is_option!(stringify!(b).to_string())),
        Value::Array(a) => match &t {
            Type::Array(boxed_type) => {
                match gen_array(name.to_owned(), *boxed_type.to_owned(), a.to_vec(), structs) {
                    Ok(r) => Ok(r),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::new(ErrorKind::Other, "Invalid Type")),
        },
        Value::Struct(value) => match &t {
            Type::Struct(_) => match gen_struct(name, t, value.to_owned(), structs) {
                Ok(r) => Ok(r),
                Err(err) => Err(err),
            },
            _ => Err(Error::new(ErrorKind::Other, "Invalid Type")),
        },
    }
}
