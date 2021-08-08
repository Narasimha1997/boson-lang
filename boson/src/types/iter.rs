use crate::types::object;

use object::Object;
use std::rc::Rc;


/*
    This is a draft implementation of iterators,
    This will not give optimal performance yet.
*/

#[derive(Debug, Clone)]
pub struct ObjectIterator<I> {
    pub _iter: I,
}

impl<I> Iterator for ObjectIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self._iter.next() {
            Some(item) => Some(item),
            None => None,
        }
    }
}

impl<I> ObjectIterator<I> {
    pub fn new_wrapped(iter: I) -> ObjectIterator<I> {
        return ObjectIterator { _iter: iter };
    }
}

impl<I> PartialEq for ObjectIterator<I> {
    fn eq(&self, _: &ObjectIterator<I>) -> bool {
        // As of now iterators cannot be compared.
        return false;
    }
}

pub type IterItem = Vec<Rc<Object>>;

pub struct IterType {}

impl IterType {
    pub fn new(obj: Rc<Object>) -> Result<ObjectIterator<IterItem>, String> {
        match obj.as_ref() {
            Object::Array(arr) => {
                let iterator = arr.borrow().elements.clone();
                return Ok(ObjectIterator::new_wrapped(iterator));
            }
            Object::HashTable(ht) => {
                let iterator = ht.borrow().keys();
                return Ok(ObjectIterator::new_wrapped(iterator));
            }
            _ => {
                return Err(format!(
                    "Type {} does not support iteration.",
                    obj.get_type()
                ))
            }
        }
    }
}
