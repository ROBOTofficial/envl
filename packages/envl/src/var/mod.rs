use std::collections::HashMap;

use envl_config::misc::variable::{Type, Value};
use envl_vars::misc::variable::VariableValue;

use crate::misc::error::{convert_envl_lib_error, EnvlError, EnvlLibError};

pub fn parse_var(t: Type, v: VariableValue) -> Result<Value, Box<EnvlError>> {
    match &t {
        Type::Null => {
            return Ok(Value::Null);
        }
        Type::String => {
            if let VariableValue::String(value) = &v {
                return Ok(Value::String(value.clone()));
            }
        }
        Type::Char => {
            if let VariableValue::Char(c) = &v {
                return Ok(Value::Char(*c));
            }
        }
        Type::Float => {
            if let VariableValue::Number(n) = &v {
                if let Ok(f) = n.parse::<f64>() {
                    return Ok(Value::Float(f));
                }
            }
        }
        Type::Int => {
            if let VariableValue::Number(n) = &v {
                if let Ok(i) = n.parse::<i64>() {
                    return Ok(Value::Int(i));
                }
            }
        }
        Type::Uint => {
            if let VariableValue::Number(n) = &v {
                if let Ok(u) = n.parse::<u64>() {
                    return Ok(Value::Uint(u));
                }
            }
        }
        Type::Bool => {
            if let VariableValue::Bool(b) = &v {
                return Ok(Value::Bool(b.to_owned()));
            }
        }
        Type::Option(t) => {
            return match parse_var(*t.to_owned(), v) {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            };
        }
        Type::Array(boxed_type) => {
            if let VariableValue::Array(elements) = &v {
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
        }
        Type::Struct(elements) => {
            if let VariableValue::Struct(vars) = &v {
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
                        dbg!(t.to_owned(), v.to_owned());
                        return Err(Box::from(convert_envl_lib_error(EnvlLibError {
                            message: "Invalid type".to_string(),
                        })));
                    }
                }

                return Ok(Value::Struct(hm));
            }
        }
    }

    Err(Box::from(convert_envl_lib_error(EnvlLibError {
        message: "Invalid type".to_string(),
    })))
}
