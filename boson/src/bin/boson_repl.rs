extern crate boson;
extern crate rustyline;

use boson::api::BosonLang;
use boson::types::object::Object;

use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn display_result(obj: Rc<Object>) {
    match obj.as_ref() {
        Object::Noval => {}
        _ => println!("{}", obj.as_ref().describe()),
    }
}

pub fn main() {
    println!("Welcome to Boson REPL");
    println!(
        "This is a REPL binary for boson - a general purpose programming language written in rust."
    );
    println!("Boson v0.0.1");

    let mut rl = Editor::<()>::new();

    let mut lang = BosonLang::new_from_buffer(
        "println(\"VM Check - Passed.\");\n"
            .as_bytes()
            .to_vec()
            .clone(),
    );

    let result = lang.eval_state();
    if result.is_some() {
        display_result(result.unwrap());
    }

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                lang.update(line.as_bytes().to_vec().clone());

                // eval bytecode:
                let ret_obj_res = lang.eval_state();
                if ret_obj_res.is_some() {
                    display_result(ret_obj_res.unwrap());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\nGood Bye :)");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
