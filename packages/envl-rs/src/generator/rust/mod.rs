use std::{collections::HashMap, io::Error};

use quote::quote;

use crate::{
    generator::rust::{types::parse_v_type, utils::struct_derive, var::parse_var},
    VariableHashMap,
};

pub mod types;
pub mod utils;
pub mod var;

pub fn generate_rust_file(data: VariableHashMap) -> Result<String, Error> {
    let s_derive = struct_derive();
    let mut structs = Vec::new();
    let mut struct_values = Vec::new();
    let mut types_hm = HashMap::new();
    let mut value_hm = HashMap::new();

    for (name, value) in data {
        let parsed_type = parse_v_type(name.to_owned(), value.to_owned().v_type, &mut structs);
        types_hm.insert(name.to_owned(), parsed_type);

        match parse_var(name.to_owned(), value.to_owned(), &mut struct_values) {
            Ok(v) => {
                value_hm.insert(name, v);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    let env_type = types_hm
        .iter()
        .map(|(n, v)| {
            let name = n.parse::<proc_macro2::TokenStream>().unwrap();
            quote! { #name: #v }
        })
        .collect::<Vec<_>>();
    let env_value = value_hm
        .iter()
        .map(|(n, v)| {
            let name = n.parse::<proc_macro2::TokenStream>().unwrap();
            let value = v.parse::<proc_macro2::TokenStream>().unwrap();
            quote! { #name: #value }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        #[deny(clippy::all)]

        #(#structs)*

        #s_derive
        #[rustfmt::skip]
        pub struct Env {
            #(
                pub #env_type,
            )*
        }

        #[rustfmt::skip]
        pub fn envl() -> Env {
            #(#struct_values)*

            Env {
                #(
                    #env_value,
                )*
            }
        }
    }
    .to_string())
}
