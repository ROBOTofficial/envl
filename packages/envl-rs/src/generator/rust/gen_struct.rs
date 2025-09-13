use std::{collections::HashMap, io::Error};

use envl_config::misc::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::rust::value::gen_value;

pub fn gen_struct(
    name: String,
    t: HashMap<String, Type>,
    v: HashMap<String, Value>,
    structs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
    let struct_type = format!("Struct{}", name).parse::<TokenStream>().unwrap();
    let struct_name = format!("struct{}", name).parse::<TokenStream>().unwrap();
    let mut struct_values = Vec::new();

    for (n, element_type) in t {
        if let Some(value) = v.get(&n) {
            let element_name = match value {
                Value::Struct(_) => {
                    format!("{}{}", n.to_owned(), struct_name)
                }
                _ => n.to_owned(),
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
            let name = n.parse::<TokenStream>().unwrap();
            quote! {#name: #v}
        })
        .collect::<Vec<_>>();

    structs.push(quote! {
        let #struct_name = #struct_type {
            #(
                #elements,
            )*
        };
    });

    Ok(quote! {
        #struct_name
    })
}
