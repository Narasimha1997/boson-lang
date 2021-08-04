use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::types::array;
use crate::types::object;

use array::Array;
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
            _ => "undef".to_string(),
        }
    }

    pub fn exec(&self, args: Vec<Rc<Object>>) -> Result<Rc<Object>, String> {
        match self {
            BuiltinKind::Print => {

                if args.len() == 0 {
                    return Err(
                        "print() takes atleast one argument, 0 provided".to_string()
                    );
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
                        Ok(Rc::new(Object::Int(arr.as_ref().elements.len() as i64)))
                    }
                    Object::HashTable(ht) => {
                        Ok(Rc::new(Object::Int(ht.as_ref().entries.len() as i64)))
                    }
                    _ => Err(format!("len() cannot be applied on {}", obj.get_type())),
                }
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

                return Ok(Rc::new(Object::Array(Rc::new(Array {
                    name: "todo".to_string(),
                    elements: strings,
                }))));
            }

            BuiltinKind::TimeUnix => {
                if args.len() != 0 {
                    return Err(format!(
                        "epoch_time() takes zero arguments, {} provided",
                        args.len()
                    ));
                }

                let epoch_time_res = SystemTime::now().duration_since(UNIX_EPOCH);
                if epoch_time_res.is_err() {
                    return Err("Failed to fetch UNIX epoch time.".to_string());
                }

                let epoch_time = epoch_time_res.unwrap();

                return Ok(Rc::new(
                    Object::Float(epoch_time.as_secs_f64())
                ));
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
