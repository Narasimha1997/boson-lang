use boson::api::BosonLang;

use std::env::args;
use std::process;

fn info() {
    println!("boson-compile v0.1.0");
    println!("Boson is an educational general purpose programming language written in Rust.");
    println!("This binary compiles boson program files and saves the bytecode.");
}

pub fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        info();
        return;
    }

    let f_name = &args[1];
    if f_name == "help" {
        info();
        return;
    }

    // run compiler:
    let ret = BosonLang::save_bytecode_from_file(f_name.clone());
    if ret.is_some() {
        println!("Wrote {} bytes.", ret.unwrap());
        process::exit(0);
    }

    process::exit(-1);
}
