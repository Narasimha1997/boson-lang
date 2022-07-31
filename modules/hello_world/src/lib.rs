extern crate boson;

use boson::types::{
    dyn_module::DynamicModuleInternalError, dyn_module::DynamicModuleResult, object::Object,
};
use std::rc::Rc;

#[no_mangle]
pub extern "Rust" fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (open)");
    Ok(Rc::new(Object::Str(format!("called open()"))))
}

#[no_mangle]
pub extern "Rust" fn  close(_close_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (close)");
    Ok(Rc::new(Object::Str(format!("called close()"))))
}

#[no_mangle]
pub extern "Rust" fn write(_write_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (write)");
    Ok(Rc::new(Object::Str(format!("called write()"))))
}

#[no_mangle]
pub extern "Rust" fn read(_read_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (read)");
    Ok(Rc::new(Object::Str(format!("called read()"))))
}

#[no_mangle]
pub extern "Rust" fn exec(method: String, params: &Vec<Rc<Object>>) -> DynamicModuleResult {
    match method.as_ref() {
        "greet_me" => {
            if params.len() != 1 {
                return Err(DynamicModuleInternalError {
                    cause: format!("no_param"),
                    message: format!("parameter 'name' not found"),
                });
            }

            let name = params[0].as_ref().describe();
            return Ok(Rc::new(Object::Str(format!("Hello! {}", name))));
        }
        _ => {
            return Err(DynamicModuleInternalError {
                cause: format!("no_method"),
                message: format!("method {} not found", method),
            })
        }
    }
}
