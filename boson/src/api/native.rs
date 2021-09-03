use crate::types::hash;
use crate::types::object;

use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use io::Read;
use io::Seek;
use io::SeekFrom;
use io::Write;
use std::cell::RefCell;

use hash::HashTable;
use object::Object;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

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
    let ns_time = (*duration_ms * 1000000 as f64).round() as u64;
    thread::sleep(Duration::from_nanos(ns_time));
}

pub fn sys_shell() -> String {
    // get the shell from ENV BOSON_SHELL
    // for native implementation, shell is sh by default
    let shell_var = env::var("BOSON_SHELL");
    return if shell_var.is_err() {
        match env::consts::OS {
            "windows" => "cmd /C".to_string(),
            "macos" => "zsh -c".to_string(),
            _ => "sh -c".to_string(),
        }
    } else {
        shell_var.unwrap()
    };
}

// These are alpha functions, built just to satisfy
// the immediate feature addition. In future, these will be
// replaced by modules. (FS module)

// Note: These are stateless functions.
// Every operation will result in a new fd being opened.

// Can be used in three ways:
// 1. Read all, pass no argument
// 2. Read from X first bytes, pass X as the argument.
// 3. Read Y bytes starting from X - [X, X+Y) Pass X and Y as arguments
// If parameter is not to be passed pass None, or pass an Option.
pub fn fread(path: String, start: Option<u64>, n_b: Option<u64>) -> Result<(Vec<u8>, u64), String> {
    let f_handle_res = fs::OpenOptions::new().read(true).open(&path);
    if f_handle_res.is_err() {
        return Err(format!(
            "Cannot open {}, Reason: {:?}",
            path,
            f_handle_res.unwrap_err()
        ));
    }

    let mut f_handle = f_handle_res.unwrap();

    if start.is_some() {
        let start_byte = start.unwrap();
        let result = f_handle.seek(SeekFrom::Start(start_byte));
        if result.is_err() {
            return Err(format!("{:?}", result.unwrap_err()));
        }
    }

    let mut read_buffer = vec![];

    if n_b.is_some() {
        let b_to_read = n_b.unwrap();
        read_buffer.resize(b_to_read as usize, 0u8);
        let read_result = f_handle.read_exact(&mut read_buffer);
        if read_result.is_err() {
            return Err(format!("Read Error {:?}", read_result.unwrap()));
        }

        return Ok((read_buffer, b_to_read));
    } else {
        let read_result = f_handle.read_to_end(&mut read_buffer);
        if read_result.is_err() {
            return Err(format!("Read Error {:?}", read_result.unwrap()));
        }

        return Ok((read_buffer, read_result.unwrap() as u64));
    }
}

// Writes content and returns the new size of the file.
// This function will always create a new file.
pub fn fwrite(path: String, data: &Vec<u8>) -> Result<u64, String> {
    let f_handle_res = fs::OpenOptions::new().write(true).open(&path);
    if f_handle_res.is_err() {
        return Err(format!(
            "Cannot open {}, Reason: {:?}",
            path,
            f_handle_res.unwrap_err()
        ));
    }

    // write:
    let mut f_handle = f_handle_res.unwrap();
    let result = f_handle.write_all(&data);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(data.len() as u64);
}

// Writes content and returns the new size of the file.
// This function append to a file, if exists or creates a new file.
pub fn fappend(path: String, data: Vec<u8>) -> Result<u64, String> {
    let f_handle_res = fs::OpenOptions::new().write(true).append(true).open(&path);
    if f_handle_res.is_err() {
        return Err(format!(
            "Cannot open {}, Reason: {:?}",
            path,
            f_handle_res.unwrap_err()
        ));
    }

    // write:
    let mut f_handle = f_handle_res.unwrap();
    let result = f_handle.write_all(&data);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(data.len() as u64);
}

// Returns the size of a file.
pub fn finfo(path: String) -> Result<Rc<Object>, String> {
    let m_result = fs::metadata(path);
    if m_result.is_err() {
        return Err(format!("{}", m_result.unwrap_err()));
    }

    // convert to hash table
    let metadata = m_result.unwrap();
    let mut h_map = HashMap::new();

    h_map.insert(
        Rc::new(Object::Str(format!("is_file"))),
        Rc::new(Object::Bool(metadata.is_file())),
    );
    h_map.insert(
        Rc::new(Object::Str(format!("is_dir"))),
        Rc::new(Object::Bool(metadata.is_dir())),
    );
    h_map.insert(
        Rc::new(Object::Str(format!("size"))),
        Rc::new(Object::Int(metadata.len() as i64)),
    );
    h_map.insert(
        Rc::new(Object::Str(format!("is_read_only"))),
        Rc::new(Object::Bool(metadata.permissions().readonly())),
    );

    if let Some(created) = metadata.created().ok() {
        if let Some(time_f) = created.duration_since(SystemTime::UNIX_EPOCH).ok() {
            h_map.insert(
                Rc::new(Object::Str(format!("created_at"))),
                Rc::new(Object::Float(time_f.as_secs_f64())),
            );
        }
    }

    if let Some(access) = metadata.accessed().ok() {
        if let Some(time_f) = access.duration_since(SystemTime::UNIX_EPOCH).ok() {
            h_map.insert(
                Rc::new(Object::Str(format!("accessed_at"))),
                Rc::new(Object::Float(time_f.as_secs_f64())),
            );
        }
    }

    let h_table = HashTable {
        name: "finfo".to_string(),
        entries: h_map,
    };

    return Ok(Rc::new(Object::HashTable(RefCell::new(h_table))));
}

// Read stdin input:
pub fn stdin_read() -> Result<Vec<u8>, String> {
    let mut buffer = vec![];
    let result = io::stdin().read(&mut buffer);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(buffer);
}

// Read a line as string:
pub fn read_line(display: String) -> Result<String, String> {
    print(&display);
    let mut string_buffer = String::new();
    let result = io::stdin().read_line(&mut string_buffer);
    if result.is_err() {
        return Err(format!("IO Error"));
    }
    return Ok(string_buffer);
}

// Write stdout output:
pub fn stdout_write(data: &Vec<u8>) -> Result<(), String> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let result = lock.write_all(&data);
    if result.is_err() {
        return Err(format!("IO Error"));
    }

    return Ok(result.unwrap());
}
