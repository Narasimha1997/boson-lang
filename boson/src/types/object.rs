use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::types::array::Array;
use crate::types::builtins::BuiltinKind;
use crate::types::closure::ClosureContext;
use crate::types::hash::HashTable;
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
    ClosureContext(Rc<ClosureContext>),
    Array(Rc<Array>),
    HashTable(Rc<HashTable>),
    Builtins(BuiltinKind),
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Int(i) => i.hash(state),
            Object::Bool(b) => b.hash(state),
            Object::Char(c) => c.hash(state),
            Object::Str(st) => st.hash(state),
            Object::Float(f) => f.to_string().hash(state),
            Object::Array(arr) => arr.hash(state),
            Object::HashTable(ht) => ht.hash(state),
            Object::Subroutine(sub) => sub.hash(state),
            Object::ClosureContext(ctx) => ctx.hash(state),
            _ => "undef".hash(state),
        }
    }
}

impl Object {
    pub fn describe(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::Char(c) => c.to_string(),
            Object::Str(st) => st.clone(),
            Object::Float(f) => f.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Array(arr) => arr.describe(),
            Object::HashTable(ht) => ht.describe(),
            Object::Subroutine(sub) => sub.describe(),
            Object::ClosureContext(ctx) => ctx.describe(),
            Object::Builtins(_) => "builtin".to_string(),
            _ => String::from("undef"),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Object::Int(_) => "int".to_string(),
            Object::Char(_) => "char".to_string(),
            Object::Str(_) => "string".to_string(),
            Object::Float(_) => "float".to_string(),
            Object::Bool(_) => "bool".to_string(),
            Object::Array(_) => "array".to_string(),
            Object::HashTable(_) => "hashmap".to_string(),
            Object::Subroutine(_) => "func".to_string(),
            Object::Builtins(_) => "func".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Object::Bool(val) => val.clone(),
            Object::Noval => false,
            Object::Str(str) => *str != "",
            Object::Int(i) => *i != 0,
            Object::Char(c) => *c != '\0',
            Object::Array(a) => a.as_ref().elements.len() != 0,
            Object::HashTable(h) => h.as_ref().entries.len() != 0,
            _ => true,
        }
    }

    pub fn get_indexed(&self, idx: &Rc<Object>) -> Result<Rc<Object>, String> {
        match (self, idx.as_ref()) {
            (Object::Array(arr), Object::Int(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }
                let result = arr.get_object(*i as usize);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Object::HashTable(ht), _) => {
                let result = ht.get(idx);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Object::Str(st), Object::Int(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }

                let ch = st.chars().nth(*i as usize);
                if ch.is_none() {
                    return Err(format!("String \"{}\" index out of bounds for {}", st, i));
                }

                return Ok(Rc::new(Object::Char(ch.unwrap())));
            }
            _ => {
                return Err(format!(
                    "Object of type {} does not support indexing of type {}",
                    self.get_type(),
                    idx.get_type()
                ));
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
