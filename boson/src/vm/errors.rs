use crate::isa;
use crate::vm::frames::ExecutionFrame;

use std::rc::Rc;

use isa::InstructionKind;

// VM runtime Error
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum VMErrorKind {
    VMPanic,
    IPOutOfBounds,
    SPOutOfBounds,
    CallStackOverflow,
    DataStackOverflow,
    InvalidGlobalIndex,
    StackCorruption,
    IllegalOperation,
    InvalidOperandTypes,
    CallStackUnderflow,
    DataStackUnderflow,
    GlobalPoolSizeExceeded,
    InstructionNotImplemented,
    DivideByZeroError,
    UnresolvedBuiltinFunction,
    BuiltinFunctionError,
    TypeError,
    UnknownFreeVariable,
    FunctionArgumentsError,
    AssertionError,
    IndexError,
    OverflowError,
    IterationError,
    ThreadKillError,
    ThreadCreateError,
    ThreadWaitError,
}

#[derive(Debug, Clone)]
pub struct VMError {
    pub message: String,
    pub t: VMErrorKind,
    pub instruction: Option<InstructionKind>,
    pub pos: usize,
}

impl VMError {
    pub fn new(
        message: String,
        t: VMErrorKind,
        instruction: Option<InstructionKind>,
        pos: usize,
    ) -> VMError {
        return VMError {
            message: message,
            t: t,
            instruction: instruction,
            pos: pos,
        };
    }

    pub fn new_from_isa_error(isa_error: &ISAError, inst: InstructionKind) -> VMError {
        match isa_error.t {
            ISAErrorKind::TypeError => {
                return VMError {
                    message: isa_error.message.clone(),
                    t: VMErrorKind::TypeError,
                    instruction: Some(inst),
                    pos: 0,
                };
            }
            ISAErrorKind::OverflowError => {
                return VMError {
                    message: isa_error.message.clone(),
                    t: VMErrorKind::OverflowError,
                    instruction: Some(inst),
                    pos: 0,
                };
            }
            ISAErrorKind::DivideByZeroError => {
                return VMError {
                    message: isa_error.message.clone(),
                    t: VMErrorKind::DivideByZeroError,
                    instruction: Some(inst),
                    pos: 0,
                };
            }
            _ => {
                return VMError {
                    message: isa_error.message.clone(),
                    t: VMErrorKind::IllegalOperation,
                    instruction: Some(inst),
                    pos: 0,
                };
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ISAErrorKind {
    DivideByZeroError,
    OverflowError,
    TypeError,
    InvalidOperation,
}

#[derive(Debug)]
pub struct ISAError {
    pub message: String,
    pub t: ISAErrorKind,
}

impl ISAError {
    pub fn new(msg: String, t: ISAErrorKind) -> ISAError {
        return ISAError { message: msg, t: t };
    }
}

pub type StackFrame = Rc<ExecutionFrame>; // state of each frame when exception occured
