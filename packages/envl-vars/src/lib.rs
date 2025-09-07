use crate::{
    lexer::Lexer,
    misc::{error::EnvlVarsError, variable::Variable},
    parser::Parser,
};

pub mod lexer;
pub mod misc;
pub mod parser;

pub fn generate_ast(file_path: String, code: String) -> Result<Vec<Variable>, EnvlVarsError> {
    let lexer = Lexer::new(file_path, code);
    let tokens = lexer.generate();
    let parser = Parser::new(tokens);
    parser.parse()
}
