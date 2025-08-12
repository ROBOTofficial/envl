pub mod lexer;
pub mod parser;
pub mod token;

pub use token::Tokens;

pub struct CApi {}

impl CApi {
    #[unsafe(no_mangle)]
    pub fn generate_ast() {}
}
