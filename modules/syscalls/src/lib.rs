extern crate boson;
extern crate syscall_numbers;

use boson::types::{
    dyn_module::DynamicModuleInternalError, dyn_module::DynamicModuleResult, object::Object,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[no_mangle]
pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn close(_close_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn write(_write_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn read(_read_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[inline(always)]
fn get_syscalls_as_map() -> Rc<Object> {
    let all_syscall_names = syscall_numbers::syscall_names();
    let mut hash_map = boson::types::hash::HashTable {
        name: "syscall_names".to_string(),
        entries: HashMap::new(),
    };

    for (idx, syscall_name) in all_syscall_names.iter().enumerate() {
        if *syscall_name == "" {
            hash_map.entries.insert(
                Rc::new(Object::Str(format!("{}_{}", "SYSCALL", idx))),
                Rc::new(Object::Int(idx as i64)),
            );
        } else {
            hash_map.entries.insert(
                Rc::new(Object::Str(syscall_name.to_string().to_uppercase())),
                Rc::new(Object::Int(idx as i64)),
            );
        }
    }

    Rc::new(Object::HashTable(RefCell::new(hash_map)))
}

#[no_mangle]
pub fn exec(method: String, _params: &Vec<Rc<Object>>) -> DynamicModuleResult {
    match method.as_ref() {
        "get_syscalls" => return Ok(get_syscalls_as_map()),
        _ => {
            return Err(DynamicModuleInternalError {
                cause: "no_method".to_string(),
                message: format!("method {} not found", method)
            })
        }
    }
}
