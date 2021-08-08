use crate::types::object;

use object::Object;
use std::rc::Rc;

/*
    This is a dummy iterator, not a real one.
    Optimal performance not gaurenteed.
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectIterator {
    pub idx: usize,
    pub size: usize,
    pub elements: Vec<Rc<Object>>,
}

impl ObjectIterator {
    pub fn new(obj: Rc<Object>) -> Result<ObjectIterator, String> {
        match obj.as_ref() {
            Object::Array(arr) => {
                return Ok(ObjectIterator {
                    idx: 0,
                    size: arr.borrow().elements.len(),
                    elements: arr.borrow().elements.clone(),
                });
            }
            Object::HashTable(ht) => {
                let table = ht.borrow();
                let length = table.length();
                return Ok(ObjectIterator {
                    idx: 0,
                    size: length,
                    elements: table.keys(),
                });
            }
            Object::Str(st) => {
                let vec_string: Vec<Rc<Object>> =
                    st.chars().map(|c| Rc::new(Object::Char(c))).collect();
                return Ok(ObjectIterator {
                    idx: 0,
                    size: vec_string.len(),
                    elements: vec_string,
                });
            }
            _ => {
                return Err(format!(
                    "Object of type {} does not support iteration.",
                    obj.get_type()
                ));
            }
        }
    }

    pub fn next(&mut self) -> Option<Rc<Object>> {
        if self.idx >= self.size {
            return None;
        }
        let object = self.elements[self.idx];
        self.idx += 1;
        return Some(object);
    }

    pub fn describe(&self) -> String {
        return format!("Iterator(position={} ,size={})", self.idx, self.size);
    }

    pub fn has_next(&self) -> bool {
        return self.idx < self.size;
    }
}
