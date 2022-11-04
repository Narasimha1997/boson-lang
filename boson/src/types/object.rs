use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Rc;

use crate::types::array::Array;
use crate::types::buffer::Buffer;
use crate::types::builtins::BuiltinKind;
use crate::types::closure::ClosureContext;
use crate::types::dyn_module::NativeModuleRef;
use crate::types::exception::Exception;
use crate::types::hash::HashTable;
use crate::types::iter::ObjectIterator;
use crate::types::subroutine::Subroutine;
use crate::types::th::ThreadBlock;

// needed by attribute function call resolver
use crate::api;
use crate::compiler;
use crate::vm;

use api::Platform;
use compiler::symtab::ConstantPool;
use vm::ffi::BosonFFI;
use vm::global::GlobalPool;
use vm::stack::DataStack;
use vm::thread::BosonThreads;

pub trait AttributeResolver {
    fn attrs(&self) -> Vec<Rc<Object>>;
    fn resolve_set_attr(&self, _keys: &Vec<Rc<Object>>, _value: Rc<Object>) -> Option<String>;
    fn resolve_get_attr(&self, keys: &Vec<Rc<Object>>) -> Result<Rc<Object>, String>;
    fn resolve_call_attr(
        &mut self,
        keys: &Vec<Rc<Object>>,
        args: &Vec<Rc<Object>>,
        ds: &mut DataStack,
        platform: &mut Platform,
        gp: &mut GlobalPool,
        c: &mut ConstantPool,
        th: &mut BosonThreads,
        ffi: &mut BosonFFI,
    ) -> Result<Rc<Object>, String>;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Object {
    Noval,
    Int(i64),
    Bool(bool),
    Char(char),
    Str(String),
    Float(f64),
    Byte(u8),
    Subroutine(Rc<Subroutine>),
    ClosureContext(Rc<ClosureContext>),
    Array(RefCell<Array>),
    ByteBuffer(RefCell<Buffer>),
    HashTable(RefCell<HashTable>),
    Builtins(BuiltinKind),
    Iter(RefCell<ObjectIterator>),
    Exception(Rc<Exception>),
    Thread(RefCell<ThreadBlock>),
    NativeModule(RefCell<NativeModuleRef>),
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Int(i) => i.hash(state),
            Object::Bool(b) => b.hash(state),
            Object::Char(c) => c.hash(state),
            Object::Str(st) => st.hash(state),
            Object::Float(f) => f.to_string().hash(state),
            Object::Array(arr) => arr.borrow().hash(state),
            Object::HashTable(ht) => ht.borrow().hash(state),
            Object::Subroutine(sub) => sub.hash(state),
            Object::ClosureContext(ctx) => ctx.hash(state),
            Object::Exception(exc) => exc.hash(state),
            Object::Byte(byte) => byte.hash(state),
            Object::ByteBuffer(buff) => buff.borrow().hash(state),
            Object::NativeModule(native) => native.borrow().handle.hash(state),
            // No hash for iterators and thread block
            _ => "undef".hash(state),
        }
    }
}

