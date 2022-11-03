extern crate boson;
extern crate syscall_numbers;

mod platform;

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

#[inline(always)]
unsafe fn syscall_raw(no: usize, params: &[Rc<Object>]) -> Result<Rc<Object>, String> {
    let mut usize_pointers = vec![];

    for param in params.iter() {
        match param.as_ref() {
            Object::Str(str) => usize_pointers.push(str.as_ptr() as usize),
            Object::ByteBuffer(buffer) => {
                usize_pointers.push(buffer.borrow().data.as_ptr() as usize);
            }
            Object::Int(i) => {
                usize_pointers.push(*i as usize);
            }
            _ => {
                return Err(format!(
                    "value of type {} cannot be converted into a syscall compatible parameter",
                    param.get_type()
                ))
            }
        }
    }

    let syscall_result = match usize_pointers.len() {
        0 => platform::syscall0(no),
        1 => platform::syscall1(no, &usize_pointers),
        2 => platform::syscall2(no, &usize_pointers),
        3 => platform::syscall3(no, &usize_pointers),
        4 => platform::syscall4(no, &usize_pointers),
        5 => platform::syscall5(no, &usize_pointers),
        6 => platform::syscall6(no, &usize_pointers),
        _ => {
            return Err(format!(
                "max supported system call arguments are 6, but got {}",
                usize_pointers.len()
            ))
        }
    };

    Ok(Rc::new(Object::Int(syscall_result as i64)))
}

#[no_mangle]
pub fn exec(method: String, params: &Vec<Rc<Object>>) -> DynamicModuleResult {
    match method.as_ref() {
        "get_syscalls" => return Ok(get_syscalls_as_map()),
        "call" => {
            if params.len() == 0 {
                return Err(DynamicModuleInternalError {
                    cause: "no_method".to_string(),
                    message: format!("call() accepts at least one parameter"),
                });
            }

            match params[0].as_ref() {
                Object::Int(i) => unsafe {
                    let result = syscall_raw(*i as usize, &params[1..]);
                    if result.is_ok() {
                        return Ok(result.unwrap());
                    } else {
                        return Err(DynamicModuleInternalError {
                            cause: "internal_err".to_string(),
                            message: result.unwrap_err(),
                        });
                    }
                },
                _ => {
                    return Err(DynamicModuleInternalError {
                        cause: "invalid_type".to_string(),
                        message: format!(
                            "call() accepts first parameter of type int but got {}",
                            params[0].get_type()
                        ),
                    })
                }
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
