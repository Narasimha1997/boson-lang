use crate::types::object;

use object::AttributeResolver;
use object::Object;

use std::rc::Rc;

// needed by attribute function call resolver
use crate::api;
use crate::compiler;
use crate::vm;

use api::Platform;
use compiler::symtab::ConstantPool;
use vm::ffi::BosonFFI;
use vm::global::GlobalPool;
use vm::thread::BosonThreads;
use vm::stack::DataStack;

#[derive(Debug, Clone)]
pub struct DynamicModuleInternalError {
    pub cause: String,
    pub message: String,
}

pub type DynamicModuleResult = Result<Rc<Object>, DynamicModuleInternalError>;

/// also called BDSM! for short
pub trait BosonDynamicSubModule {
    /// open the module with some parameters, translates to FFI call internally
    /// loads Rust dynamic module and calls its open() implementation and registers an entry
    /// in VM's FFI reference table returning a handle (key in the symbol table)
    fn open(init_params: Rc<Object>) -> DynamicModuleResult;
    /// close the module with some parameters, this will translate to FFI dynamic module unload
    /// and removes the entry from Symbol table, cleaning up the memory.
    fn close(close_params: Rc<Object>) -> DynamicModuleResult;
    /// read (expect) something from the module, the module can do anything internally to serve the request
    fn read(read_params: Rc<Object>) -> DynamicModuleResult;
    /// write (expect) pass something to the module, the module can interpret this paramerer however it wants
    fn write(write_params: Rc<Object>) -> DynamicModuleResult;

    /// exec - execute any generic function implemented by the module provider
    fn exec(func: String, params: Rc<Object>) -> DynamicModuleResult;
}

pub type OpenFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type CloseFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type ReadFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type WriteFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type ExecFunctionSymbol = unsafe extern "Rust" fn(String, &Vec<Rc<Object>>) -> DynamicModuleResult;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NativeModuleRef {
    pub handle: i64,
}

impl NativeModuleRef {
    pub fn new(handle: i64) -> Self {
        NativeModuleRef { handle }
    }

    pub fn describe(&self) -> String {
        format!("NativeModule({})", self.handle)
    }
}

impl AttributeResolver for NativeModuleRef {
    fn resolve_get_attr(&self, _keys: &Vec<Rc<Object>>) -> Result<Rc<Object>, String> {
        Ok(Rc::new(Object::Noval))
    }

    fn resolve_set_attr(&self, _keys: &Vec<Rc<Object>>, _value: Rc<Object>) -> Option<String> {
        None
    }

    fn resolve_call_attr(
        &mut self,
        keys: &Vec<Rc<Object>>,
        args: &Vec<Rc<Object>>,
        _ds: &mut DataStack,
        _platform: &mut Platform,
        _gp: &mut GlobalPool,
        _c: &mut ConstantPool,
        _th: &mut BosonThreads,
        ffi: &mut BosonFFI,
    ) -> Result<Rc<Object>, String> {
        let method = keys[0].as_ref();
        match method {
            Object::Str(method_str) => {
                let ffi_exec_result = ffi.exec(self.handle as usize, method_str.clone(), args);
                if ffi_exec_result.is_err() {
                    return Err(ffi_exec_result.unwrap_err());
                }

                let ffi_result = ffi_exec_result.unwrap();
                if ffi_result.is_err() {
                    return Err(ffi_result.unwrap_err().message);
                }

                return Ok(ffi_result.unwrap());
            }
            _ => return Err(format!("invalid method type, found {}", method.get_type())),
        }
    }

    fn attrs(&self) -> Vec<Rc<Object>> {
        // todo: obtain the list of methods from the native module and pass them
        vec![]
    }
}
