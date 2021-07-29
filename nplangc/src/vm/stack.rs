use crate::vm::frames;
use crate::types::object;

use crate::config::DATA_STACK_SIZE;
use crate::config::FRAME_STACK_SIZE;

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

impl CallStack {
    pub fn new() -> CallStack {
        return CallStack{
            stack: vec![],
            current_index: 0,
            max_size: FRAME_STACK_SIZE
        }
    }
}
