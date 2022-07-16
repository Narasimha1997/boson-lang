extern crate libloading;

use std::collections::HashMap;

pub type FFIError = String;

pub struct BosonFFI {
    objects_tracker: usize,
    lib_table: HashMap<usize, String>,
}

impl BosonFFI {
    pub fn empty() -> Self {
        BosonFFI {
            objects_tracker: 0,
            lib_table: HashMap::new(),
        }
    }

    pub fn load_dynlib(&self) -> Result<usize, FFIError> {
        Ok(0)
    }

    pub fn unload_dynlib(&self, _descriptor: usize) -> Result<(), FFIError> {
        Ok(())
    }

    pub fn call_function(&self, _descriptor: usize, _name: String) -> Result<(), FFIError> {
        Ok(())
    }
}
