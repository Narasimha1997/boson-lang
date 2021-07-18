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
    fn describe(&self) -> String {
        let values: Vec<String> = (&self.elements)
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        return format!("Array<{}>[{}]", self.name, values.join(", "));
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
