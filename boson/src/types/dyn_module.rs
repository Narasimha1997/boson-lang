use crate::types::object::Object;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct DynamicModuleInternalError {
    pub cause: String,
    pub message: String,
}

pub type DynamicModuleResult<Rc<Object>> = Result<Rc<Object>, DynamicModuleInternalError>;

/// also called BDSM! for short
pub trait BosonDynamicSubModule {
    /// open the module with some parameters, translates to FFI call internally
    /// loads Rust dynamic module and calls its open() implementation and registers an entry
    /// in VM's FFI reference table returning a handle (key in the symbol table)
    pub fn open(init_params: Rc<Object>) -> DynamicModuleResult;
    /// close the module with some parameters, this will translate to FFI dynamic module unload
    /// and removes the entry from Symbol table, cleaning up the memory.
    pub fn close(close_params: Rc<Object>) -> DynamicModuleResult;
    /// read (expect) something from the module, the module can do anything internally to serve the request
    pub fn read(read_params: Rc<Object>) -> DynamicModuleResult;
    /// write (expect) pass something to the module, the module can interpret this paramerer however it wants
    pub fn write(write_params: Rc<Object>) -> DynamicModuleResult;
}

