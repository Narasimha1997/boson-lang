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

#[test]
pub fn indexing() {
    // check indexing - GET and SET
    let array = types::array::Array {
        elements: vec![Rc::new(Object::Int(10))],
        name: "test".to_string(),
    };

    let mut arr_object = Object::Array(RefCell::new(array));
    let result = arr_object.get_indexed(&Rc::new(Object::Int(0)));
    assert_eq!(result.is_ok(), true);
    assert_eq!(*result.unwrap().as_ref(), Object::Int(10));

    // out of bounds:
    let result = arr_object.get_indexed(&Rc::new(Object::Int(1)));
    assert_eq!(result.is_err(), true);

    // set indexed:
    let result = arr_object.set_indexed(&Rc::new(Object::Int(0)), Rc::new(Object::Int(20)));
    assert_eq!(result.is_none(), true);

    // set indexed out of bounds:
    let result = arr_object.set_indexed(&Rc::new(Object::Int(1)), Rc::new(Object::Int(20)));
    assert_eq!(result.is_some(), true);

    // hash map set and get operations:
    let hm = types::hash::HashTable {
        entries: HashMap::new(),
        name: "test".to_string(),
    };

    let mut h_obj = Object::HashTable(RefCell::new(hm));

    // set:
    let result = h_obj.set_indexed(
        &Rc::new(Object::Str("Age".to_string())),
        Rc::new(Object::Int(23)),
    );
    assert_eq!(result.is_none(), true);

    // get
    let result = h_obj.get_indexed(&Rc::new(Object::Str("Age".to_string())));
    assert_eq!(result.is_ok(), true);
    assert_eq!(*result.unwrap().as_ref(), Object::Int(23));

    // get key error
    let result = h_obj.get_indexed(&Rc::new(Object::Str("NotAge".to_string())));
    assert_eq!(result.is_err(), true);

    let string_obj = Object::Str("Prasanna".to_string());
    let result = string_obj.get_indexed(&Rc::new(Object::Int(3)));
    assert_eq!(result.is_ok(), true);
    assert_eq!(*result.unwrap().as_ref(), Object::Char('s'));

    // out of bounds:
    let result = string_obj.get_indexed(&Rc::new(Object::Int(30)));
    assert_eq!(result.is_ok(), false);

    // unsupported object
    assert_eq!(
        Object::Int(20)
            .get_indexed(&Rc::new(Object::Int(30)))
            .is_err(),
        true
    );
}

#[test]
pub fn object_equality() {
    assert_eq!(Object::Int(20) == Object::Int(20), true);

    assert_eq!(Object::Int(20) != Object::Int(25), true);

    assert_eq!(Object::Float(3.144) == Object::Float(3.144), true);

    assert_eq!(Object::Float(3.144) != Object::Float(3.140), true);

    assert_eq!(
        Object::Str(String::from("Hey String!")) == Object::Str(String::from("Hey String!")),
        true
    );

    assert_eq!(
        Object::Str(String::from("Hey String!")) != Object::Str(String::from("Heyy String!")),
        true
    );

    // arrays:
    assert_eq!(
        Object::Array(RefCell::new(types::array::Array {
            elements: vec![Rc::new(Object::Int(10))],
            name: "test".to_string(),
        })) == Object::Array(RefCell::new(types::array::Array {
            elements: vec![Rc::new(Object::Int(10))],
            name: "test2".to_string(),
        })),
        true
    );

    assert_eq!(
        Object::Array(RefCell::new(types::array::Array {
            elements: vec![Rc::new(Object::Int(10))],
            name: "test".to_string(),
        })) != Object::Array(RefCell::new(types::array::Array {
            elements: vec![Rc::new(Object::Int(10)), Rc::new(Object::Int(3))],
            name: "test".to_string(),
        })),
        true
    );
}

#[test]
pub fn builtin_resolution() {
    // check an existing builtin:
    let existing_fn = String::from("println");
    let result = types::builtins::BuiltinKind::get_by_name(&existing_fn);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), types::builtins::BuiltinKind::Println);

    let non_existing_fn = String::from("create_function");
    let result = types::builtins::BuiltinKind::get_by_name(&non_existing_fn);
    assert_eq!(result.is_none(), true);
}
