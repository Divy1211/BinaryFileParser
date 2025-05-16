use std::collections::HashMap;
use pyo3::exceptions::PyKeyError;
use pyo3::PyResult;
use crate::types::parseable_type::ParseableType;

pub struct IfTracker {
    ifs_entered: usize,
    ifs_run: usize,
    break_flag: bool,
}

impl IfTracker {
    pub fn new() -> IfTracker {
        IfTracker {
            ifs_entered: 0,
            ifs_run: 0,
            break_flag: false,
        }
    }
}

pub struct Context {
    pub idxes: Vec<usize>,
    pub keys: HashMap<String, ParseableType>,
    pub if_tracker: Option<IfTracker>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            idxes: vec![],
            keys: HashMap::new(),
            if_tracker: None,
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

    pub fn enter_if(&mut self) {
        self.if_tracker.as_mut().map(|tracker| tracker.ifs_entered += 1);
    }

    pub fn run_if(&mut self) {
        self.if_tracker.as_mut().map(|tracker| tracker.ifs_run += 1);
    }

    pub fn break_if(&mut self) {
        self.if_tracker.as_mut().map(|tracker| tracker.break_flag = true);
    }
    
    pub fn do_break(&self) -> bool {
        self.if_tracker.as_ref().map(|tracker| {
            tracker.ifs_entered == tracker.ifs_run || tracker.break_flag
        }).unwrap_or(false)
    }
}