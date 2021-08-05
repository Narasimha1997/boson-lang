use crate::compiler::symtab::ConstantPool;
use crate::isa;
use crate::types::array;
use crate::types::builtins;
use crate::types::hash;
use crate::types::object;
use crate::vm::alu;
use crate::vm::errors;
use crate::vm::frames;
use crate::vm::global;
use crate::vm::stack;

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::rc::Rc;

use alu::Arithmetic;
use alu::Bitwise;
use alu::Comparision;
use alu::Logical;
use array::Array;
use builtins::BuiltinKind;
use errors::ISAError;
use errors::ISAErrorKind;
use errors::VMError;
use errors::VMErrorKind;
use frames::ExecutionFrame;
use global::GlobalPool;
use hash::HashTable;
use isa::InstructionKind;
use object::Object;
use stack::DataStack;

pub struct Controls {}

impl Controls {
    pub fn jump(cf: &mut RefMut<ExecutionFrame>, pos: usize) -> Result<usize, VMError> {
        let error = cf.set_ip(pos);
        if error.is_some() {
            return Err(error.unwrap());
        }
        return Ok(pos);
    }

    pub fn jump_not_truthy(
        cf: &mut RefMut<ExecutionFrame>,
        ds: &mut DataStack,
        pos: usize,
    ) -> Result<bool, VMError> {
        let popped_res = ds.pop_object(InstructionKind::INotJump);
        if popped_res.is_err() {
            return Err(popped_res.unwrap_err());
        }


        let popped_obj = popped_res.unwrap();
        if !popped_obj.as_ref().is_true() {
            let jmp_result = Controls::jump(cf, pos);
            if jmp_result.is_err() {
                return Err(jmp_result.unwrap_err());
            }

            return Ok(true);
        }

        return Ok(false);
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

    pub fn load_free(
        ds: &mut DataStack,
        frame: &mut RefMut<ExecutionFrame>,
        idx: usize,
    ) -> Option<VMError> {
        let free_obj_result = frame.get_free(idx, InstructionKind::ILoadFree);
        if free_obj_result.is_err() {
            return Some(free_obj_result.unwrap_err());
        }

        let free_object = free_obj_result.unwrap();
        // push it to the stack:
        let push_result = ds.push_object(free_object, InstructionKind::ILoadFree);

        if push_result.is_err() {
            return Some(push_result.unwrap_err());
        }

        return None;
    }

    pub fn store_local(
        ds: &mut DataStack,
        pos: usize,
        f: &RefMut<ExecutionFrame>,
    ) -> Result<i64, VMError> {
        let popped_result = ds.pop_object(InstructionKind::IStoreLocal);
        if popped_result.is_err() {
            return Err(popped_result.unwrap_err());
        }

        let bp = f.get_bp();
        ds.stack[bp + pos] = popped_result.unwrap();

        return Ok((bp + pos) as i64);
    }

    pub fn load_local(
        ds: &mut DataStack,
        pos: usize,
        f: &RefMut<ExecutionFrame>,
    ) -> Result<i64, VMError> {
        let bp = f.get_bp();
        let local_object_res = ds.stack.get(bp + pos);
        if local_object_res.is_none() {
            return Err(VMError::new(
                "Stack overflow!".to_string(),
                VMErrorKind::DataStackOverflow,
                Some(InstructionKind::ILoadLocal),
                0,
            ));
        }

        let local_object = local_object_res.unwrap().clone();
        // push the object to stack:
        let push_result = ds.push_object(local_object, InstructionKind::ILoadLocal);
        if push_result.is_err() {
            return Err(push_result.unwrap_err());
        }

        return Ok(push_result.unwrap());
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

        return Ok((left_pop.unwrap(), right_pop.unwrap()));
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

    fn pop_n(
        ds: &mut DataStack,
        n: usize,
        inst: &InstructionKind,
    ) -> Result<Vec<Rc<Object>>, VMError> {
        let mut objs = vec![];

        for _ in 0..n {
            let popped = ds.pop_object(inst.clone());
            if popped.is_err() {
                return Err(popped.unwrap_err());
            }

            let obj = popped.unwrap();
            objs.push(obj);
        }

        return Ok(objs);
    }

    pub fn get_index_value(ds: &mut DataStack) -> Option<VMError> {
        let popped_idx_result = ds.pop_object(InstructionKind::IGetIndex);
        if popped_idx_result.is_err() {
            return Some(popped_idx_result.unwrap_err());
        }

        let popped_left_result = ds.pop_object(InstructionKind::IGetIndex);
        if popped_idx_result.is_err() {
            return Some(popped_left_result.unwrap_err());
        }

        let index_obj = popped_idx_result.unwrap();
        let left_obj = popped_left_result.unwrap();

        // perform indexing:
        let index_result = left_obj.get_indexed(&index_obj);
        if index_result.is_err() {
            return Some(VMError::new(
                index_result.unwrap_err(),
                VMErrorKind::IndexError,
                Some(InstructionKind::IGetIndex),
                0
            ));
        }

        let push_result = ds.push_object(index_result.unwrap(), InstructionKind::IGetIndex);
        if push_result.is_err() {
            return Some(push_result.unwrap_err());
        }

        return None;
    }

    pub fn load_builtin(ds: &mut DataStack, idx: usize) -> Result<i64, VMError> {
        let builtin_kind = BuiltinKind::get_by_index(idx);
        if builtin_kind.is_none() {
            return Err(VMError::new(
                format!("Unresolved built-in function with index {}", idx),
                VMErrorKind::UnresolvedBuiltinFunction,
                Some(InstructionKind::ILoadBuiltIn),
                0,
            ));
        }

        // push to the stack
        let obj = Rc::new(Object::Builtins(builtin_kind.unwrap()));
        let push_res = ds.push_object(obj, InstructionKind::ILoadBuiltIn);
        if push_res.is_err() {
            return Err(push_res.unwrap_err());
        }

        return Ok(push_res.unwrap());
    }

    pub fn execute_return(
        ds: &mut DataStack,
        frame: &Ref<ExecutionFrame>,
        has_val: bool,
    ) -> Option<VMError> {
        let mut returned_obj: Rc<Object> = Rc::new(Object::Noval);
        if has_val {
            let returned_obj_res = ds.pop_object(InstructionKind::IRetVal);
            if returned_obj_res.is_err() {
                return Some(returned_obj_res.unwrap_err());
            }

            returned_obj = returned_obj_res.unwrap();
        }

        let local_boundary = frame.get_bp();
        // clear off the stack till this point:
        ds.stack.truncate(local_boundary);
        ds.stack_pointer = local_boundary as i64;
        let push_res = ds.push_object(returned_obj, InstructionKind::IRetVal);
        if push_res.is_err() {
            return Some(push_res.unwrap_err());
        }

        return None;
    }

    pub fn execute_call(
        inst: &InstructionKind,
        ds: &mut DataStack,
        n_args: usize,
    ) -> Result<Option<RefCell<ExecutionFrame>>, VMError> {
        // pop the function:

        let popped = ds.pop_object(inst.clone());
        if popped.is_err() {
            return Err(popped.unwrap_err());
        }

        let popped_obj = popped.unwrap();
        match popped_obj.as_ref() {
            Object::Builtins(func) => {
                // pop the arguments:
                let popped_args = Controls::pop_n(ds, n_args, inst);
                if popped_args.is_err() {
                    return Err(popped_args.unwrap_err());
                }

                let mut args = popped_args.unwrap();
                args.reverse();
                // call the builtin:
                let exec_result = func.exec(args);
                if exec_result.is_err() {
                    return Err(VMError::new(
                        exec_result.unwrap_err(),
                        VMErrorKind::BuiltinFunctionError,
                        Some(inst.clone()),
                        0,
                    ));
                }

                let result_obj = exec_result.unwrap();
                if result_obj.is_true() {
                    let push_res = ds.push_object(result_obj, inst.clone());
                    if push_res.is_err() {
                        return Err(push_res.unwrap_err());
                    }
                }

                return Ok(None);
            }
            Object::ClosureContext(ctx) => {
                let closure = ctx.as_ref();
                let subroutine = closure.compiled_fn.as_ref();

                if subroutine.num_parameters != n_args {
                    return Err(VMError::new(
                        format!(
                            "Function {} expects {} arguments, given {}",
                            subroutine.name, subroutine.num_parameters, n_args
                        ),
                        VMErrorKind::FunctionArgumentsError,
                        Some(InstructionKind::ICall),
                        0,
                    ));
                }

                let frame_bp = if ds.stack_pointer < n_args as i64 {
                    (ds.stack_pointer + 1) as usize - n_args
                } else {
                    (ds.stack_pointer) as usize - n_args
                };

                // allocate the stack for local variables and frame:
                let new_frame = ExecutionFrame::new(Rc::new(closure.clone()), frame_bp);

                let n_locals = closure.compiled_fn.num_locals;
                let n_params = closure.compiled_fn.num_parameters;
                let mut local_space = vec![];
                local_space.resize(n_locals - n_params, Rc::new(Object::Noval));

                // push the local space on to the stack
                let push_res = ds.push_objects(InstructionKind::ICall, local_space);
                if push_res.is_err() {
                    return Err(push_res.unwrap_err());
                }

                // set the new stack pointer:
                ds.stack_pointer = (new_frame.base_pointer + n_locals) as i64;
                return Ok(Some(RefCell::new(new_frame)));
            }
            _ => {
                return Err(VMError::new(
                    format!("Cannot call {}", popped_obj.as_ref().describe()),
                    VMErrorKind::StackCorruption,
                    Some(inst.clone()),
                    0,
                ));
            }
        }
    }

    pub fn execute_unary_op(inst: &InstructionKind, ds: &mut DataStack) -> Option<VMError> {
        let pop_result = ds.pop_object(inst.clone());
        if pop_result.is_err() {
            return Some(pop_result.unwrap_err());
        }

        let obj = pop_result.unwrap();

        let result = match inst {
            InstructionKind::INeg => Bitwise::not(&obj),
            InstructionKind::ILNot => Logical::not(&obj),
            _ => Err(ISAError::new(
                format!("{} is not a unary op", inst.as_string()),
                ISAErrorKind::InvalidOperation,
            )),
        };

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

    pub fn build_array(
        inst: &InstructionKind,
        ds: &mut DataStack,
        length: usize,
    ) -> Result<i64, VMError> {
        let popped_res = Controls::pop_n(ds, length, inst);
        if popped_res.is_err() {
            return Err(popped_res.unwrap_err());
        }

        let mut popped = popped_res.unwrap();
        popped.reverse();

        let array = Array {
            name: "todo".to_string(),
            elements: popped,
        };

        let array_obj = Rc::new(Object::Array(Rc::new(array)));

        // push the array on to the stack:
        let push_res = ds.push_object(array_obj, inst.clone());
        if push_res.is_err() {
            return Err(push_res.unwrap_err());
        }

        return Ok(push_res.unwrap());
    }

    pub fn build_hash(
        inst: &InstructionKind,
        ds: &mut DataStack,
        length: usize,
    ) -> Result<i64, VMError> {
        let popped_res = Controls::pop_n(ds, length, inst);
        if popped_res.is_err() {
            return Err(popped_res.unwrap_err());
        }

        let mut hash_table = HashMap::new();
        let mut popped = popped_res.unwrap();
        popped.reverse();

        let mut idx = 0;
        while idx < length {
            let key = popped[idx].clone();
            idx += 1;
            let value = popped[idx].clone();
            idx += 1;
            hash_table.insert(key, value);
        }

        let ht = HashTable {
            name: "todo".to_string(),
            entries: hash_table,
        };

        let ht_obj = Rc::new(Object::HashTable(Rc::new(ht)));
        let push_res = ds.push_object(ht_obj, inst.clone());

        if push_res.is_err() {
            return Err(push_res.unwrap_err());
        }

        return Ok(push_res.unwrap());
    }

    pub fn create_closure(
        ds: &mut DataStack,
        constants: &ConstantPool,
        n_free: usize,
        func_idx: usize,
    ) -> Option<VMError> {
        // pop off the free objects
        let popped_res = Controls::pop_n(ds, n_free, &InstructionKind::IClosure);
        if popped_res.is_err() {
            return Some(popped_res.unwrap_err());
        }
        // get free objects:
        let free_objects = popped_res.unwrap();

        // retrive  the function from constant pool:
        let function_res = constants.get_object(func_idx);
        if function_res.is_none() {
            return Some(VMError::new(
                "Error fetching unknown constant".to_string(),
                VMErrorKind::InvalidGlobalIndex,
                Some(InstructionKind::IClosure),
                0,
            ));
        }

        let function = function_res.unwrap();

        match function.as_ref() {
            Object::Subroutine(sub) => {
                // create a closure:
                let closure_obj = ExecutionFrame::new_closure(sub.clone(), free_objects);
                // load the closure on data-stack:
                let push_res = ds.push_object(closure_obj, InstructionKind::IClosure);
                if push_res.is_err() {
                    return Some(push_res.unwrap_err());
                }
            }
            _ => {
                return Some(VMError::new(
                    format!(
                        "Only functions can be loaded as closure not {}",
                        function.as_ref().get_type()
                    ),
                    VMErrorKind::InvalidGlobalIndex,
                    Some(InstructionKind::IClosure),
                    0,
                ));
            }
        }

        return None;
    }

    pub fn raise_assertion_error(ds: &mut DataStack) -> Option<VMError> {
        let popped_result = ds.pop_object(InstructionKind::IAssertFail);
        if popped_result.is_err() {
            return Some(popped_result.unwrap_err());
        }

        let assert_obj = popped_result.unwrap();

        let assert_fail_str = format!(
            "Assertion Failed: {}", assert_obj.describe()
        );

        return Some(VMError::new(
            assert_fail_str,
            VMErrorKind::AssertionError,
            Some(InstructionKind::IAssertFail),
            0
        ));
    }
}
