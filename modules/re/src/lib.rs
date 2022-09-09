extern crate boson;
extern crate once_cell;
extern crate regex;

use once_cell::sync::OnceCell;
use regex::Regex;

use std::sync::RwLock;
use std::{cell::RefCell, collections::HashMap};

use boson::types::{
    array::Array, dyn_module::DynamicModuleInternalError, dyn_module::DynamicModuleResult,
    object::Object,
};
use std::rc::Rc;

pub struct RegexTable {
    pub table: HashMap<usize, Regex>,
    pub counter: usize,
}

impl RegexTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            counter: 0,
        }
    }
}

static REGEX_TABLE: OnceCell<RwLock<RegexTable>> = OnceCell::new();

#[inline(always)]
fn add_entry_into_table(expression: String) -> Result<Rc<Object>, DynamicModuleInternalError> {
    let table = REGEX_TABLE.get();
    if let Some(rw_mutex) = table {
        if let Ok(mut write_handle) = rw_mutex.write() {
            let regex_handle_res = Regex::new(&expression);
            if regex_handle_res.is_err() {
                return Err(DynamicModuleInternalError {
                    cause: "compile_err".to_string(),
                    message: "failed to create regex entry".to_string(),
                });
            }

            let regex_handle = regex_handle_res.unwrap();
            let counter = write_handle.counter;
            write_handle.table.insert(counter, regex_handle);
            write_handle.counter += 1;
            return Ok(Rc::new(Object::Int(counter as i64)));
        }
    }

    return Err(DynamicModuleInternalError {
        cause: "compile_err".to_string(),
        message: "failed to create regex entry".to_string(),
    });
}

#[inline(always)]
fn extract_matches(
    counter: usize,
    target_string: &str,
) -> Result<Rc<Object>, DynamicModuleInternalError> {
    let table = REGEX_TABLE.get();
    if let Some(rw_mutex) = table {
        if let Ok(read_handle) = rw_mutex.read() {
            let regex_opt = read_handle.table.get(&counter);
            if regex_opt.is_none() {
                return Err(DynamicModuleInternalError {
                    cause: "internal_err".to_string(),
                    message: format!("no entry in the regex table found for {}", counter),
                });
            }

            let regex = regex_opt.unwrap();
            let mut matches = vec![];
            for matched in regex.find_iter(&target_string) {
                let start = matched.start();
                let end = matched.end();
                matches.push(Rc::new(Object::Str(target_string[start..end].to_string())))
            }

            return Ok(Rc::new(Object::Array(RefCell::new(Array {
                name: "todo".to_string(),
                elements: matches,
            }))));
        }
    }

    return Err(DynamicModuleInternalError {
        cause: "internal_err".to_string(),
        message: "failed to extract patterns".to_string(),
    });
}

// open and close functions
#[no_mangle]
pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    // init table
    let table = RegexTable::new();
    if let Err(_) = REGEX_TABLE.set(RwLock::new(table)) {
        return Err(DynamicModuleInternalError {
            cause: "init_error".to_string(),
            message: "failed to init internal regex table".to_string(),
        });
    }

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

#[no_mangle]
pub fn exec(method: String, params: &Vec<Rc<Object>>) -> DynamicModuleResult {
    match method.as_ref() {
        "compile" => {
            if params.len() != 1 {
                return Err(DynamicModuleInternalError {
                    cause: "no_method".to_string(),
                    message: format!("compile() expects 1 parameters, but got {}", params.len()),
                });
            }

            let expression = params[0].as_ref().describe();
            return add_entry_into_table(expression.to_string());
        }
        "get_matches" => {
            if params.len() != 2 {
                return Err(DynamicModuleInternalError {
                    cause: "no_method".to_string(),
                    message: format!(
                        "get_matches() expects 2 parameters, but got {}",
                        params.len()
                    ),
                });
            }

            match (params[0].as_ref(), params[1].as_ref()) {
                (Object::Int(counter), Object::Str(target)) => {
                    return extract_matches(*counter as usize, &target)
                }
                _ => return Err(DynamicModuleInternalError {
                    cause: "invalid_type".to_string(),
                    message: format!(
                        "get_matches expects parameters of type int and string, but got {} and {}",
                        params[0].as_ref().get_type(),
                        params[1].as_ref().get_type()
                    ),
                }),
            }
        }
        _ => {
            return Err(DynamicModuleInternalError {
                cause: "no_method".to_string(),
                message: format!("method {} not found", method),
            })
        }
    }
}
