use std::collections::HashMap;

use envl_utils::types::Position;

use crate::misc::variable::{Type, Value};

#[derive(Debug, Clone)]
pub struct Setting<T> {
    pub value: T,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingWithoutPotision<T> {
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub envl_file_path: Option<Setting<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsWithoutPosition {
    pub envl_file_path: Option<SettingWithoutPotision<String>>,
}

#[derive(Debug, Clone)]
pub struct Var<T = Type, U = Value> {
    pub v_type: T,
    pub default_value: U,
    pub actions_value: U,
    pub position: Position,
}

#[derive(Debug, PartialEq)]
pub struct VarWithoutPosition<T = Type, U = Value> {
    pub v_type: T,
    pub default_value: U,
    pub actions_value: U,
}

pub type Vars = HashMap<String, Var>;

pub type VarsWithoutPosition = HashMap<String, VarWithoutPosition>;

#[derive(Debug)]
pub struct Config {
    pub settings: Settings,
    pub vars: Vars,
}

#[derive(Debug, PartialEq)]
pub struct ConfigWithoutPosition {
    pub settings: SettingsWithoutPosition,
    pub vars: VarsWithoutPosition,
}

pub fn remove_setting_position_prop(settings: Settings) -> SettingsWithoutPosition {
    SettingsWithoutPosition {
        envl_file_path: if let Some(setting) = settings.envl_file_path {
            Some(SettingWithoutPotision {
                value: setting.value,
            })
        } else {
            None
        },
    }
}

pub fn remove_position_prop(config: Config) -> ConfigWithoutPosition {
    ConfigWithoutPosition {
        settings: remove_setting_position_prop(config.settings),
        vars: config
            .vars
            .iter()
            .map(|(n, v)| {
                (
                    n.clone(),
                    VarWithoutPosition {
                        v_type: v.v_type.to_owned(),
                        default_value: v.default_value.to_owned(),
                        actions_value: v.actions_value.to_owned(),
                    },
                )
            })
            .collect::<HashMap<String, VarWithoutPosition>>(),
    }
}
