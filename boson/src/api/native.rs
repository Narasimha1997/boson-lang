use crate::types::object;

use std::rc::Rc;

use object::Object;

/*
    Contains all the implementation of native built-ins
*/

pub fn print(st: &String) {
    print!("{}", st);
}

pub fn println(st: &String) {
    println!("{}", st);
}

pub fn exec(args: &Vec<Rc<Object>>) -> Result<(i32, Vec<u8>), String> {
    
}