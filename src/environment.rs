use std::collections::HashMap;

use crate::interpreter::Value;
use crate::scanner::Token;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexme) {
            self.values.insert(name.lexme, value);
        } else {
            match &mut self.enclosing {
                Some(env) => {
                    env.assign(name, value)?;
                    return Ok(());
                }
                None => return Err(format!("Failed to assign. There is no variable assigned")),
            }
        }

        Ok(())
    }

    pub fn get(&self, name: Token) -> Result<Value, String> {
        match self.values.get(&name.lexme).clone() {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(enclose_env) => match enclose_env.values.get(&name.lexme).clone() {
                    Some(val) => Ok(val.clone()),
                    None => enclose_env.get(name),
                },
                None => return Err(format!("Undefined variable name {} ", name.lexme)),
            },
        }
    }
}
