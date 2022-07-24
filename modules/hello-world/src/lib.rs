extern crate boson;

use boson::types::{dyn_module::DynamicModuleResult, object::Object};
use std::rc::Rc;

#[no_mangle]
pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (open)");
    Ok(Rc::new(Object::Str(format!("called open()"))))
}

#[no_mangle]
pub fn close(_close_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (close)");
    Ok(Rc::new(Object::Str(format!("called close()"))))
}

#[no_mangle]
pub fn write(_write_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (write)");
    Ok(Rc::new(Object::Str(format!("called write()"))))
}

#[no_mangle]
pub fn read(_read_params: Rc<Object>) -> DynamicModuleResult {
    println!("hello world (read)");
    Ok(Rc::new(Object::Str(format!("called read()"))))
}
