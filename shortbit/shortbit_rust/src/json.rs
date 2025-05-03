//! src/json.rs

use std::collections::HashMap;

/// `Number` types for json conversion
#[derive(Clone)]
pub enum Number {
    Float(f64),
    Byte(u8),
    Int(u64),
    SIng(i64),
}

/// recursive clean json enum
#[derive(Clone)]
pub enum CleanJson {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<CleanJson>),
    Object(HashMap<String, CleanJson>),
    Null,
}
