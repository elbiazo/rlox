use std::collections::HashMap;

use crate::interpreter::Value;
use crate::scanner::Token;

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, &str> {
        match self.values.get(&name.lexme).clone() {
            Some(val) => Ok(val.clone()),
            None => return Err("Undefined variable"),
        }
    }
}
