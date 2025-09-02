use std::io::Error;

use quote::quote;

use crate::VariableHashMap;

pub fn generate_rust_file(data: VariableHashMap) -> Result<String, Error> {
    Ok(quote! {
        use envl::VariableHashMap;
        use envl_config::misc::variable::Value;

        #[derive(Debug, Clone)]
        pub struct Env {}

        pub const ENV: Env = Env {};
    }
    .to_string())
}
