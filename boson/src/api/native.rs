use crate::types::object;

use std::env;
use std::process::Command;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use std::thread;
use std::time::Duration;
use object::Object;

/*
    Contains all the implementation of native built-ins
*/

pub fn print(st: &String) {
    print!("{}", st);
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

pub fn get_args() -> Vec<Rc<Object>> {
    let mut cmd_args = env::args();
    // skip the binary name
    cmd_args.next();

    // get a vector slice starting from index 1:
    let arg_str_objects: Vec<Rc<Object>> = cmd_args.map(|arg| Rc::new(Object::Str(arg))).collect();
    return arg_str_objects;
}

pub fn get_env(name: &String) -> Result<String, String> {
    let result = env::var(name);
    if result.is_err() {
        return Err(format!("{}", result.unwrap_err()));
    }
    return Ok(result.unwrap());
}

pub fn get_envs() -> env::Vars {
    return env::vars();
}

pub fn get_unix_time() -> Result<f64, String> {
    let epoch_time_res = SystemTime::now().duration_since(UNIX_EPOCH);
    if epoch_time_res.is_err() {
        return Err(format!("{}", epoch_time_res.unwrap_err()));
    }

    let epoch_time = epoch_time_res.unwrap();
    return Ok(epoch_time.as_secs_f64());
}

pub fn get_platform_info() -> Vec<String> {
    return vec![
        env::consts::ARCH.to_string(),
        env::consts::FAMILY.to_string(),
        env::consts::OS.to_string(),
    ];
}

pub fn sleep(duration_ms: &f64) {
    let ns_time = (*duration_ms * 1000 as f64).round() as u64;
    thread::sleep(Duration::from_nanos(ns_time));
}