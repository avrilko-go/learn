use crate::{KvPair, Value};

pub mod abi;

impl KvPair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(crate::value::Value::String(s.to_string())),
        }
    }
}
