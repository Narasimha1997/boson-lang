use crate::vm::frames;
use crate::types::object;

use std::rc::Rc;
use frames::ExecutionFrame;
use object::Object;

pub struct CallStack {
    pub stack: Vec<ExecutionFrame>,
    pub current_index: usize,
    pub max_size: usize,
}

pub struct DataStack {
    pub stack: Vec<Rc<Object>>,
    pub stack_pointer: usize,
    pub max_size: usize,
}