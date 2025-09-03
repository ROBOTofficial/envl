use std::{env::current_dir, fs::read_to_string};

use clap::{Parser, Subcommand};
use envl::{generator::generate_file, load_envl_core, misc::filesystem::write_file};

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

fn get_config_file() -> String {
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();

    read_to_string(config_path).unwrap()
}

fn main() {
    let args = Args::parse();
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();
    let config_code = get_config_file();

    match args.command {
        Command::Build { output } => {
            let data = load_envl_core(current_dir.clone(), config_path, config_code).unwrap();

            let f = generate_file(data, output.clone()).unwrap();
            write_file(current_dir.join(output).display().to_string(), f).unwrap();
        }
    }
}
