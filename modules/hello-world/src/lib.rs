extern crate boson;

use boson::types::{dyn_module::DynamicModuleResult, object::Object};
use std::rc::Rc;

#[no_mangle]
pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    println!("called internal sub-module");
    Ok(Rc::new(Object::Str(format!("called hello world!"))))
}

#[no_mangle]
pub fn close(_close_params: Rc<Object>) -> DynamicModuleResult {
    println!("closing sub-module");
    Ok(Rc::new(Object::Noval))
}
