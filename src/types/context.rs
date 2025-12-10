use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::{pyclass, pymethods, Bound, PyResult};
use pyo3::prelude::{PyAnyMethods, PyTupleMethods};
use pyo3::types::{PyDict, PyTuple};
use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;

pub struct IfTracker {
    ifs_entered: usize,
    ifs_run: usize,
    break_flag: bool,
}

impl Default for IfTracker {
    fn default() -> Self {
        Self::new()
    }
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

#[pyclass(name = "Context")]
#[derive(Clone)]
pub struct ContextPtr {
    pub inner: Arc<RwLock<Context>>,
}

impl Default for ContextPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextPtr {
    pub fn new() -> ContextPtr {
        ContextPtr {
            inner: Arc::new(RwLock::new(Context::new())),
        }
    }
    pub fn from(ctx: Context) -> ContextPtr {
        ContextPtr {
            inner: Arc::new(RwLock::new(ctx)),
        }
    }
}

#[pymethods]
impl ContextPtr {
    #[new]
    #[pyo3(signature = (**keys))]
    pub fn new_py(keys: Option<&Bound<'_, PyDict>>) -> PyResult<ContextPtr> {
        let Some(keys) = keys else {
            return Ok(ContextPtr::new());
        };
        let mut ctx = Context::new();
        for (key, value) in keys {
            let key = key.extract::<String>().expect("kwarg");
            let value = value.cast::<PyTuple>()?;
            if <Bound<PyTuple> as PyTupleMethods>::len(value) != 2 {
                return Err(PyValueError::new_err(format!(
                    "Could not create key from argument '{}'. Context keys must be a (data_type, value) pair",
                    key
                )))
            }
            let (data_type, item) = unsafe {
                (value.get_item_unchecked(0), value.get_item_unchecked(1))   
            };
            let data_type = data_type.extract::<BfpType>()?;
            let item = data_type.to_parseable(&item)?;
            
            ctx.set(&key, item);
        }
        Ok(ContextPtr::from(ctx))
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
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
        if let Some(tracker) = self.if_tracker.as_mut() {
            tracker.ifs_entered += 1;
        }
    }

    pub fn run_if(&mut self) {
        if let Some(tracker) = self.if_tracker.as_mut() {
            tracker.ifs_run += 1
        }
    }

    pub fn break_if(&mut self) {
        if let Some(tracker) = self.if_tracker.as_mut() {
            tracker.break_flag = true
        }
    }
    
    pub fn do_break(&self) -> bool {
        self.if_tracker.as_ref().map(|tracker| {
            tracker.ifs_entered == tracker.ifs_run || tracker.break_flag
        }).unwrap_or(false)
    }
}