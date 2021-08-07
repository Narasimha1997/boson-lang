use crate::api::BosonLang;
use crate::types::object::Object;

use std::env::args;

fn info() {
    println!("boson-eval v0.1.0")
    println!("Boson is an educational general purpose programming language written in Rust.");
    println!("This binary evaluates program files. Usage: boson-eval file-name");   
}

pub fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        info();
    }

    let f_name = args[1];
    if f_name == "help" {
        info();
    }

    
}