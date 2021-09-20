use boson::api::BosonLang;

use std::env::args;
use std::process;

fn info() {
    println!("boson-eval v0.1.0");
    println!("Boson is an educational general purpose programming language written in Rust.");
    println!("This binary evaluates program files. Usage: boson-eval file-name");
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

    // run evaluator:
    if f_name.ends_with(".b") {
        let ret = BosonLang::eval_bytecode(f_name.clone());
        if ret.is_some() {
            process::exit(0);
        }
        process::exit(-1);
    } else {
        let ret = BosonLang::eval_file(f_name.clone());
        if ret.is_some() {
            process::exit(0);
        }
        process::exit(-1);
    }
}
