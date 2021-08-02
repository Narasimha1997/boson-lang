use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::types::object::Object;

#[derive(Clone, Debug)]
pub struct Array {
    pub name: String,
    pub elements: Vec<Rc<Object>>,
}

impl Array {
    pub fn describe(&self) -> String {
        let values: Vec<String> = (&self.elements)
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        return format!("Array([{}])", values.join(", "));
    }

    pub fn get_values_ref(&self) -> &Vec<Rc<Object>> {
        return &self.elements;
    }

    pub fn get_values(&self) -> Vec<Rc<Object>> {
        return self.elements.clone();
    }

    pub fn get_sliced(&self, start: usize, end: usize) -> Vec<Rc<Object>> {
        return self.elements[start..end].to_vec();
    }

    pub fn set_object(&mut self, pos: usize, obj: Rc<Object>) -> Option<String> {
        if pos >= self.elements.len() {
            return Some(format!("Array index out of range for position {}", pos));
        }

        self.elements[pos] = obj;
        return None;
    }

    pub fn get_object(&self, pos: usize) -> Result<Rc<Object>, String> {
        if pos >= self.elements.len() {
            return Err(format!("Array index out of range for position {}", pos));
        }

        return Ok(self.elements[pos].clone());
    }

    pub fn get_object_ref(&self, pos: usize) -> Result<&Rc<Object>, String> {
        if pos >= self.elements.len() {
            return Err(format!("Array index out of range for position {}", pos));
        }

        return Ok(&self.elements[pos]);
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Array) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }

        for idx in 0..self.elements.len() {
            if self.elements[idx] != other.elements[idx] {
                return false;
            }
        }

        return true;
    }
}

/*
    Arrays are hashed based on their names
*/

impl Hash for Array {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
