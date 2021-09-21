use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::process;
use std::rc::Rc;

use crate::api;
use crate::api::BosonLang;
use crate::compiler;
use crate::types::array;
use crate::types::buffer;
use crate::types::hash;
use crate::types::iter;
use crate::types::object;
use crate::vm;

use compiler::symtab::ConstantPool;
use vm::global::GlobalPool;
use vm::thread::BosonThreads;
use vm::thread::ThreadParams;
use vm::BosonVM;

use api::Platform;
use array::Array;
use hash::HashTable;
use object::Object;

#[repr(u8)]
#[derive(PartialEq, Clone, Debug, Eq, Copy)]
pub enum BuiltinKind {
    Print,
    Truthy,
    Println,
    Length,
    Builtins,
    TimeUnix,
    Eval,
    Disasm,
    Args,
    Exit,
    Env,
    Envs,
    Platform,
    String,
    Int,
    Float,
    Bool,
    Byte,
    Char,
    Iter,
    Next,
    HasNext,
    Bytes,
    TypeOf,
    CreateArray,
    Exec,
    ExecRaw,
    ExecShell,
    SleepMs,
    SleepSec,
    CallFunc,
    CallAsync,
    FStat,
    FWrite,
    FAppend,
    FRead,
    ReadLine,
    SWrite,
    SRead,
    Wait,
    BytecodeEval,
    EndMark, // the end marker will tell the number of varinats in BuiltinKind, since
             // they are sequential.
}

fn repr_is_big_endian(args: &Vec<Rc<Object>>) -> bool {
    if args.len() == 2 && args[1].get_type() == "bool" && args[1].is_true() {
        true
    } else {
        false
    }
}

impl BuiltinKind {
    pub fn get_size() -> usize {
        return BuiltinKind::EndMark as usize;
    }

    pub fn get_idx(kind: BuiltinKind) -> Option<usize> {
        Some(kind as usize)
    }

    pub fn desribe(&self) -> String {
        match self {
            BuiltinKind::Print => "print".to_string(),
            BuiltinKind::Truthy => "is_true".to_string(),
            BuiltinKind::Println => "println".to_string(),
            BuiltinKind::Length => "len".to_string(),
            BuiltinKind::Builtins => "builtins".to_string(),
            BuiltinKind::TimeUnix => "unix_time".to_string(),
            BuiltinKind::Eval => "eval".to_string(),
            BuiltinKind::Disasm => "disasm".to_string(),
            BuiltinKind::Args => "args".to_string(),
            BuiltinKind::Exit => "exit".to_string(),
            BuiltinKind::Env => "env".to_string(),
            BuiltinKind::Envs => "envs".to_string(),
            BuiltinKind::Platform => "platform".to_string(),
            BuiltinKind::CreateArray => "create_array".to_string(),
            BuiltinKind::Int => "int".to_string(),
            BuiltinKind::String => "string".to_string(),
            BuiltinKind::Float => "float".to_string(),
            BuiltinKind::Bool => "bool".to_string(),
            BuiltinKind::TypeOf => "type_of".to_string(),
            BuiltinKind::Byte => "byte".to_string(),
            BuiltinKind::Bytes => "bytes".to_string(),
            BuiltinKind::Char => "char".to_string(),
            BuiltinKind::Iter => "iter".to_string(),
            BuiltinKind::HasNext => "has_next".to_string(),
            BuiltinKind::Next => "next".to_string(),
            BuiltinKind::Exec => "exec".to_string(),
            BuiltinKind::ExecRaw => "exec_raw".to_string(),
            BuiltinKind::SleepSec => "sleep_sec".to_string(),
            BuiltinKind::SleepMs => "sleep_ms".to_string(),
            BuiltinKind::CallFunc => "call_func".to_string(),
            BuiltinKind::CallAsync => "call_async".to_string(),
            BuiltinKind::Wait => "wait".to_string(),
            BuiltinKind::FStat => "fstat".to_string(),
            BuiltinKind::FWrite => "fwrite".to_string(),
            BuiltinKind::FAppend => "fappend".to_string(),
            BuiltinKind::ReadLine => "input".to_string(),
            BuiltinKind::SRead => "stdin".to_string(),
            BuiltinKind::SWrite => "stdout".to_string(),
            BuiltinKind::FRead => "fread".to_string(),
            BuiltinKind::BytecodeEval => "eval_bytecode".to_string(),
            _ => "undef".to_string(),
        }
    }

