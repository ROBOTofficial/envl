use envl_utils::error::EnvlError;

use crate::{lexer::Lexer, misc::config::Config, parser::Parser};

pub mod lexer;
pub mod misc;
pub mod parser;
pub mod test;

pub fn generate_ast(file_path: String, code: String) -> Result<Config, EnvlError> {
    let lex = Lexer::new(file_path.clone(), code);
    let tokens = lex.generate();
    let parser = Parser::new(file_path, tokens);
    parser.parse()
}
