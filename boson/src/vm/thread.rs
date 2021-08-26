use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use crate::api;
use crate::compiler::symtab;
use crate::types::closure;
use crate::types::object;
use crate::vm;
use crate::vm::global;

use api::Platform;
use closure::ClosureContext;
use global::GlobalPool;
use object::Object;
use symtab::ConstantPool;
use vm::BosonVM;

use crate::vm::errors;
use errors::VMError;

pub type ThreadReturnType = Result<Rc<Object>, VMError>;

pub struct ThreadParams {
    closure: Rc<ClosureContext>,
    params: Vec<Rc<Object>>,
    globals: GlobalPool,
    constants: ConstantPool
}

unsafe impl Send for ThreadParams {}

pub struct BosonThreads {
    pub thread_map: HashMap<u64, thread::JoinHandle<ThreadReturnType>>,
    // this counter will increment each time a new thread is created.
    // so it will have a new ID.
    pub current_count: u64,
}

impl BosonThreads {
    pub fn new_empty() -> BosonThreads {
        return BosonThreads {
            thread_map: HashMap::new(),
            current_count: 0,
        };
    }

    pub fn wait_and_return(&mut self, thread_id: u64) -> Result<ThreadReturnType, String> {
        if !self.thread_map.contains_key(&thread_id) {
            return Err(format!(
                "Cannot kill thread with ID {}, thread does not exist anymore.",
                thread_id
            ));
        }

        // kill the thread:
        let handle_ref = self.thread_map.remove(&thread_id);
        if handle_ref.is_none() {
            return Err(format!(
                "Cannot joing on a thread {} which does not exist.",
                thread_id
            ));
        }

        let join_result = handle_ref.unwrap().join();
        if join_result.is_err() {
            return Err(format!("{:?}", join_result.unwrap_err()));
        }

        return Ok(join_result.unwrap());
    }

    pub fn create_thread_sandbox(
        &mut self,
        thread_params: ThreadParams,
        platform: Arc<&Platform>,
    ) {

        let handle = thread::spawn( move || {
            
            let unwrapped_platform = Arc::try_unwrap(platform);
            

            let result = BosonVM::execute_sandbox(
                thread_params.closure, thread_params.params, 
                thread_params.globals, thread_params.constants
            );
        });
    }
}
