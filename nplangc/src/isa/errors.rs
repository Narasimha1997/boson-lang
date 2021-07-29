

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ISAErrorKind {
    DivideByZeroError,
    TypeError,
}

pub struct ISAError {
   pub message: String,
   pub t: ISAErrorKind
}

impl ISAError {
    pub fn new(msg: String, t: ISAErrorKind) -> ISAError {
        return ISAError{
            message: msg,
            t: t
        };
    }
}