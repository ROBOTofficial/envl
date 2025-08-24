use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Char(char),
    Float(f64),
    Int(i64),
    Uint(u64),
    Bool(bool),
    Array(Vec<Value>),
    Struct(HashMap<String, Value>),
}
