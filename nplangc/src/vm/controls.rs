use crate::compiler::symtab::ConstantPool;
use crate::isa;
use crate::types::object;
use crate::vm::alu;
use crate::vm::errors;
use crate::vm::frames;
use crate::vm::global;
use crate::vm::stack;

use std::rc::Rc;

use alu::Arithmetic;
use alu::Bitwise;
use alu::Comparision;
use alu::Logical;
use errors::ISAError;
use errors::ISAErrorKind;
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

    pub fn get_binary_operands(
        ds: &mut DataStack,
        inst: &InstructionKind,
    ) -> Result<(Rc<Object>, Rc<Object>), VMError> {
        let right_pop = ds.pop_object(inst.clone());
        if right_pop.is_err() {
            return Err(right_pop.unwrap_err());
        }

        let left_pop = ds.pop_object(inst.clone());
        if right_pop.is_err() {
            return Err(right_pop.unwrap_err());
        }

        return Ok((right_pop.unwrap(), left_pop.unwrap()));
    }

    pub fn execute_binary_op(inst: &InstructionKind, ds: &mut DataStack) -> Option<VMError> {
        let operands_result = Controls::get_binary_operands(ds, inst);
        if operands_result.is_err() {
            return Some(operands_result.unwrap_err());
        }

        let (left, right) = operands_result.unwrap();

        let result = match inst {
            InstructionKind::IAdd => Arithmetic::add(&left, &right),
            InstructionKind::ISub => Arithmetic::sub(&left, &right),
            InstructionKind::IMul => Arithmetic::mul(&left, &right),
            InstructionKind::IDiv => Arithmetic::div(&left, &right),
            InstructionKind::IMod => Arithmetic::modulus(&left, &right),
            InstructionKind::IAnd => Bitwise::and(&left, &right),
            InstructionKind::IOr => Bitwise::or(&left, &right),
            InstructionKind::ILOr => Logical::or(&left, &right),
            InstructionKind::ILAnd => Logical::and(&left, &right),
            InstructionKind::ILGt => Comparision::gt(&left, &right),
            InstructionKind::ILGte => Comparision::gte(&left, &right),
            InstructionKind::ILLt => Comparision::lt(&left, &right),
            InstructionKind::ILLTe => Comparision::lte(&left, &right),
            InstructionKind::ILEq => Comparision::eq(&left, &right),
            InstructionKind::ILNe => Comparision::neq(&left, &right),

            _ => Err(ISAError::new(
                format!("{} is not a binary op", inst.as_string()),
                ISAErrorKind::InvalidOperation,
            )),
        };

        // push result on to stack:
        if result.is_err() {
            return Some(VMError::new_from_isa_error(
                &result.unwrap_err(),
                inst.clone(),
            ));
        }

        let result_obj = result.unwrap();

        // push result to stack:
        let result_push = ds.push_object(result_obj, inst.clone());
        if result_push.is_err() {
            return Some(result_push.unwrap_err());
        }
        return None;
    }
}
