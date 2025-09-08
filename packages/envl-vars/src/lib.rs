use envl_utils::error::EnvlError;

use crate::{lexer::Lexer, misc::variable::Variable, parser::Parser};

pub mod lexer;
pub mod misc;
pub mod parser;

pub fn generate_ast(file_path: String, code: String) -> Result<Vec<Variable>, EnvlError> {
    let lexer = Lexer::new(file_path, code);
    let tokens = lexer.generate();
    let parser = Parser::new(tokens);
    parser.parse()
}
