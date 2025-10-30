use envl_config::misc::variable::Type;
use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::rust::utils::struct_derive;

pub fn parse_v_type(v_name: String, v_type: Type, structs: &mut Vec<TokenStream>) -> TokenStream {
    match v_type {
        Type::Array(boxed_element_type) => {
            let value = parse_v_type(format!("Array{}", v_name), *boxed_element_type, structs);
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
