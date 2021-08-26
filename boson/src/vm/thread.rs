use std::collections::HashMap;
use std::rc::Rc;
use std::thread;

use crate::api;
use crate::compiler::symtab;
use crate::types::closure;
use crate::types::object;
use crate::vm;
use crate::vm::global;

use api::BosonLang;
use api::Platform;
use api::PlatformKind;
use closure::ClosureContext;
use global::GlobalPool;
use object::Object;
use symtab::ConstantPool;
use vm::BosonVM;

use crate::vm::errors;
use errors::VMError;

pub struct ThreadReturnType {
    pub result: Result<Rc<Object>, VMError>,
}

impl ThreadReturnType {
    pub fn new(result: Result<Rc<Object>, VMError>) -> ThreadReturnType {
        return ThreadReturnType { result: result };
    }
}

unsafe impl Send for ThreadReturnType {}

pub struct ThreadParams {
    closure: Rc<ClosureContext>,
    params: Vec<Rc<Object>>,
    globals: GlobalPool,
    constants: ConstantPool,
}

impl ThreadParams {
    pub fn new(
        closure: Rc<ClosureContext>,
        params: Vec<Rc<Object>>,
        globals: GlobalPool,
        constants: ConstantPool,
    ) -> ThreadParams {
        return ThreadParams {
            closure: closure,
            params: params,
            globals: globals,
            constants: constants,
        };
    }
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
            return Err("Failed to join thread.".to_string());
        }

        return Ok(join_result.unwrap());
    }

    pub fn create_thread_sandbox(
        &mut self,
        thread_params: ThreadParams,
        platform: &Platform,
    ) -> Result<u64, String> {
        let new_platform = match platform.platform_type {
            PlatformKind::Native => BosonLang::prepare_native_platform(),
            _ => {
                // TODO: re-iterate this section after web assembly support.
                return Err(format!(
                    "Platform {:?} not implemented.",
                    platform.platform_type
                ));
            }
        };

        let handle = thread::spawn(move || {
            let result = BosonVM::execute_sandbox(
                thread_params.closure,
                thread_params.params,
                &new_platform,
                thread_params.globals,
                thread_params.constants,
            );

            return ThreadReturnType::new(result);
        });

        // register the thread handle
        let thread_id = self.current_count;
        self.thread_map.insert(thread_id, handle);
        self.current_count += 1;
        return Ok(thread_id);
    }
}
