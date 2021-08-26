use std::rc::Rc;
use std::vec::Vec;

use crate::types;
use crate::vm::errors;

use errors::VMError;
use errors::VMErrorKind;
use types::object::Object;

use crate::config::GLOBAL_POOL_SIZE;

#[derive(Clone)]
pub struct GlobalPool {
    pub pool: Vec<Rc<Object>>,
    pub max_size: usize,
}

// make GlobalPool capable of sharing between threads.
// by design, GlobalPool will always be thread safe as each thread will
// get it's own copy of global pool.
// so this unsafe impl is just a cover-up to fool the compiler to make global pool
// sharebale across threads.
unsafe impl Send for GlobalPool {}

impl GlobalPool {
    pub fn new() -> GlobalPool {
        let mut pool = Vec::with_capacity(GLOBAL_POOL_SIZE);
        pool.resize(GLOBAL_POOL_SIZE, Rc::new(Object::Noval));
        return GlobalPool {
            pool: pool,
            max_size: GLOBAL_POOL_SIZE,
        };
    }

    pub fn get(&self, idx: usize) -> Option<Rc<Object>> {
        return self.pool.get(idx).cloned();
    }

    pub fn get_ref(&self, idx: usize) -> Option<&Rc<Object>> {
        return self.pool.get(idx);
    }

    pub fn set_object(&mut self, object: Rc<Object>, idx: usize) -> Option<VMError> {
        if idx >= self.max_size {
            return Some(VMError::new(
                format!("Index {} exceeds global pool size {}", idx, self.max_size),
                VMErrorKind::GlobalPoolSizeExceeded,
                None,
                0,
            ));
        }

        self.pool[idx] = object;
        return None;
    }

    pub fn set_none(&mut self, idx: usize) -> Option<VMError> {
        return self.set_object(Rc::new(Object::Noval), idx);
    }
}
