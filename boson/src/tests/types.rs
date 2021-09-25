use crate::types;

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
    assert_eq!(int_object_zeroed.is_true(), true);

    let float_object = Object::Float(1.41);
    assert_eq!(float_object.is_true(), true);
}