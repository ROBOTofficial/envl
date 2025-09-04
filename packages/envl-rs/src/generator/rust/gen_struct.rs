use std::{
    collections::HashMap,
    io::Error,
};

use envl_config::misc::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::rust::value::gen_value;

pub fn gen_struct(
    name: String,
    t: HashMap<String, Type>,
    v: HashMap<String, Value>,
    structs: &mut Vec<String>,
) -> Result<TokenStream, Error> {
    let struct_type = format!("Struct{}", name);
    let struct_name = format!("struct{}", name).to_uppercase();
    let mut struct_values = Vec::new();

    for (n, element_type) in t {
        if let Some(value) = v.get(&n) {
            let element_name = match value {
                Value::Struct(_) => {
                    format!("{}{}", name.to_owned(), struct_name)
                }
                _ => name.to_owned(),
            };
            match gen_value(
                element_name.to_owned(),
                element_type.to_owned(),
                value.to_owned(),
                structs,
            ) {
                Ok(r) => {
                    struct_values.push((element_name, r));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            return Err(Error::other(""));
        }
    }

    let elements = struct_values
        .iter()
        .map(|(n, v)| {
            quote! {stringify!(#n), stringify!(#v)}
        })
        .collect::<Vec<_>>();

    structs.push(
        quote! {
            pub const #struct_name = struct #struct_type {
                #(
                    pub #elements,
                )*
            };
        }
        .to_string(),
    );

    let result = struct_name.parse::<TokenStream>().unwrap();

    Ok(quote! {
        #result
    })
}
