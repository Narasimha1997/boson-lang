use std::fmt;

#[derive(Debug)]
pub enum CompilerErrorKind {
    UnresolvedSymbol,
    ConstantAssignment,
    InvalidOperand,
}

pub struct CompileError {
    pub t: CompilerErrorKind,
    pub message: String,
    pub pos: usize,
}

impl CompileError {
    #[allow(dead_code)]
    pub fn new(message: &str, t: CompilerErrorKind, pos: usize) -> CompileError {
        return CompileError {
            message: message.to_string(),
            t: t,
            pos: pos,
        };
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type={:?} Position={} Reason={}",
            self.t, self.pos, self.message
        )
    }
}
