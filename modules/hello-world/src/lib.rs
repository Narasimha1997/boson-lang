extern crate boson;

use std::rc::Rc;
use boson::types::{object::Object, dyn_module::DynamicModuleResult};

pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    println!("called internal sub-module");
    Ok(Rc::new(Object::Str(format!("called hello world!"))))
}