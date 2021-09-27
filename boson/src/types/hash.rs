use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use std::vec::Vec;

use crate::types::array::Array;
use crate::types::object::Object;

#[derive(Clone, Debug)]
pub struct HashTable {
    pub name: String,
    pub entries: HashMap<Rc<Object>, Rc<Object>>,
}

impl HashTable {
    pub fn describe(&self) -> String {
        let combined_pairs: Vec<String> = (&self.entries)
            .into_iter()
            .map(|(key, value)| format!("{}: {}", key.describe(), value.describe()))
            .collect();

        return format!("HashTable({{{}}})", combined_pairs.join(", "));
    }

    pub fn keys(&self) -> Vec<Rc<Object>> {
        return Vec::from_iter(self.entries.keys().cloned());
    }

    pub fn set(&mut self, key: Rc<Object>, value: Rc<Object>) {
        self.entries.insert(key, value);
    }

    pub fn values(&self) -> Vec<Rc<Object>> {
        return Vec::from_iter(self.entries.values().cloned());
    }

    pub fn length(&self) -> usize {
        return self.entries.len();
    }

    pub fn get(&self, key: &Rc<Object>) -> Result<Rc<Object>, String> {
        let result = self.entries.get(key);
        if result.is_none() {
            return Err(format!("Key {} not found.", key.describe()));
        }

        return Ok(result.unwrap().clone());
    }

    pub fn get_ref(&self, key: &Rc<Object>) -> Result<&Rc<Object>, String> {
        let result = self.entries.get(key);
        if result.is_none() {
            return Err(format!("Key {} not found.", key.describe()));
        }

        return Ok(result.unwrap());
    }

    // attrs:
    pub fn attrs(&self) -> Vec<Rc<Object>> {
        return vec![
            Rc::new(Object::Str(String::from("keys"))),
            Rc::new(Object::Str(String::from("__name__"))),
            Rc::new(Object::Str(String::from("values"))),
        ];
    }

    pub fn get_attribute(&self, key: &String) -> Result<Rc<Object>, String> {
        match key.as_ref() {
            "__name__" => return Ok(Rc::new(Object::Str(self.name.clone()))),
            _ => {
                return Err(format!(
                    "Attribute {} not found for type {}",
                    key, "HashTable"
                ));
            }
        }
    }

    pub fn call_attribute(
        &mut self,
        key: &String,
        args: &Vec<Rc<Object>>,
    ) -> Result<Rc<Object>, String> {
        match key.as_ref() {
            "keys" => {
                if args.len() != 0 {
                    return Err(format!(
                        "keys() takes zero arguments, provided {}.",
                        args.len()
                    ));
                }

                let key_array = self.keys();
                let array_obj = Array {
                    elements: key_array,
                    name: self.name.clone(),
                };

                return Ok(Rc::new(Object::Array(RefCell::new(array_obj))));
            }
            "values" => {
                if args.len() != 0 {
                    return Err(format!(
                        "keys() takes zero arguments, provided {}.",
                        args.len()
                    ));
                }

                let key_array = self.values();
                let array_obj = Array {
                    elements: key_array,
                    name: self.name.clone(),
                };

                return Ok(Rc::new(Object::Array(RefCell::new(array_obj))));
            }
            _ => {
                return Err(format!(
                    "Attribute {} not found for type {}",
                    key, "HashTable"
                ));
            }
        }
    }
}

impl Hash for HashTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for HashTable {
    fn eq(&self, other: &HashTable) -> bool {
        self.name == other.name
    }
}

impl Eq for HashTable {}

impl fmt::Display for HashTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
