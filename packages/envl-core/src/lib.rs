pub mod token;
pub mod parser;
pub mod lexer;

pub struct CApi {}

impl CApi {
    #[unsafe(no_mangle)]
    pub fn generate_ast() {}
}
