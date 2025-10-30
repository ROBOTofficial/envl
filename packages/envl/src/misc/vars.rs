use std::collections::HashMap;

use envl_utils::types::Position;
use envl_vars::misc::variable::{Variable, VariableValue};

#[derive(Debug, Clone)]
pub struct Value {
    pub value: VariableValue,
    pub position: Position,
}

pub type Variables = HashMap<String, Value>;

pub fn vars_to_hashmap(vars: Vec<Variable>) -> Variables {
    let mut hm = HashMap::new();

    for var in vars {
        hm.insert(
            var.name,
            Value {
                value: var.value,
                position: var.position,
            },
        );
    }

    hm
}
