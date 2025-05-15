use std::collections::HashMap;
use pyo3::exceptions::PyKeyError;
use pyo3::PyResult;
use crate::types::parseable_type::ParseableType;

pub struct Context {
    pub idxes: Vec<usize>,
    pub keys: HashMap<String, ParseableType>
}

impl Context {
    pub fn new() -> Self {
        Self {
            idxes: vec![],
            keys: HashMap::new(),
        }
    }
    
    pub fn get(&self, key: &String) -> PyResult<ParseableType> {
        match self.keys.get(key) {
            None => {
                Err(PyKeyError::new_err(format!("Key with name '{}' was not found in the context", key)))
            }
            Some(val) => {
                Ok(val.clone())
            }
        }
    }
    
    pub fn set(&mut self, key: &String, val: ParseableType) {
        self.keys.insert(key.clone(), val);
    }
}