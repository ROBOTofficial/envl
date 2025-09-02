use std::io::Error;

use envl_config::misc::config::Config;
use quote::quote;

pub fn generate_rust_file(config: Config) -> Result<String, Error> {
    Ok(quote! {
        use envl::VariableHashMap;
        use envl_config::misc::variable::Value;

        #[derive(Debug, Clone)]
        pub struct Env {}

        pub const ENV: Env = Env {};
    }
    .to_string())
}
