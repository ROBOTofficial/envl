use std::collections::HashMap;

use envl_config::misc::variable::{Type, Value};
use envl_vars::misc::variable::VariableValue;

use crate::misc::error::{convert_envl_lib_error, EnvlError, EnvlLibError};

pub fn parse_var(t: Type, v: VariableValue) -> Result<Value, EnvlError> {
    match &t {
        Type::Null => {
            return Ok(Value::Null);
        }
        Type::String => match &v {
            VariableValue::String(value) => {
                return Ok(Value::String(value.clone()));
            }
            _ => {}
        },
        Type::Char => match &v {
            VariableValue::Char(c) => {
                return Ok(Value::Char(c.clone()));
            }
            _ => {}
        },
        Type::Float => match &v {
            VariableValue::Number(n) => {
                if let Ok(f) = n.parse::<f64>() {
                    return Ok(Value::Float(f));
                }
            }
            _ => {}
        },
        Type::Int => match &v {
            VariableValue::Number(n) => {
                if let Ok(i) = n.parse::<i64>() {
                    return Ok(Value::Int(i));
                }
            }
            _ => {}
        },
        Type::Uint => match &v {
            VariableValue::Number(n) => {
                if let Ok(u) = n.parse::<u64>() {
                    return Ok(Value::Uint(u));
                }
            }
            _ => {}
        },
        Type::Bool => match &v {
            VariableValue::Number(n) => {
                if let Ok(b) = n.parse::<bool>() {
                    return Ok(Value::Bool(b));
                }
            }
            _ => {}
        },
        Type::Option(t) => {
            return match parse_var(*t.to_owned(), v) {
                Ok(value) => Ok(Value::Option(Box::from(value))),
                Err(err) => Err(err),
            };
        }
        Type::Array(boxed_type) => match &v {
            VariableValue::Array(elements) => {
                let element_type = *boxed_type.clone();
                let mut results = Vec::new();

                for element in elements {
                    match parse_var(element_type.clone(), element.clone()) {
                        Ok(e) => {
                            results.push(e);
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                return Ok(Value::Array(results));
            }
            _ => {}
        },
        Type::Struct(elements) => match &v {
            VariableValue::Struct(vars) => {
                let mut hm = HashMap::new();

                for (name, value) in vars {
                    if let Some(t) = elements.get(name) {
                        match parse_var(t.clone(), value.clone()) {
                            Ok(r) => {
                                hm.insert(name.clone(), r);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return Err(convert_envl_lib_error(EnvlLibError {
                            message: "Invalid type".to_string(),
                        }));
                    }
                }

                return Ok(Value::Struct(hm));
            }
            _ => {}
        },
    }

    Err(convert_envl_lib_error(EnvlLibError {
        message: "Invalid type".to_string(),
    }))
}
