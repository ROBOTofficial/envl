use std::{collections::HashMap, io::Error};

use envl_config::misc::variable::{Type, Value};
use quote::quote;

use crate::{generator::rust::value::gen_value, VarData, VariableHashMap};

pub mod array;
pub mod gen_struct;
pub mod value;

pub fn parse_v_type(v_name: String, v_type: Type, structs: &mut Vec<String>) -> String {
    match v_type {
        Type::Array(boxed_element_type) => format!(
            "Vec<{}>",
            parse_v_type(v_name, *boxed_element_type, structs)
        ),
        Type::Bool => "bool".to_string(),
        Type::Char => "char".to_string(),
        Type::Float => "f64".to_string(),
        Type::Int => "i64".to_string(),
        Type::Null => "None".to_string(),
        Type::String => "String".to_string(),
        Type::Option(t) => format!("Option<{}>", parse_v_type(v_name, *t, structs)),
        Type::Struct(elements) => {
            let struct_name = format!("Struct{}", v_name);
            let struct_value = elements
                .iter()
                .map(|(n, v)| {
                    let name = match v {
                        Type::Struct(_) => {
                            format!("{}{}", n, struct_name)
                        }
                        _ => n.to_string(),
                    };
                    let v_type = parse_v_type(name.to_owned(), v.to_owned(), structs);
                    quote! {#name, #v_type}
                })
                .collect::<Vec<_>>();

            structs.push(
                quote! {
                    #[derive(Debug, Clone)]
                    struct #struct_name {
                        #(
                            pub #struct_value,
                        )*
                    }
                }
                .to_string(),
            );

            struct_name
        }
        Type::Uint => "u64".to_string(),
    }
}

pub fn parse_var(name: String, var: VarData, structs: &mut Vec<String>) -> Result<String, Error> {
    match var.value {
        Value::Null => {
            match gen_value(
                name,
                var.v_type.to_owned(),
                var.default_value.to_owned(),
                structs,
            ) {
                Ok(r) => Ok(r),
                Err(err) => Err(err),
            }
        }
        _ => match gen_value(name, var.v_type.to_owned(), var.value.to_owned(), structs) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        },
    }
}

pub fn generate_rust_file(data: VariableHashMap) -> Result<String, Error> {
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
        .map(|(n, v)| quote! {stringify!(#n), stringify!(#v)})
        .collect::<Vec<_>>();
    let env_value = value_hm
        .iter()
        .map(|(n, v)| quote! {stringify!(#n), stringify!(#v)})
        .collect::<Vec<_>>();

    Ok(quote! {
        use envl::VariableHashMap;
        use envl_config::misc::variable::Value;

        #(#structs)*
        #(#struct_values)*

        #[derive(Debug, Clone)]
        pub struct Env {
            #(
                pub #env_type,
            )*
        }

        pub const ENV: Env = Env {
            #(
                #env_value,
            )*
        };
    }
    .to_string())
}
