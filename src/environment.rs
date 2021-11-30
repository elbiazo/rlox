use std::collections::HashMap;
use crate::scanner::Token;
use crate::interpreter::Value;

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap::<String, Value>,
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

    pub fn get(&self, name: Token) -> Option<&Value>{
        self.values.get(&name.lexme).clone()
    }
}