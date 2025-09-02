use std::{collections::HashMap, io::Error};

use envl_config::misc::variable::{Type, Value};
use quote::quote;

use crate::{VarData, VariableHashMap};

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
                    let v_type = parse_v_type(n.to_owned(), v.to_owned(), structs);
                    quote! {#n, #v_type}
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

pub fn generate_rust_file(data: VariableHashMap) -> Result<String, Error> {
    let mut structs = Vec::new();
    let mut hm = HashMap::new();

    for (name, value) in data {
        let parsed_type = parse_v_type(name.to_owned(), value.v_type, &mut structs);
        hm.insert(name, parsed_type);
    }

    let env_type = hm
        .iter()
        .map(|(n, v)| quote! {stringify!(#n), stringify!(#v)})
        .collect::<Vec<_>>();

    Ok(quote! {
        use envl::VariableHashMap;
        use envl_config::misc::variable::Value;

        #(#structs)*

        #[derive(Debug, Clone)]
        pub struct Env {
            #(
                #env_type,
            )*
        }

        pub const ENV: Env = Env {};
    }
    .to_string())
}
