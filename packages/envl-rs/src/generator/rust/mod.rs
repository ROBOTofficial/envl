use std::{collections::HashMap, io::Error};

use envl_config::misc::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{generator::rust::value::gen_value, VarData, VariableHashMap};

pub mod array;
pub mod gen_struct;
pub mod value;

pub fn struct_derive() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq)]
    }
}

pub fn parse_v_type(v_name: String, v_type: Type, structs: &mut Vec<TokenStream>) -> TokenStream {
    match v_type {
        Type::Array(boxed_element_type) => {
            let value = parse_v_type(v_name, *boxed_element_type, structs);
            quote! {
                Vec<#value>
            }
        }
        Type::Bool => quote! {bool},
        Type::Char => quote! {char},
        Type::Float => quote! {f64},
        Type::Int => quote! {i64},
        Type::Null => quote! {None},
        Type::String => quote! {String},
        Type::Option(t) => {
            let value = parse_v_type(v_name, *t, structs);
            quote! {
                Option<#value>
            }
        }
        Type::Struct(elements) => {
            let s_derive = struct_derive();
            let struct_name = format!("Struct{}", v_name).parse::<TokenStream>().unwrap();
            let struct_value = elements
                .iter()
                .map(|(n, v)| {
                    let name = match v {
                        Type::Struct(_) => {
                            format!("{}{}", struct_name, n)
                        }
                        _ => n.to_string(),
                    };
                    let token_stream_name = n.parse::<TokenStream>().unwrap();
                    let v_type = parse_v_type(name.to_owned(), v.to_owned(), structs);
                    quote! {#token_stream_name: #v_type}
                })
                .collect::<Vec<_>>();

            structs.push(quote! {
                #s_derive
                #[rustfmt::skip]
                pub struct #struct_name {
                    #(
                        pub #struct_value,
                    )*
                }
            });

            quote! {
                #struct_name
            }
        }
        Type::Uint => quote! {u64},
    }
}

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
