use std::io::Error;

use envl_config::misc::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::rust::var::gen_value;

pub fn gen_array(
    name: String,
    t: Type,
    v: Vec<Value>,
    structs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
    let mut vec_values = Vec::new();

    for value in v {
        match gen_value(name.to_owned(), t.to_owned(), value, structs) {
            Ok(r) => {
                vec_values.push(r);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(quote! {
        Vec::from([
            #(
                #vec_values,
            )*
        ])
    })
}
