use envl_config::{
    generate_ast as gen_config_ast,
    misc::{
        config::Config,
        variable::{Type, Value},
    },
    parser::error::ParserError as ConfigParserError,
};
use envl_vars::{
    generate_ast as gen_vars_ast,
    misc::position::Position,
    misc::variable::{Variable, VariableValue},
    parser::ParserError as VarsParserError,
};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{Error, Read},
};

use crate::misc::vars::vars_to_hashmap;

pub mod misc;
pub mod test;

#[derive(Debug, Clone)]
pub struct EnvlLibError {
    pub message: String,
}

pub enum EnvlError {
    Vars(VarsParserError),
    Config(ConfigParserError),
    Original(EnvlLibError),
    Error(Error),
}

pub trait Env {
    fn from_hashmap<T>(hashmap: VariableHashMap) -> Result<T, EnvlError>;
}

#[derive(Debug, Clone)]
pub struct VarData {
    pub v_type: Type,
    pub default_value: Value,
    pub actions_value: Value,
    pub value: VariableValue,
    pub position: Position,
}

pub type VariableHashMap = HashMap<String, VarData>;

pub fn load_envl<T: Env>() -> Result<T, EnvlError> {
    match current_dir() {
        Ok(current_dir_path) => {
            let config_file_path = current_dir_path.join(".envlconf");
            match File::open(config_file_path.to_owned()) {
                Ok(mut f) => {
                    let mut buf = String::new();
                    let _ = f.read_to_string(&mut buf);
                    load_envl_core(config_file_path.display().to_string(), buf)
                }
                Err(err) => Err(EnvlError::Error(err)),
            }
        }
        Err(err) => Err(EnvlError::Error(err)),
    }
}

fn load_envl_core<T: Env>(file_path: String, code: String) -> Result<T, EnvlError> {
    match load_files(file_path, code) {
        Ok((vars, config)) => {
            let vars_hm = vars_to_hashmap(vars);
            let mut result = HashMap::new();

            for (name, value) in config.vars {
                if let Some(v) = vars_hm.get(&name) {
                    result.insert(
                        name,
                        VarData {
                            v_type: value.v_type.clone(),
                            default_value: value.default_value,
                            actions_value: value.actions_value,
                            value: v.value.clone(),
                            position: v.position.clone(),
                        },
                    );
                } else {
                    return Err(EnvlError::Original(EnvlLibError {
                        message: format!("{} is not foud", &name),
                    }));
                }
            }

            T::from_hashmap::<T>(result)
        }
        Err(err) => Err(err),
    }
}

pub fn load_files(file_path: String, code: String) -> Result<(Vec<Variable>, Config), EnvlError> {
    match gen_config_ast(file_path.clone(), code.clone()) {
        Ok(config) => match gen_vars_ast(file_path, code) {
            Ok(vars) => Ok((vars, config)),
            Err(err) => Err(EnvlError::Vars(err)),
        },
        Err(err) => Err(EnvlError::Config(err)),
    }
}
