pub mod token;

pub struct CApi {}

impl CApi {
    #[unsafe(no_mangle)]
    pub fn generate_ast() {}
}
