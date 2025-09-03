use std::io::{Error, ErrorKind};

use envl_config::misc::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::rust::{array::gen_array, gen_struct::gen_struct};

pub fn gen_value(
    name: String,
    t: Type,
    v: Value,
    structs: &mut Vec<String>,
) -> Result<TokenStream, Error> {
    let result = match &v {
        Value::Null => Ok(quote! {None}),
        Value::String(s) => Ok(quote! {#s}),
        Value::Char(c) => Ok(quote! {#c}),
        Value::Float(f) => Ok(quote! {#f}),
        Value::Int(i) => Ok(quote! {#i}),
        Value::Uint(u) => Ok(quote! {#u}),
        Value::Bool(b) => Ok(quote! {#b}),
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
            Type::Struct(_) => match gen_struct(name, t.to_owned(), value.to_owned(), structs) {
                Ok(r) => Ok(r),
                Err(err) => Err(err),
            },
            _ => Err(Error::new(ErrorKind::Other, "Invalid Type")),
        },
    };

    match result {
        Ok(token) => match t.clone() {
            Type::Option(_) => Ok(quote! {
                Some(#token)
            }),
            _ => Ok(token.to_owned()),
        },
        Err(err) => Err(err),
    }
}
