use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::types::subroutine::Subroutine;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Noval,
    Int(i64),
    Bool(bool),
    Char(char),
    Str(String),
    Float(f64),
    Subroutine(Rc<Subroutine>),
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Int(i) => i.hash(state),
            Object::Bool(b) => b.hash(state),
            Object::Char(c) => c.hash(state),
            Object::Str(st) => st.hash(state),
            Object::Float(f) => f.to_string().hash(state),
            _ => "undef".hash(state),
        }
    }
}

impl Object {
    fn describe(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::Char(c) => c.to_string(),
            Object::Str(st) => st.clone(),
            Object::Float(f) => f.to_string(),
            Object::Bool(b) => b.to_string(),
            _ => String::from("undef"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
