use crate::vm::errors::StackFrame;
use crate::vm::errors::VMErrorKind;

use std::hash::Hash;
use std::hash::Hasher;

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Exception {
    handle_name: String,
    exception_string: String,
    root_error_type: VMErrorKind,
    pub stack_trace: Vec<StackFrame>, // stack trace is represented backwards, 0th element is the current function,
                                  // len - 1 is the root function or usually main.
}

impl Exception {
    pub fn new(
        name: String,
        message: String,
        err_kind: VMErrorKind,
        trace: Vec<StackFrame>,
    ) -> Exception {
        return Exception {
            handle_name: name,
            exception_string: message,
            root_error_type: err_kind,
            stack_trace: trace,
        };
    }

    pub fn describe(&self) -> String {
        return format!("{:?}: {}", self.root_error_type, self.exception_string);
    }
}

// hash and partial equality:
impl PartialEq for Exception {
    fn eq(&self, other: &Exception) -> bool {
        self.handle_name == other.handle_name
    }
}

impl Hash for Exception {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle_name.hash(state);
    }
}

impl PartialOrd for Exception {
    fn partial_cmp(&self, _other: &Exception) -> Option<Ordering> {
        None
    }
}