use crate::compiler::symtab::ConstantPool;
use crate::isa;
use crate::types::object;
use crate::vm::errors;
use crate::vm::frames;
use crate::vm::global;
use crate::vm::stack;

use std::rc::Rc;

use errors::VMError;
use errors::VMErrorKind;
use frames::ExecutionFrame;
use global::GlobalPool;
use isa::InstructionKind;
use object::Object;
use stack::DataStack;

pub struct Controls {}

impl Controls {
    pub fn jump(cf: &mut ExecutionFrame, pos: usize) -> Result<usize, VMError> {
        let error = cf.set_ip(pos);
        if error.is_some() {
            return Err(error.unwrap());
        }
        return Ok(pos);
    }

    pub fn store_global(
        gp: &mut GlobalPool,
        ds: &mut DataStack,
        pos: usize,
    ) -> Result<usize, VMError> {
        let obj_res = ds.pop_object(InstructionKind::IStoreGlobal);
        if obj_res.is_err() {
            return Err(obj_res.unwrap_err());
        }

        let error = gp.set_object(obj_res.unwrap(), pos);
        if error.is_some() {
            return Err(error.unwrap());
        }

        return Ok(pos);
    }

    pub fn load_global(gp: &GlobalPool, ds: &mut DataStack, pos: usize) -> Result<i64, VMError> {
        let object = gp.get(pos);
        if object.is_some() {
            let res = ds.push_object(object.unwrap(), InstructionKind::ILoadGlobal);
            return res;
        }

        return Err(VMError::new(
            format!("Index {} exceeds global pool size {}", pos, gp.max_size),
            VMErrorKind::GlobalPoolSizeExceeded,
            None,
            0,
        ));
    }

    pub fn load_constant(
        cp: &ConstantPool,
        ds: &mut DataStack,
        pos: usize,
    ) -> Result<i64, VMError> {
        let element = cp.get_object(pos).unwrap();
        let result = ds.push_object(element, InstructionKind::IConstant);
        return result;
    }
}
