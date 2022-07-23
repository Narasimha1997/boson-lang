extern crate libloading;

use crate::types::{dyn_module, object::Object};

use dyn_module::{
    CloseFunctionSymbol, DynamicModuleResult, OpenFunctionSymbol, ReadFunctionSymbol,
    WriteFunctionSymbol,
};
use std::collections::HashMap;
use std::rc::Rc;

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

    pub fn load_dynlib(
        &mut self,
        path: String,
        params: Rc<Object>,
    ) -> Result<(usize, DynamicModuleResult), FFIError> {
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

            let open_symbol_res: Result<libloading::Symbol<OpenFunctionSymbol>, libloading::Error> =
                handle.get(b"open");
            if open_symbol_res.is_err() {
                return Err(format!("failed to open dynamic module {}", path));
            }

            let open_symbol = open_symbol_res.unwrap();
            let open_eval_result = open_symbol(params);
            self.lib_table.insert(self.objects_tracker, handle);
            self.objects_tracker += 1;
            Ok((self.objects_tracker - 1, open_eval_result))
        }
    }

    pub fn unload_dynlib(&self, descriptor: usize, params: Rc<Object>) -> Result<DynamicModuleResult, FFIError> {
        unsafe {
            let handle_opt = self.lib_table.get(&descriptor);
            if handle_opt.is_none() {
                return Err(format!("handle {} is not loaded", descriptor));
            }


            let handle = handle_opt.unwrap();

            let close_symbol_res: Result<libloading::Symbol<CloseFunctionSymbol>, libloading::Error> = handle.get(b"close");
            if close_symbol_res.is_err() {
                return Err(format!("failed to close dynamic module {}", descriptor));
            }

            let close_symbol = close_symbol_res.unwrap();
            let close_result = close_symbol(params);

            Ok(close_result)
        }
    }

    pub fn call_function(&self, _descriptor: usize, _name: String) -> Result<(), FFIError> {
        Ok(())
    }
}
