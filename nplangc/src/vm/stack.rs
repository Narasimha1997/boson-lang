use crate::isa;
use crate::types::object;
use crate::vm::errors;
use crate::vm::frames;

use crate::config::DATA_STACK_SIZE;
use crate::config::FRAME_STACK_SIZE;

use errors::VMError;
use errors::VMErrorKind;
use frames::ExecutionFrame;
use isa::InstructionKind;
use object::Object;
use std::rc::Rc;

pub struct CallStack {
    pub stack: Vec<Rc<ExecutionFrame>>,
    pub stack_pointer: i64,
    pub max_size: usize,
}

pub struct DataStack {
    pub stack: Vec<Rc<Object>>,
    pub stack_pointer: i64,
    pub max_size: usize,
}

impl CallStack {
    pub fn new() -> CallStack {
        return CallStack {
            stack: vec![],
            stack_pointer: -1,
            max_size: FRAME_STACK_SIZE,
        };
    }

    pub fn push_frame(&mut self, frame: Rc<ExecutionFrame>) -> Result<i64, VMError> {
        if (self.stack_pointer + 1) >= self.max_size as i64 {
            return Err(VMError::new(
                "Stack Overflow!".to_string(),
                VMErrorKind::CallStackOverflow,
                Some(InstructionKind::ICall),
                0,
            ));
        }

        self.stack.push(frame);
        self.stack_pointer += 1;
        return Ok(self.stack_pointer);
    }

    pub fn pop_frame(&mut self) -> Result<Rc<ExecutionFrame>, VMError> {
        if self.stack_pointer == -1 {
            return Err(VMError::new(
                "Stack underflow".to_string(),
                VMErrorKind::CallStackUnderflow,
                Some(InstructionKind::IRet),
                0,
            ));
        }

        let popped = self.stack.pop();
        self.stack_pointer -= 1;

        return Ok(popped.unwrap());
    }

    pub fn get_top(&self) -> i64 {
        return self.stack_pointer;
    }
}

impl DataStack {
    pub fn new() -> DataStack {
        return DataStack {
            stack: vec![],
            stack_pointer: -1,
            max_size: DATA_STACK_SIZE,
        };
    }

    pub fn push_object(&mut self, obj: Rc<Object>, inst: InstructionKind) -> Result<i64, VMError> {
        if (self.stack_pointer + 1) >= self.max_size as i64 {
            return Err(VMError::new(
                "Stack Overflow!".to_string(),
                VMErrorKind::DataStackOverflow,
                Some(inst),
                0,
            ));
        }

        self.stack.push(obj);
        self.stack_pointer += 1;
        return Ok(self.stack_pointer);
    }

    pub fn pop_object(&mut self, inst: InstructionKind) -> Result<Rc<Object>, VMError> {
        if self.stack_pointer == -1 {
            return Err(VMError::new(
                "Stack underflow".to_string(),
                VMErrorKind::DataStackUnderflow,
                Some(inst),
                0,
            ));
        }

        let popped = self.stack.pop();
        self.stack_pointer -= 1;
        return Ok(popped.unwrap());
    }
}