impl Object {
    pub fn describe(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::Char(c) => c.to_string(),
            Object::Str(st) => st.clone(),
            Object::Float(f) => f.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Byte(byte) => byte.to_string(),
            Object::Array(arr) => arr.borrow().describe(),
            Object::HashTable(ht) => ht.borrow().describe(),
            Object::Subroutine(sub) => sub.describe(),
            Object::ClosureContext(ctx) => ctx.describe(),
            Object::Iter(it) => it.borrow().describe(),
            Object::Builtins(_) => "builtin".to_string(),
            Object::Exception(exc) => exc.describe(),
            Object::ByteBuffer(buff) => buff.borrow().describe(),
            Object::Thread(th) => th.borrow().describe(),
            Object::NativeModule(native) => native.borrow().describe(),
            _ => String::from("undef"),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Object::Int(_) => "int".to_string(),
            Object::Char(_) => "char".to_string(),
            Object::Str(_) => "string".to_string(),
            Object::Float(_) => "float".to_string(),
            Object::Bool(_) => "bool".to_string(),
            Object::Byte(_) => "raw".to_string(),
            Object::Array(_) => "array".to_string(),
            Object::ByteBuffer(_) => "bytes".to_string(),
            Object::HashTable(_) => "hashmap".to_string(),
            Object::Iter(_) => "iter".to_string(),
            Object::Exception(_) => "exception".to_string(),
            Object::Builtins(_) | Object::Subroutine(_) | Object::ClosureContext(_) => {
                "func".to_string()
            }
            Object::NativeModule(_) => "native".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Object::Bool(val) => val.clone(),
            Object::Noval => false,
            Object::Str(str) => *str != "",
            Object::Int(i) => *i != 0,
            Object::Char(c) => *c != '\0',
            Object::Array(a) => a.borrow().elements.len() != 0,
            Object::ByteBuffer(buff) => buff.borrow().length != 0,
            Object::HashTable(h) => h.borrow().entries.len() != 0,
            Object::Iter(it) => it.borrow().has_next(),
            Object::Byte(b) => *b != 0,
            _ => true,
        }
    }

    pub fn get_indexed(&self, idx: &Rc<Object>) -> Result<Rc<Object>, String> {
        match (self, idx.as_ref()) {
            (Object::Array(arr), Object::Int(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }
                let result = arr.borrow().get_object(*i as usize);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Object::ByteBuffer(buffer), Object::Int(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }

                let result = buffer.borrow().get_byte_at(*i as usize);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                // wrap the result into raw type
                return Ok(Rc::new(Object::Byte(result.unwrap())));
            }

            (Object::HashTable(ht), _) => {
                let result = ht.borrow().get(idx);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                return Ok(result.unwrap());
            }
            (Object::Str(st), Object::Int(i)) => {
                if *i < 0 {
                    return Err(format!("Index {} must be greater than or equal to zero", i));
                }

                let ch = st.chars().nth(*i as usize);
                if ch.is_none() {
                    return Err(format!("String \"{}\" index out of bounds for {}", st, i));
                }

                return Ok(Rc::new(Object::Char(ch.unwrap())));
            }
            _ => {
                return Err(format!(
                    "Object of type {} does not support indexing of type {}",
                    self.get_type(),
                    idx.get_type()
                ));
            }
        }
    }

    pub fn set_indexed(&mut self, idx: &Rc<Object>, data: Rc<Object>) -> Option<String> {
        match (&self, idx.as_ref()) {
            (Object::Array(arr), Object::Int(i)) => {
                if *i < 0 {
                    return Some(format!("Index {} must be greater than or equal to zero", i));
                }

                // set object at index:
                let result = arr.borrow_mut().set_object(*i as usize, data);
                if result.is_some() {
                    return Some(result.unwrap());
                }
                return None;
            }
            (Object::ByteBuffer(buffer), Object::Int(i)) => {
                if *i < 0 {
                    return Some(format!("Index {} must be greater than or equal to zero", i));
                }

                match data.as_ref() {
                    Object::Byte(byte) => {
                        let result = buffer.borrow_mut().set_byte_at(*i as usize, *byte);
                        if result.is_none() {
                            return Some(format!("Index {} out of bounds", i));
                        }

                        return None;
                    }
                    _ => {
                        return Some(format!(
                            "Object of type RawBuffer does not support assignment of type {}",
                            data.get_type()
                        ));
                    }
                }
            }
            (Object::HashTable(ht), _) => {
                ht.borrow_mut().set(idx.clone(), data);
                return None;
            }
            _ => {
                return Some(format!(
                    "Object of type {} does not support index assignment of type {}",
                    self.get_type(),
                    idx.get_type()
                ))
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

impl AttributeResolver for Object {
    fn resolve_set_attr(&self, _keys: &Vec<Rc<Object>>, _value: Rc<Object>) -> Option<String> {
        // to be implemented
        return None;
    }

    fn resolve_get_attr(&self, keys: &Vec<Rc<Object>>) -> Result<Rc<Object>, String> {
        let f_key = &keys[0];

        match f_key.as_ref() {
            Object::Str(st) => {
                match st.as_ref() {
                    // base attributes
                    "__address__" => return Ok(Rc::new(Object::Str(format!("{:p}", self)))),
                    "__size__" => return Ok(Rc::new(Object::Int(mem::size_of::<Self>() as i64))),
                    _ => {}
                }

                match self {
                    Object::HashTable(ht) => {
                        return ht.borrow().resolve_get_attr(keys);
                    }
                    _ => {
                        return Err(format!(
                            "Object of type {} does not have attribute resolver.",
                            self.get_type()
                        ))
                    }
                }
            }

            _ => {}
        }

        return Ok(Rc::new(Object::Noval));
    }

    fn resolve_call_attr(
        &mut self,
        keys: &Vec<Rc<Object>>,
        args: &Vec<Rc<Object>>,
        ds: &mut DataStack,
        platform: &mut Platform,
        gp: &mut GlobalPool,
        c: &mut ConstantPool,
        th: &mut BosonThreads,
        ffi: &mut BosonFFI,
    ) -> Result<Rc<Object>, String> {
        match self {
            Object::HashTable(ht) => {
                return ht
                    .borrow_mut()
                    .resolve_call_attr(&keys, &args, ds, platform, gp, c, th, ffi)
            }
            Object::NativeModule(nt) => {
                return nt
                    .borrow_mut()
                    .resolve_call_attr(&keys, &args, ds, platform, gp, c, th, ffi)
            }
            _ => {
                return Err(format!(
                    "No function attributes found for type {}",
                    self.get_type()
                ))
            }
        }
    }

    // attributes:
    fn attrs(&self) -> Vec<Rc<Object>> {
        let mut main_attrs = vec![
            Rc::new(Object::Str(String::from("__address__"))),
            Rc::new(Object::Str(String::from("__size__"))),
        ];

        match self {
            Object::HashTable(ht) => {
                main_attrs.extend(ht.borrow().attrs());
            }
            _ => {}
        }

        return main_attrs;
    }
}

// make Object capable of sharing between threads.
// by design, Object will always be thread safe as each thread will
// get it's own copy of Object.
// so this unsafe impl is just a cover-up to fool the compiler to make Object
// sharebale across threads.
unsafe impl Send for Object {}
