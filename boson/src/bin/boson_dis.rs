use boson::api::BosonLang;

use std::env::args;

fn info() {
    println!("boson-dis v0.1.0");
    println!("Boson is an educational general purpose programming language written in Rust.");
    println!("This binary generates bytecode and displays it. Usage: boson-dis file-name");
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

    if f_name.ends_with(".b") {
        let result = BosonLang::disasm_bytecode(f_name.clone());
        if result.is_some() {
            println!("{}", result.unwrap());
        }
    } else {
        // run evaluator:
        let result = BosonLang::disasm_file(f_name.clone());
        if result.is_some() {
            println!("{}", result.unwrap());
        }
    }
}
