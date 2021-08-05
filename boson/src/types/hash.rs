use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use std::vec::Vec;

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
