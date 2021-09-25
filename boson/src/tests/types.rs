use crate::types;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use types::object::Object;

#[test]
pub fn truthy() {
    // check the truthiness of objects:
    let bool_object_true = Object::Bool(true);
    assert_eq!(bool_object_true.is_true(), true);

    let bool_object_false = Object::Bool(false);
    assert_eq!(bool_object_false.is_true(), false);

    let noval_object = Object::Noval;
    assert_eq!(noval_object.is_true(), false);

    let int_object = Object::Int(10);
    assert_eq!(int_object.is_true(), true);

    let int_object_zeroed = Object::Int(0);
    assert_eq!(int_object_zeroed.is_true(), false);

    let float_object = Object::Float(1.41);
    assert_eq!(float_object.is_true(), true);

    let string_object = Object::Str("Hi!".to_string());
    assert_eq!(string_object.is_true(), true);

    let empty_string = Object::Str("".to_string());
    assert_eq!(empty_string.is_true(), false);
    // array, hash table - filled and empty:
    let mut arr_obj = types::array::Array {
        elements: vec![Rc::new(Object::Int(10))],
        name: "test".to_string(),
    };

    assert_eq!(Object::Array(RefCell::new(arr_obj.clone())).is_true(), true);

    arr_obj.elements = vec![];
    assert_eq!(Object::Array(RefCell::new(arr_obj)).is_true(), false);

    // hash table:
    let mut hash_table = types::hash::HashTable {
        entries: HashMap::new(),
        name: "test".to_string(),
    };

    hash_table.set(Rc::new(Object::Int(10)), Rc::new(Object::Int(20)));
    assert_eq!(
        Object::HashTable(RefCell::new(hash_table.clone())).is_true(),
        true
    );

    hash_table.entries = HashMap::new();
    assert_eq!(Object::HashTable(RefCell::new(hash_table)).is_true(), false);
}
