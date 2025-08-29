use std::env::current_dir;

use clap::{Parser, Subcommand};
use envl_config::generate_ast;

use crate::{
    generate_typefile::generate_typefile,
    misc::file_system::{read_file, write_file},
};

pub mod generate_typefile;
pub mod misc;

#[derive(Parser, Debug, Clone)]
#[command(version, about, flatten_help = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Type { output: String },
}

fn main() {
    let args = Args::parse();
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();
    let code = read_file(config_path.to_owned()).unwrap();
    let config = generate_ast(config_path.to_owned(), code).unwrap();

    match args.command {
        Command::Type { output } => {
            let typefile = generate_typefile(config, output.clone()).unwrap();
            write_file(current_dir.join(output).display().to_string(), typefile).unwrap();
        }
    }
}
