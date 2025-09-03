use std::io::Error;

use envl_config::misc::variable::{Type, Value};
use quote::quote;

use crate::generator::rust::value::gen_value;

pub fn gen_array(
    name: String,
    t: Type,
    v: Vec<Value>,
    structs: &mut Vec<String>,
) -> Result<String, Error> {
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
        vec![
            #(
                #vec_values,
            )*
        ]
    }
    .to_string())
}
