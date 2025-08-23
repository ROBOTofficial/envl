use std::collections::HashMap;

use crate::misc::variable::{Type, Value};

#[derive(Debug, Clone)]
pub struct Settings {
    pub envl_file_path: Option<String>,
}

#[derive(Debug)]
pub struct Var<T = Type, U = Value> {
    pub v_type: T,
    pub default_value: U,
    pub actions_value: U,
}

pub type Vars = HashMap<String, Var>;

#[derive(Debug)]
pub struct Config {
    pub settings: Settings,
    pub vars: Vars,
}
