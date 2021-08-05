use crate::isa;

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
}

#[derive(Debug)]
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
        if isa_error.t == ISAErrorKind::TypeError {
            return VMError {
                message: isa_error.message.clone(),
                t: VMErrorKind::TypeError,
                instruction: Some(inst),
                pos: 0,
            };
        } else {
            return VMError {
                message: isa_error.message.clone(),
                t: VMErrorKind::DivideByZeroError,
                instruction: Some(inst),
                pos: 0,
            };
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ISAErrorKind {
    DivideByZeroError,
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