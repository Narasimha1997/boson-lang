use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::hash::Hasher;
use std::process;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::api::BosonLang;
use crate::types::array;
use crate::types::hash;
use crate::types::object;

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
    Str,
    Int,
    Float,
    Bool,
    CreateArray,
    EndMark, // the end marker will tell the number of varinats in BuiltinKind, since
             // they are sequential.
}

impl BuiltinKind {
    pub fn get_size() -> usize {
        return BuiltinKind::EndMark as usize;
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
            _ => "undef".to_string(),
        }
    }

    pub fn exec(&self, args: Vec<Rc<Object>>) -> Result<Rc<Object>, String> {
        match self {
            BuiltinKind::Print => {
                if args.len() == 0 {
                    return Err("print() takes atleast one argument, 0 provided".to_string());
                }

                // print function:
                let length = args.len();
                for idx in 0..length - 1 {
                    print!("{} ", args[idx].describe());
                }

                print!("{}", args[length - 1].describe());
                return Ok(Rc::new(Object::Noval));
            }

            BuiltinKind::Println => {
                // println function:
                let length = args.len();

                if length == 0 {
                    println!();
                } else {
                    for idx in 0..length - 1 {
                        print!("{} ", args[idx].describe());
                    }

                    print!("{}\n", args[length - 1].describe());
                }
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

                let epoch_time_res = SystemTime::now().duration_since(UNIX_EPOCH);
                if epoch_time_res.is_err() {
                    return Err("Failed to fetch UNIX epoch time.".to_string());
                }

                let epoch_time = epoch_time_res.unwrap();

                return Ok(Rc::new(Object::Float(epoch_time.as_secs_f64())));
            }

            BuiltinKind::Args => {
                if args.len() != 0 {
                    return Err(format!(
                        "args() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let mut cmd_args = env::args();

                let mut args_array = Array {
                    name: "builtin_args".to_string(),
                    elements: vec![],
                };

                if cmd_args.len() == 0 {
                    return Ok(Rc::new(Object::Array(RefCell::new(args_array))));
                }

                // skip the binary name
                cmd_args.next();

                // get a vector slice starting from index 1:
                let arg_str_objects: Vec<Rc<Object>> =
                    cmd_args.map(|arg| Rc::new(Object::Str(arg))).collect();

                args_array.elements = arg_str_objects;
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
                let env_value_res = env::var(env_key);
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
                let envs = env::vars();
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
                            elements: arr_vec
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

                let arch_string = env::consts::ARCH.to_string();
                let family_string = env::consts::FAMILY.to_string();
                let os_string = env::consts::OS.to_string();

                let mut platform_table = HashTable {
                    name: "platform".to_string(),
                    entries: HashMap::new(),
                };

                platform_table.set(
                    Rc::new(Object::Str("arch".to_string())),
                    Rc::new(Object::Str(arch_string))
                );

                platform_table.set(
                    Rc::new(Object::Str("family".to_string())),
                    Rc::new(Object::Str(family_string))
                );

                platform_table.set(
                    Rc::new(Object::Str("os".to_string())),
                    Rc::new(Object::Str(os_string))
                );

                return Ok(Rc::new(Object::HashTable(
                    RefCell::new(platform_table)
                )));
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
