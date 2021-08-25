use std::thread;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::types::object;
use object::Object;

use crate::vm::errors;
use errors::VMError;


pub type ThreadReturnType = Result<Rc<Object>, VMError>;

pub struct BosonThreads {
    pub thread_map: HashMap<u64, thread::JoinHandle<ThreadReturnType>>,
    // this counter will increment each time a new thread is created.
    // so it will have a new ID.
    pub current_count: u64
}

impl BosonThreads {
    pub fn new_empty() -> BosonThreads {
        return BosonThreads {
            thread_map: HashMap::new(),
            current_count: 0
        }
    }

    /*pub fn wait_and_return(&mut self, thread_id: u64) -> Result<ThreadReturnType, String> {
        if !self.thread_map.contains_key(&thread_id) {
            return Err(format!(
                "Cannot kill thread with ID {}, thread does not exist anymore.",
                thread_id
            ));
        }

        // kill the thread:
        let handle_ref = self.thread_map.get(&thread_id).unwrap();
        let join_result = handle_ref.clone().join();
        if join_result.is_err() {
            return Err(format!("{:?}", join_result.unwrap_err()));
        }

        return Ok(join_result.unwrap());
    } */
}

