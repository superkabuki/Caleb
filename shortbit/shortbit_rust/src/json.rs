//! src/json.rs

use std::collections::BTreeMap;

/// `Number` types for json conversion
#[derive(Clone)]
pub enum Number {
    Float(f64),
    Byte(u8),
    Int(u64),
    SIng(i64),
}

impl Number {
    fn format(&self) -> String {
        use Number::*;

        // TODO: This is verbose. Make a macro?
        match self {
            Float(n) => format!("{}", n),
            Byte(n) => format!("{}", n),
            Int(n) => format!("{}", n),
            SIng(n) => format!("{}", n),
        }
    }
}

/// recursive clean json enum
#[derive(Clone)]
pub enum CleanJson {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<CleanJson>),
    Object(BTreeMap<String, CleanJson>),
    Null,
}

/// takes in `kv_clean`ed SCTE-35 struct data, converts
/// to a string of valid json data
pub fn to_json(cj: BTreeMap<String, CleanJson>) -> String {
    /// formats `CleanJson` into valid json data
    fn transform(v: CleanJson) -> String {
        match v {
            CleanJson::Bool(b) => format!("{}", b),
            CleanJson::Number(n) => n.format(),
            CleanJson::String(s) => format!(r#""{}""#, s),
            CleanJson::Array(a) => format!("[{}]", {
                let mut arr_res: Vec<String> = Vec::new();
                for arr_v in a {
                    let arr_value: String = transform(arr_v);
                    if arr_value.len() > 0 {
                        arr_res.push(arr_value);
                    }
                }
                arr_res.join(",")
            }),
            CleanJson::Object(o) => to_json(o),
            _ => String::from(""),
        }
    }

    let mut res: Vec<String> = Vec::new();
    for (key, bv) in cj {
        let value: String = transform(bv);
        if value.len() > 0 {
            res.push(format!(r#""{}":{}"#, key, value));
        }
    }
    format!(r#"{{{}}}"#, res.join(","))
}

/// takes in valid json, returns `pretty print` json data
pub fn to_pretty(_json: String, _indent: u8) -> String {
    todo!()
}
