use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Char,
    Float,
    Int,
    Uint,
    Bool,
    Array(Box<Type>),
    Struct(Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Char(char),
    Float(f64),
    Int(i64),
    Uint(u64),
    Bool(bool),
    Array(Vec<Value>),
    Struct(HashMap<String, Value>),
}
