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
}

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
}
