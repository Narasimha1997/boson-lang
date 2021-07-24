use std::rc::Rc;
use std::vec::Vec;
use std::hash::{Hash, Hasher};

use crate::types::subroutine::Subroutine;
use crate::types::object::Object;


#[derive(Clone, Debug)]
pub struct ClosureContext {
    pub compiled_fn: Rc<Subroutine>,
    pub free_objects: Vec<Rc<Object>>
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

