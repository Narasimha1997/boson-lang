use crate::types::object;

use std::rc::Rc;
use std::process::Command;

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
    let mut command = Command::new(args[0].as_ref().describe());
    for idx in 1..args.len() {
        command.arg(args[idx].as_ref().describe());
    }

    let result = command.output();
    if result.is_err() {
        return Err(format!("Sub Command Error: {}", result.unwrap_err()));
    }

    // return the output:
    let cmd_result = result.unwrap();
    let output_data = if cmd_result.status.success() {
        cmd_result.stdout
    } else {
        cmd_result.stderr
    };

    // get the exit status code:
    let exit_code = match cmd_result.status.code() {
        Some(code) => code,
        None => return Err("Failed to get exit code.".to_string()),
    };

    return Ok((exit_code, output_data));
}