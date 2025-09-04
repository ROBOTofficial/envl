use std::io::Error;

use envl_config::misc::variable::{Type, Value};
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};

use crate::generator::rust::{array::gen_array, gen_struct::gen_struct};

pub fn gen_value(
    name: String,
    t: Type,
    v: Value,
    structs: &mut Vec<String>,
) -> Result<TokenStream, Error> {
    let result = match &v {
        Value::Null => Ok(quote! {None}),
        Value::String(s) => Ok(quote! {String::from(#s)}),
        Value::Char(c) => Ok(quote! {#c}),
        Value::Float(f) => Ok(Literal::f64_unsuffixed(*f).to_token_stream()),
        Value::Int(i) => Ok(Literal::i64_unsuffixed(*i).to_token_stream()),
        Value::Uint(u) => Ok(Literal::u64_unsuffixed(*u).to_token_stream()),
        Value::Bool(b) => Ok(quote! {#b}),
        Value::Array(a) => match &t {
            Type::Array(boxed_type) => {
                match gen_array(name.to_owned(), *boxed_type.to_owned(), a.to_vec(), structs) {
                    Ok(r) => Ok(r),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::other("Invalid Type")),
        },
        Value::Struct(value) => match &t {
            Type::Struct(struct_type) => {
                match gen_struct(name, struct_type.to_owned(), value.to_owned(), structs) {
                    Ok(r) => Ok(r),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::other("Invalid Type")),
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
