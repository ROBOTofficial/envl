use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Null,
    String,
    Char,
    Float,
    Int,
    Uint,
    Bool,
    Array(Box<VarType>),
    Struct(HashMap<String, VarType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub file_path: String,
    pub row: usize,
    pub col: usize,
}
