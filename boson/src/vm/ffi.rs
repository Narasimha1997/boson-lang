extern crate libloading;

use std::collections::HashMap;

pub type FFIError = String;

pub struct BosonFFI {
    objects_tracker: usize,
    lib_table: HashMap<usize, libloading::Library>,
}

impl BosonFFI {
    pub fn empty() -> Self {
        BosonFFI {
            objects_tracker: 0,
            lib_table: HashMap::new(),
        }
    }

    pub fn load_dynlib(&mut self, path: String) -> Result<usize, FFIError> {
        unsafe {
            let handle_result = libloading::Library::new(path.clone());
            if handle_result.is_err() {
                return Err(format!(
                    "failed to load dynlib {}, error={}",
                    path,
                    handle_result.unwrap_err()
                ));
            }

            let handle = handle_result.unwrap();
            self.lib_table.insert(self.objects_tracker, handle);
            let current_fd = self.objects_tracker;
            self.objects_tracker += 1;
            Ok(current_fd)
        }
    }

    pub fn unload_dynlib(&self, descriptor: usize) -> Result<(), FFIError> {
        let handle_opt = self.lib_table.get(&descriptor);
        if handle_opt.is_none() {
            return Err(format!("handle {} is not loaded", descriptor));
        }

        let _handle = handle_opt.unwrap();
        return Ok(());
    }

    pub fn call_function(&self, _descriptor: usize, _name: String) -> Result<(), FFIError> {
        Ok(())
    }
}
