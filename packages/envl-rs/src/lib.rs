use envl_config::{
    generate_ast as gen_config_ast,
    misc::{
        config::Config,
        variable::{Type, Value},
    },
};
use envl_utils::types::Position;
use envl_vars::{
    generate_ast as gen_vars_ast,
    misc::variable::{Variable, VariableValue},
};
use std::{collections::HashMap, env::current_dir, fs::File, io::Read, path::PathBuf};

use crate::{
    misc::{
        error::{
            convert_envl_config_error, convert_envl_lib_error, convert_envl_vars_error,
            convert_io_error, EnvlError, EnvlLibError,
        },
        filesystem::read_file,
        vars::vars_to_hashmap,
    },
    var::parse_var,
};

pub mod generator;
pub mod misc;
pub mod var;

#[derive(Debug, Clone)]
pub struct VarData {
    pub value: Value,
    pub v_type: Type,
    pub default_value: Value,
    pub actions_value: Value,
    pub basic_value: Option<VariableValue>,
    pub position: Position,
}

pub type VariableHashMap = HashMap<String, VarData>;

pub fn load_envl() -> Result<VariableHashMap, EnvlError> {
    match current_dir() {
        Ok(current_dir_path) => {
            let config_file_path = current_dir_path.join(".envlconf");
            match File::open(config_file_path.to_owned()) {
                Ok(mut f) => {
                    let mut buf = String::new();
                    let _ = f.read_to_string(&mut buf);
                    load_envl_core(
                        current_dir_path,
                        config_file_path.display().to_string(),
                        buf,
                    )
                }
                Err(err) => Err(convert_io_error(err)),
            }
        }
        Err(err) => Err(convert_io_error(err)),
    }
}

pub fn load_envl_core(
    current_dir: PathBuf,
    config_file_path: String,
    code: String,
) -> Result<VariableHashMap, EnvlError> {
    match load_files(current_dir, config_file_path, code) {
        Ok((vars, config)) => {
            let vars_hm = vars_to_hashmap(vars);
            let mut result = HashMap::new();

            for (name, value) in config.vars {
                if let Some(v) = vars_hm.get(&name) {
                    match parse_var(value.v_type.clone(), v.value.clone()) {
                        Ok(var) => {
                            result.insert(
                                name,
                                VarData {
                                    value: var,
                                    v_type: value.v_type.clone(),
                                    default_value: value.default_value,
                                    actions_value: value.actions_value,
                                    basic_value: Some(v.value.clone()),
                                    position: v.position.clone(),
                                },
                            );
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                } else {
                    match value.v_type {
                        Type::Option(_) => {
                            result.insert(
                                name,
                                VarData {
                                    value: Value::Null,
                                    v_type: value.v_type,
                                    default_value: value.default_value,
                                    actions_value: value.actions_value,
                                    basic_value: None,
                                    position: value.position,
                                },
                            );
                        }
                        _ => {
                            return Err(convert_envl_lib_error(EnvlLibError {
                                message: format!("{} is not foud", &name),
                            }));
                        }
                    };
                }
            }

            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub fn load_files(
    current_dir: PathBuf,
    config_file_path: String,
    code: String,
) -> Result<(Vec<Variable>, Config), EnvlError> {
    match gen_config_ast(config_file_path.clone(), code.clone()) {
        Ok(config) => {
            let file_path = if let Some(ref file_path) = config.settings.envl_file_path {
                file_path.value.clone()
            } else {
                current_dir.join(".envl").display().to_string()
            };
            match read_file(file_path.to_owned()) {
                Ok(code) => match gen_vars_ast(file_path, code) {
                    Ok(vars) => Ok((vars, config)),
                    Err(err) => Err(convert_envl_vars_error(err)),
                },
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(convert_envl_config_error(err)),
    }
}