    pub fn exec(
        &self,
        args: Vec<Rc<Object>>,
        platform: &Platform,
        gp: &mut GlobalPool,
        c: &mut ConstantPool,
        th: &mut BosonThreads,
    ) -> Result<Rc<Object>, String> {
        match self {
            BuiltinKind::Print => {
                if args.len() == 0 {
                    return Err("print() takes atleast one argument, 0 provided".to_string());
                }

                // print function:
                let length = args.len();
                let mut fmt_string = String::new();
                for idx in 0..length - 1 {
                    fmt_string.push_str(&format!("{} ", args[idx].describe()));
                }

                fmt_string.push_str(&format!("{}", args[length - 1].describe()));

                // call the platform print function:
                let print_fn = platform.print;
                print_fn(&fmt_string);

                return Ok(Rc::new(Object::Noval));
            }

            BuiltinKind::Println => {
                // println function:
                let length = args.len();

                let mut fmt_string = String::new();
                if length == 0 {
                    fmt_string.push_str("\n");
                } else {
                    for idx in 0..length - 1 {
                        fmt_string.push_str(&format!("{} ", args[idx].describe()));
                    }

                    fmt_string.push_str(&format!("{}\n", args[length - 1].describe()));
                }

                // call the platform println function:
                let print_fn = platform.print;
                print_fn(&fmt_string);

                return Ok(Rc::new(Object::Noval));
            }

            BuiltinKind::Truthy => {
                // is_true functions
                if args.len() != 1 {
                    return Err(format!(
                        "is_true() takes one argument, {} provided.",
                        args.len()
                    ));
                }

                return Ok(Rc::new(Object::Bool(args[0].as_ref().is_true())));
            }

            BuiltinKind::Length => {
                if args.len() != 1 {
                    return Err(format!("len() takes one argument, {} provided", args.len()));
                }

                let obj = args[0].as_ref();
                match obj {
                    Object::Str(st) => Ok(Rc::new(Object::Int(st.len() as i64))),
                    Object::Array(arr) => {
                        Ok(Rc::new(Object::Int(arr.borrow().elements.len() as i64)))
                    }
                    Object::HashTable(ht) => {
                        Ok(Rc::new(Object::Int(ht.borrow().entries.len() as i64)))
                    }
                    _ => Err(format!("len() cannot be applied on {}", obj.get_type())),
                }
            }

            BuiltinKind::Eval => {
                if args.len() != 1 {
                    return Err(format!(
                        "eval() takes one argument, {} provided",
                        args.len()
                    ));
                }

                let obj = args[0].as_ref();
                if obj.get_type() != "string" {
                    return Err(format!(
                        "eval() takes string as argument, {} provided",
                        obj.get_type()
                    ));
                }

                let buffer = obj.describe().as_bytes().to_vec();
                let result = BosonLang::eval_buffer(buffer);
                if result.is_none() {
                    return Ok(Rc::new(Object::Noval));
                }

                return Ok(result.unwrap());
            }

            BuiltinKind::Builtins => {
                if args.len() != 0 {
                    return Err(format!(
                        "builtins() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let all_builtins = BuiltinKind::get_names();
                let mut strings = vec![];
                for name in all_builtins {
                    strings.push(Rc::new(Object::Str(name.clone())));
                }

                return Ok(Rc::new(Object::Array(RefCell::new(Array {
                    name: "todo".to_string(),
                    elements: strings,
                }))));
            }

            BuiltinKind::Disasm => {
                if args.len() != 1 {
                    return Err(format!(
                        "disasm() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                let obj = args[0].as_ref();
                if obj.get_type() != "string" {
                    return Err(format!(
                        "eval() takes string as argument, {} provided",
                        obj.get_type()
                    ));
                }

                // disassemble:
                let buffer = obj.describe().as_bytes().to_vec();
                let output_result = BosonLang::disasm_buffer(buffer);
                if output_result.is_none() {
                    return Ok(Rc::new(Object::Noval));
                }

                return Ok(Rc::new(Object::Str(output_result.unwrap())));
            }

            BuiltinKind::TimeUnix => {
                if args.len() != 0 {
                    return Err(format!(
                        "unix_time() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let get_time_fn = platform.get_unix_time;
                let epoch_time_res = get_time_fn();

                if epoch_time_res.is_err() {
                    return Err("Failed to fetch UNIX epoch time.".to_string());
                }

                let epoch_time = epoch_time_res.unwrap();
                return Ok(Rc::new(Object::Float(epoch_time)));
            }

            BuiltinKind::Args => {
                if args.len() != 0 {
                    return Err(format!(
                        "args() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let args_fn = platform.get_args;
                let args = args_fn();

                let mut args_array = Array {
                    name: "builtin_args".to_string(),
                    elements: vec![],
                };

                args_array.elements = args;
                return Ok(Rc::new(Object::Array(RefCell::new(args_array))));
            }

            BuiltinKind::Exit => {
                if args.len() != 1 {
                    return Err(format!("exit() takes 1 argument, {} provided", args.len()));
                }

                let obj = args[0].as_ref();
                match obj {
                    Object::Int(exit_code) => {
                        process::exit(*exit_code as i32);
                    }
                    _ => {
                        return Err(format!(
                            "exit() takes int as an argument, {} provided",
                            obj.get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Env => {
                if args.len() == 0 {
                    return Err(format!("get_env() takes atleast one argument, 0 provided",));
                }

                let env_name_obj = args[0].as_ref();
                if env_name_obj.get_type() != "string" {
                    return Err(format!(
                        "env() takes string as first argument, {} provided",
                        env_name_obj.get_type()
                    ));
                }

                let env_key = env_name_obj.describe();

                // call the platform specific get_env:
                let get_env_fn = platform.get_env;
                let env_value_res = get_env_fn(&env_key);
                if env_value_res.is_err() {
                    if args.len() == 2 {
                        // default value is provided, return it
                        return Ok(args[1].clone());
                    }
                    return Ok(Rc::new(Object::Noval));
                }

                let env_value = env_value_res.unwrap();
                return Ok(Rc::new(Object::Str(env_value)));
            }

            BuiltinKind::Envs => {
                if args.len() != 0 {
                    return Err(format!(
                        "envs() takes zero arguments, {} provided",
                        args.len()
                    ));
                }
                // get envs:
                let get_envs_fn = platform.get_envs;
                let envs = get_envs_fn();
                let mut env_table = HashTable {
                    name: "envs".to_string(),
                    entries: HashMap::new(),
                };
                for (key, value) in envs {
                    env_table.set(Rc::new(Object::Str(key)), Rc::new(Object::Str(value)));
                }

                return Ok(Rc::new(Object::HashTable(RefCell::new(env_table))));
            }

            BuiltinKind::CreateArray => {
                let args_len = args.len();
                if args_len == 0 || args_len > 2 {
                    return Err(format!(
                        "create_array() takes one or two arguments, provided {}.",
                        args_len
                    ));
                }

                match args[0].as_ref() {
                    Object::Int(i) => {
                        let to_fill = if args_len == 1 {
                            Rc::new(Object::Noval)
                        } else {
                            args[1].clone()
                        };

                        // create a vector
                        let mut arr_vec = vec![];
                        arr_vec.resize(*i as usize, to_fill);

                        let arr_type = Array {
                            name: "todo".to_string(),
                            elements: arr_vec,
                        };

                        return Ok(Rc::new(Object::Array(RefCell::new(arr_type))));
                    }
                    _ => {
                        return Err(format!(
                            "create_array() expects int as first argument, provided {}.",
                            args[0].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Platform => {
                if args.len() != 0 {
                    return Err(format!(
                        "arch() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let platform_info_fn = platform.get_platform_info;
                let platform_info_vec = platform_info_fn();

                let mut platform_table = HashTable {
                    name: "platform".to_string(),
                    entries: HashMap::new(),
                };

                platform_table.set(
                    Rc::new(Object::Str("arch".to_string())),
                    Rc::new(Object::Str(platform_info_vec[0].clone())),
                );

                platform_table.set(
                    Rc::new(Object::Str("family".to_string())),
                    Rc::new(Object::Str(platform_info_vec[1].clone())),
                );

                platform_table.set(
                    Rc::new(Object::Str("os".to_string())),
                    Rc::new(Object::Str(platform_info_vec[2].clone())),
                );

                return Ok(Rc::new(Object::HashTable(RefCell::new(platform_table))));
            }

            // Get type:
            BuiltinKind::TypeOf => {
                if args.len() != 1 {
                    return Err(format!(
                        "type_of() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                let t_str = args[0].as_ref().get_type();
                return Ok(Rc::new(Object::Str(t_str)));
            }

            // Conversion functions:
            BuiltinKind::String => {
                if args.len() != 1 {
                    return Err(format!(
                        "string() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::ByteBuffer(bytes) => {
                        let result_str = bytes.borrow().get_as_string();
                        if result_str.is_err() {
                            return Err(result_str.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Str(result_str.unwrap())));
                    }
                    _ => {
                        let result_str = args[0].as_ref().describe();
                        return Ok(Rc::new(Object::Str(result_str)));
                    }
                }
            }

            BuiltinKind::Int => {
                if args.len() != 1 {
                    return Err(format!(
                        "string() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Int(i) => {
                        return Ok(Rc::new(Object::Int(*i)));
                    }

                    Object::Str(st) => {
                        let result = st.trim().replace("\n", "").parse::<i64>();
                        if result.is_err() {
                            return Err(format!("String {} cannot be converted to integer.", st));
                        }

                        let result_i64 = result.unwrap();
                        return Ok(Rc::new(Object::Int(result_i64)));
                    }

                    Object::Byte(byte) => return Ok(Rc::new(Object::Int(*byte as i64))),

                    Object::Float(f) => {
                        return Ok(Rc::new(Object::Int(f.round() as i64)));
                    }

                    Object::ByteBuffer(bytes) => {
                        let i_for_b_res = bytes.borrow().get_as_i64();
                        if i_for_b_res.is_err() {
                            return Err(i_for_b_res.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Int(i_for_b_res.unwrap())));
                    }

                    Object::Char(c) => {
                        return Ok(Rc::new(Object::Int(*c as i64)));
                    }

                    Object::Bool(b) => {
                        let i_for_b = if *b { 1 } else { 0 };
                        return Ok(Rc::new(Object::Int(i_for_b)));
                    }

                    _ => {
                        return Err(format!(
                            "Object of type {} cannot be converted to int",
                            args[0].as_ref().get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Bool => {
                if args.len() != 1 {
                    return Err(format!(
                        "string() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                return Ok(Rc::new(Object::Bool(args[0].as_ref().is_true())));
            }

            BuiltinKind::Byte => {
                if args.len() != 1 {
                    return Err(format!(
                        "string() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Byte(byte) => {
                        return Ok(Rc::new(Object::Byte(*byte)));
                    }

                    Object::Int(i) => {
                        if *i < 0 || *i > 255 {
                            return Err(format!("Integer {} cannot be casted to raw", i));
                        }

                        return Ok(Rc::new(Object::Byte(*i as u8)));
                    }

                    Object::Char(c) => {
                        return Ok(Rc::new(Object::Byte(*c as u8)));
                    }

                    Object::Bool(b) => {
                        return Ok(Rc::new(Object::Byte(if *b { 1 as u8 } else { 0 as u8 })));
                    }
                    _ => {
                        return Err(format!(
                            "Object of type {} cannot be converted to byte",
                            args[0].as_ref().get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Float => {
                if args.len() != 1 {
                    return Err(format!(
                        "string() takes 1 argument, {} provided",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Int(i) => {
                        return Ok(Rc::new(Object::Float(*i as f64)));
                    }

                    Object::Str(st) => {
                        let result = st.trim().replace("\n", "").parse::<f64>();
                        if result.is_err() {
                            return Err(format!("String {} cannot be converted to integer.", st));
                        }

                        let result_f64 = result.unwrap();
                        return Ok(Rc::new(Object::Float(result_f64)));
                    }

                    Object::Float(f) => {
                        return Ok(Rc::new(Object::Float(*f)));
                    }

                    Object::Char(c) => {
                        return Ok(Rc::new(Object::Float(*c as i64 as f64)));
                    }

                    Object::Bool(b) => {
                        let f_for_b = if *b { 1.0 } else { 0.0 };
                        return Ok(Rc::new(Object::Float(f_for_b)));
                    }

                    Object::ByteBuffer(bytes) => {
                        let f_for_b_res = bytes.borrow().get_as_f64();
                        if f_for_b_res.is_err() {
                            return Err(f_for_b_res.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Float(f_for_b_res.unwrap())));
                    }

                    _ => {
                        return Err(format!(
                            "Object of type {} cannot be converted to float",
                            args[0].as_ref().get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Bytes => {
                if args.len() == 0 {
                    return Err(format!("bytes() takes atleast one argument, zero provided",));
                }

                match args[0].as_ref() {
                    Object::Int(i) => {
                        let is_bg = repr_is_big_endian(&args);
                        let b_array_res = buffer::Buffer::from_i64(i, !is_bg);
                        if b_array_res.is_err() {
                            return Err(b_array_res.unwrap_err());
                        }

                        return Ok(Rc::new(Object::ByteBuffer(RefCell::new(
                            b_array_res.unwrap(),
                        ))));
                    }

                    Object::Float(f) => {
                        let is_bg = repr_is_big_endian(&args);
                        let b_array_res = buffer::Buffer::from_f64(f, !is_bg);
                        if b_array_res.is_err() {
                            return Err(b_array_res.unwrap_err());
                        }

                        return Ok(Rc::new(Object::ByteBuffer(RefCell::new(
                            b_array_res.unwrap(),
                        ))));
                    }

                    Object::Char(c) => {
                        let result_arr =
                            buffer::Buffer::from_u8(vec![*c as u8], "todo".to_string(), true);

                        return Ok(Rc::new(Object::ByteBuffer(RefCell::new(result_arr))));
                    }

                    Object::Str(st) => {
                        let result_arr = buffer::Buffer::from_string(st);
                        return Ok(Rc::new(Object::ByteBuffer(RefCell::new(result_arr))));
                    }
                    _ => {
                        return Err(format!(
                            "Object of type {} cannot be converted to bytes",
                            args[0].as_ref().get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Char => {
                if args.len() == 0 {
                    return Err(format!(
                        "exec() expects atleast one argument, zero provided."
                    ));
                }

                match args[0].as_ref() {
                    Object::Byte(b) => {
                        return Ok(Rc::new(Object::Char(*b as char)));
                    }
                    _ => {
                        return Err(format!(
                            "Object of type {} cannot be converted to bytes",
                            args[0].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::Exec => {
                if args.len() == 0 {
                    return Err(format!(
                        "exec() expects atleast one argument, zero provided."
                    ));
                }

                let exec_fn = platform.exec;
                let result = exec_fn(&args);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                // cast it to string:
                let (exit_code, op_data) = result.unwrap();
                let result_str = String::from_utf8(op_data);
                if result_str.is_err() {
                    return Err(format!("{}", result_str.unwrap_err()));
                }

                let op_array = Array {
                    name: "todo".to_string(),
                    elements: vec![
                        Rc::new(Object::Int(exit_code as i64)),
                        Rc::new(Object::Str(result_str.unwrap())),
                    ],
                };

                return Ok(Rc::new(Object::Array(RefCell::new(op_array))));
            }

            BuiltinKind::ExecRaw => {
                if args.len() == 0 {
                    return Err(format!(
                        "exec_raw() expects atleast one argument, zero provided.",
                    ));
                }

                let exec_fn = platform.exec;
                let result = exec_fn(&args);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                // return the result and exit code
                let (exit_code, op_data) = result.unwrap();
                let raw_buffer = buffer::Buffer::from_u8(op_data, "todo".to_string(), true);

                let op_array = Array {
                    name: "todo".to_string(),
                    elements: vec![
                        Rc::new(Object::Int(exit_code as i64)),
                        Rc::new(Object::ByteBuffer(RefCell::new(raw_buffer))),
                    ],
                };

                return Ok(Rc::new(Object::Array(RefCell::new(op_array))));
            }

            BuiltinKind::Iter => {
                if args.len() != 1 {
                    return Err(format!(
                        "iter() expects one argument, {} provided.",
                        args.len()
                    ));
                }

                let object_to_iter = args[0].as_ref();
                let iter_res = iter::ObjectIterator::new(Rc::new(object_to_iter.clone()));
                if iter_res.is_err() {
                    return Err(iter_res.unwrap_err());
                }

                return Ok(Rc::new(Object::Iter(RefCell::new(iter_res.unwrap()))));
            }

            BuiltinKind::HasNext => {
                if args.len() != 1 {
                    return Err(format!(
                        "has_next() expects one argument, {} provided.",
                        args.len()
                    ));
                }

                let obj = args[0].as_ref();
                match obj {
                    Object::Iter(it) => {
                        let has_next = it.borrow().has_next();
                        return Ok(Rc::new(Object::Bool(has_next)));
                    }
                    _ => {
                        return Err(format!(
                            "has_next() can be applied only on iter, but got {}",
                            obj.get_type()
                        ))
                    }
                }
            }

            BuiltinKind::Next => {
                if args.len() != 1 {
                    return Err(format!(
                        "next() expects one argument, {} provided.",
                        args.len()
                    ));
                }

                let obj = args[0].as_ref();
                match obj {
                    Object::Iter(it) => {
                        let next_obj = it.borrow_mut().next();
                        if next_obj.is_none() {
                            return Err(format!("next() called on ended iterator",));
                        }

                        return Ok(next_obj.unwrap());
                    }
                    _ => {
                        return Err(format!(
                            "has_next() can be applied only on iter, but got {}",
                            obj.get_type()
                        ));
                    }
                }
            }

            BuiltinKind::SleepMs => {
                if args.len() != 1 {
                    return Err(format!(
                        "sleep() expects one argument, provided {}.",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Float(duration_f_ms) => {
                        let sleep_fn = platform.sleep;
                        sleep_fn(duration_f_ms);
                    }
                    Object::Int(duration_i_ms) => {
                        let sleep_fn = platform.sleep;
                        sleep_fn(&(*duration_i_ms as f64));
                    }
                    _ => {
                        return Err(format!(
                            "sleep() expects a float or int, but got {}",
                            args[0].get_type()
                        ));
                    }
                }

                return Ok(Rc::new(Object::Noval));
            }

            BuiltinKind::SleepSec => {
                if args.len() != 1 {
                    return Err(format!(
                        "sleep() expects one argument, provided {}.",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Float(duration_f_s) => {
                        let sleep_fn = platform.sleep;
                        sleep_fn(&(*duration_f_s * 1000 as f64));
                    }
                    Object::Int(duration_i_ms) => {
                        let sleep_fn = platform.sleep;
                        sleep_fn(&((*duration_i_ms * 1000) as f64));
                    }
                    _ => {
                        return Err(format!(
                            "sleep() expects a float or int, but got {}",
                            args[0].get_type()
                        ));
                    }
                }

                return Ok(Rc::new(Object::Noval));
            }

            BuiltinKind::CallFunc => {
                if args.len() != 2 {
                    return Err(format!(
                        "call_func takes 2 arguments, provided {}.",
                        args.len()
                    ));
                }

                match (args[0].as_ref(), args[1].as_ref()) {
                    (Object::ClosureContext(ctx), Object::Array(params)) => {
                        let n_parms_required = ctx.as_ref().compiled_fn.as_ref().num_parameters;
                        let n_provided = params.borrow().elements.len();
                        if n_parms_required != n_provided {
                            return Err(format!(
                                "Function {} requires {} parameters, provided {}",
                                ctx.as_ref().compiled_fn.name,
                                n_parms_required,
                                n_provided
                            ));
                        }

                        // launch the sandbox function:
                        let sandbox_result = BosonVM::execute_sandbox(
                            ctx.clone(),
                            params.borrow().elements.clone(),
                            platform,
                            gp.clone(),
                            c.clone(),
                        );

                        if sandbox_result.is_err() {
                            let error = sandbox_result.unwrap_err();
                            return Err(format!(
                                "{:?}: {} at {}, Instruction: {:?}",
                                error.t, error.message, error.pos, error.instruction
                            ));
                        }

                        let obj_result = sandbox_result.unwrap();
                        return Ok(obj_result);
                    }
                    (Object::Builtins(func), Object::Array(params)) => {
                        let params_vec = params.borrow().elements.clone();
                        let exec_result = func.exec(params_vec, platform, gp, c, th);
                        return exec_result;
                    }
                    _ => {
                        return Err(format!(
                            "call_func takes closure/func and array as arguments, but got {} {}.",
                            args[0].get_type(),
                            args[1].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::CallAsync => {
                if args.len() != 2 {
                    return Err(format!(
                        "call_async takes 2 arguments, provided {}.",
                        args.len()
                    ));
                }

                match (args[0].as_ref(), args[1].as_ref()) {
                    (Object::ClosureContext(ctx), Object::Array(params)) => {
                        let n_parms_required = ctx.as_ref().compiled_fn.as_ref().num_parameters;
                        let n_provided = params.borrow().elements.len();
                        if n_parms_required != n_provided {
                            return Err(format!(
                                "Function {} requires {} parameters, provided {}",
                                ctx.as_ref().compiled_fn.name,
                                n_parms_required,
                                n_provided
                            ));
                        }

                        let params = params.borrow().elements.clone();
                        // call the async function
                        let thread_params =
                            ThreadParams::new(ctx.clone(), params, gp.clone(), c.clone());

                        let thread_create_res = th.create_thread_sandbox(thread_params, platform);
                        if thread_create_res.is_err() {
                            return Err(thread_create_res.unwrap_err());
                        }

                        // wait for the thread to finish:
                        let thread_exec_res = th.wait_and_return(thread_create_res.unwrap());
                        if thread_exec_res.is_err() {
                            return Err(thread_exec_res.unwrap_err());
                        }

                        // thread has finished execution successfully:
                        let sandbox_result = thread_exec_res.unwrap().result;
                        if sandbox_result.is_err() {
                            let error = sandbox_result.unwrap_err();
                            return Err(format!(
                                "{:?}: {} at {}, Instruction: {:?}",
                                error.t, error.message, error.pos, error.instruction
                            ));
                        }

                        // return the result object
                        return Ok(sandbox_result.unwrap());
                    }
                    _ => {
                        return Err(format!(
                            "call_async takes closure/func and array as arguments, but got {} {}.",
                            args[0].get_type(),
                            args[1].get_type()
                        ));
                    }
                }
            }
            BuiltinKind::ReadLine => {
                if args.len() > 1 {
                    return Err(format!(
                        "input() takes at max 1 argument, provided {}",
                        args.len()
                    ));
                }

                let display_obj = args.get(0).map(|o| o.describe());
                let readline_fn = platform.read_line;
                let result = readline_fn(display_obj);
                if result.is_err() {
                    return Err(result.unwrap());
                }

                return Ok(Rc::new(Object::Str(result.unwrap())));
            }
            BuiltinKind::Wait => {
                if args.len() != 1 {
                    return Err(format!("wait() takes 1 argument, given {}", args.len()));
                }

                match args[0].as_ref() {
                    Object::Thread(t_block) => {
                        let thread_id = t_block.borrow().handle_id;
                        let thread_exec_res = th.wait_and_return(thread_id);
                        if thread_exec_res.is_err() {
                            return Err(format!(
                                "{} wait error: {}",
                                t_block.borrow().describe(),
                                thread_exec_res.unwrap_err()
                            ));
                        }

                        let sandbox_result = thread_exec_res.unwrap().result;
                        if sandbox_result.is_err() {
                            let error = sandbox_result.unwrap_err();
                            return Err(format!(
                                "{:?}: {} at {}, Instruction: {:?}",
                                error.t, error.message, error.pos, error.instruction
                            ));
                        }

                        return Ok(sandbox_result.unwrap());
                    }
                    _ => {
                        return Err(format!(
                            "wait() takes thread as argument, provided {}",
                            args[0].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::FStat => {
                if args.len() != 1 {
                    return Err(format!(
                        "fstat() takes at 1 argument, {} provided",
                        args.len()
                    ));
                }

                match args[0].as_ref() {
                    Object::Str(path) => {
                        let platform_finfo = platform.finfo;
                        let result = platform_finfo(path.clone());
                        if result.is_err() {
                            return Err(result.unwrap_err());
                        }

                        return Ok(result.unwrap());
                    }
                    _ => {
                        return Err(format!(
                            "fstat() takes string as argument, provided {}",
                            args[0].get_type()
                        ))
                    }
                }
            }

            BuiltinKind::FWrite => {
                if args.len() != 2 {
                    return Err(format!(
                        "fwrite() takes 2 arguments, provided {}",
                        args.len()
                    ));
                }

                match (args[0].as_ref(), args[1].as_ref()) {
                    (Object::Str(st), Object::ByteBuffer(buffer)) => {
                        let fwrite_fn = platform.fwrite;
                        let result = fwrite_fn(st.clone(), &buffer.borrow().data);
                        if result.is_err() {
                            return Err(result.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Int(result.unwrap() as i64)));
                    }
                    _ => {
                        return Err(format!(
                            "fwrite() takes string and bytes as arguments, provided {}, {}",
                            args[0].get_type(),
                            args[1].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::FAppend => {
                if args.len() != 2 {
                    return Err(format!(
                        "fappend() takes 2 arguments, provided {}",
                        args.len()
                    ));
                }

                match (args[0].as_ref(), args[1].as_ref()) {
                    (Object::Str(st), Object::ByteBuffer(buffer)) => {
                        let fappend_fn = platform.fappend;
                        let result = fappend_fn(st.clone(), &buffer.borrow().data);
                        if result.is_err() {
                            return Err(result.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Int(result.unwrap() as i64)));
                    }
                    _ => {
                        return Err(format!(
                            "fappend() takes string and bytes as arguments, provided {}, {}",
                            args[0].get_type(),
                            args[1].get_type()
                        ));
                    }
                }
            }

            BuiltinKind::FRead => {
                fn __read(
                    path: String,
                    start: Option<u64>,
                    n: Option<u64>,
                    p: &Platform,
                ) -> Result<Rc<Object>, String> {
                    let read_fn = p.fread;
                    let result = read_fn(path, start, n);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }

                    let (data, nb) = result.unwrap();
                    // return the array:
                    let arr = Array {
                        name: "fread_data".to_string(),
                        elements: vec![
                            Rc::new(Object::Int(nb as i64)),
                            Rc::new(Object::ByteBuffer(RefCell::new(buffer::Buffer::from_u8(
                                data,
                                "fread_data".to_string(),
                                false,
                            )))),
                        ],
                    };

                    return Ok(Rc::new(Object::Array(RefCell::new(arr))));
                }

                if args.len() == 1 {
                    // only the file path is provided
                    match args[0].as_ref() {
                        Object::Str(st) => {
                            let result = __read(st.clone(), None, None, &platform);
                            return result;
                        }
                        _ => {
                            return Err(format!(
                                "fread() takes string as first argument, but got {}",
                                args[0].get_type()
                            ))
                        }
                    }
                } else if args.len() == 2 {
                    match (args[0].as_ref(), args[1].as_ref()) {
                        (Object::Str(st), Object::Int(i)) => {
                            let result = __read(st.clone(), Some(*i as u64), None, &platform);
                            return result;
                        }
                        _ => {
                            return Err(format!(
                            "fread() takes string and int as first arguments, but got {} and {}",
                            args[0].get_type(),
                            args[1].get_type()
                        ))
                        }
                    }
                } else if args.len() == 3 {
                    match (args[0].as_ref(), args[1].as_ref(), args[2].as_ref()) {
                        (Object::Str(st), Object::Int(i), Object::Int(j)) => {
                            let result = __read(st.clone(), Some(*i as u64), Some(*j as u64), &platform);
                            return result;
                        }
                        _ => {
                            return Err(format!(
                            "fread() takes string and int as first arguments, but got {}, {} and {}",
                            args[0].get_type(),
                            args[1].get_type(),
                            args[2].get_type()
                        ))
                        }
                    }
                } else {
                    return Err(format!(
                        "fread() takes 1, 2 or 3 arguments, but got {}",
                        args.len()
                    ));
                }
            }

            BuiltinKind::SWrite => {
                if args.len() != 1 {
                    return Err(format!(
                        "stdout() takes 1 argument, provided {}",
                        args.len()
                    ));
                }

                let data_obj = args[0].as_ref();
                match data_obj {
                    Object::ByteBuffer(buffer) => {
                        let stdout_fn = platform.stdout_write;
                        let result = stdout_fn(&buffer.borrow().data);
                        if result.is_err() {
                            return Err(result.unwrap_err());
                        }

                        return Ok(Rc::new(Object::Int(result.unwrap() as i64)));
                    }
                    _ => {
                        return Err(format!(
                            "stdout() takes bytes as argument, provided {}",
                            data_obj.get_type()
                        ))
                    }
                }
            }

            BuiltinKind::SRead => {
                if args.len() != 0 {
                    return Err(format!(
                        "stdin() takes no arguments, provided {}",
                        args.len()
                    ));
                }

                let stdin_fn = platform.stdin_read;
                let result = stdin_fn();

                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                let data = result.unwrap();
                let byte_buffer = buffer::Buffer::from_u8(data, "stdout".to_string(), true);
                return Ok(Rc::new(Object::ByteBuffer(RefCell::new(byte_buffer))));
            }

            BuiltinKind::BytecodeEval => {
                if args.len() != 1 {
                    return Err(format!(
                        "eval_bytecode() takes 1 argument, provided {}.",
                        args.len()
                    ));
                }

                let obj = args[0].as_ref();
                match obj {
                    Object::Str(st) => {
                        let result = BosonLang::eval_bytecode(st.clone());
                        if result.is_none() {
                            return Ok(Rc::new(Object::Noval));
                        }

                        return Ok(result.unwrap());
                    }
                    _ => {
                        return Err(format!(""));
                    }
                }
            }

            _ => return Err("Trying to invoke invalid builtin".to_string()),
        }
    }

    pub fn get_by_name(name: &String) -> Option<BuiltinKind> {
        let builtin_size = BuiltinKind::EndMark as usize;

        for idx in 0..builtin_size {
            let builtin_kind: BuiltinKind = unsafe { ::std::mem::transmute(idx as u8) };
            if builtin_kind.desribe() == *name {
                return Some(builtin_kind);
            }
        }

        return None;
    }

    pub fn get_by_index(idx: usize) -> Option<BuiltinKind> {
        if idx >= BuiltinKind::get_size() {
            return None;
        }

        let builtin_kind: BuiltinKind = unsafe { ::std::mem::transmute(idx as u8) };
        return Some(builtin_kind);
    }

    pub fn get_names() -> Vec<String> {
        let builtin_size = BuiltinKind::EndMark as usize;

        let mut names = vec![];

        for idx in 0..builtin_size + 1 {
            // transmute to BuiltinKind
            let builtin_kind: BuiltinKind = unsafe { ::std::mem::transmute(idx as u8) };
            names.push(builtin_kind.desribe());
        }

        return names;
    }
}

impl Hash for BuiltinKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.desribe().hash(state);
    }
}
