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
    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexme) {
            self.values.insert(name.lexme, value);
        } else {
            return Err(format!("Failed to assign. There is no variable assigned"));
        }
        Ok(())
    }

    pub fn get(&self, name: Token) -> Result<Value, String> {
        match self.values.get(&name.lexme).clone() {
            Some(val) => Ok(val.clone()),
            None => return Err(format!("Undefined variable")),
        }
    }
}
