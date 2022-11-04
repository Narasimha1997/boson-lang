use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::vec::Vec;

use crate::types::object::Object;
use crate::types::subroutine::Subroutine;

#[derive(Clone, Debug, PartialOrd)]
pub struct ClosureContext {
    pub compiled_fn: Rc<Subroutine>,
    pub free_objects: Vec<Rc<Object>>,
    pub bytecode_size: usize,
}

impl ClosureContext {
    pub fn describe(&self) -> String {
        return self.compiled_fn.describe();
    }

    pub fn get_objects(&self) -> &Vec<Rc<Object>> {
        return &self.free_objects;
    }
}

impl PartialEq for ClosureContext {
    fn eq(&self, other: &ClosureContext) -> bool {
        return self.compiled_fn == other.compiled_fn;
    }
}

impl Eq for ClosureContext {}

impl Hash for ClosureContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.compiled_fn.hash(state);
    }
}
