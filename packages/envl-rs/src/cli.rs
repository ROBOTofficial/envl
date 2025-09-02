use std::env::current_dir;

use clap::{Parser, Subcommand};
use envl::{
    generator::generate_file,
    misc::filesystem::{read_file, write_file},
};
use envl_config::generate_ast;

#[derive(Parser, Debug, Clone)]
#[command(version, about, flatten_help = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Build { output: String },
}

fn main() {
    let args = Args::parse();
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();
    let code = read_file(config_path.to_owned()).unwrap();
    let config = generate_ast(config_path.to_owned(), code).unwrap();

    match args.command {
        Command::Build { output } => {
            let f = generate_file(config, output.clone()).unwrap();
            write_file(current_dir.join(output).display().to_string(), f).unwrap();
        }
    }
}
