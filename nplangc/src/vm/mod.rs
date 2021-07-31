pub mod alu;
pub mod controls;
pub mod errors;
pub mod frames;
pub mod global;
pub mod stack;

use std::cell::RefCell;
use std::rc::Rc;

use controls::Controls;
use errors::VMError;
use errors::VMErrorKind;
use frames::ExecutionFrame;
use global::GlobalPool;
use stack::CallStack;
use stack::DataStack;

use crate::compiler::symtab::ConstantPool;
use crate::compiler::CompiledBytecode;
use crate::isa::InstructionKind;
use crate::types::object;

use object::Object;

pub struct BosonVM {
    pub constants: ConstantPool,
    pub globals: GlobalPool,
    pub data_stack: DataStack,
    pub call_stack: CallStack,
}

impl BosonVM {
    pub fn new(bytecode: &CompiledBytecode) -> BosonVM {
        let main_frame = ExecutionFrame::new_from_bytecode(bytecode, "main".to_string(), 0, 0);

        let mut call_stack = CallStack::new();
        let data_stack = DataStack::new();

        let _ = call_stack.push_frame(RefCell::new(main_frame));

        let globals = GlobalPool::new();

        return BosonVM {
            constants: bytecode.constant_pool.clone(),
            call_stack: call_stack,
            data_stack: data_stack,
            globals: globals,
        };
    }

    pub fn eval_bytecode(&mut self) -> Result<Rc<Object>, VMError> {
        while self.call_stack.top().has_instructions() {
            let mut frame = self.call_stack.top();

            let (inst, operands, next) = frame.read_current_instruction();

            match inst {
                // illegal and NoOp
                InstructionKind::INoOp => {
                    frame.farword_ip(next);
                }
                InstructionKind::IIllegal => {
                    return Err(VMError::new(
                        "VM encountered illegal instruction".to_string(),
                        VMErrorKind::IllegalOperation,
                        Some(InstructionKind::IIllegal),
                        0,
                    ));
                }

                // data load and store instructions:
                InstructionKind::IConstant => {
                    let const_pos = operands[0];
                    let result =
                        Controls::load_constant(&self.constants, &mut self.data_stack, const_pos);

                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }

                    frame.farword_ip(next);
                }

                InstructionKind::IStoreGlobal => {
                    let store_pos = operands[0];
                    let result =
                        Controls::store_global(&mut self.globals, &mut self.data_stack, store_pos);

                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }

                    frame.farword_ip(next);
                }

                InstructionKind::ILoadGlobal => {
                    let store_pos = operands[0];
                    let result =
                        Controls::load_global(&mut self.globals, &mut self.data_stack, store_pos);

                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }

                    frame.farword_ip(next);
                }

                // Binary operations:
                InstructionKind::IAdd
                | InstructionKind::ISub
                | InstructionKind::IMul
                | InstructionKind::IDiv
                | InstructionKind::IMod
                | InstructionKind::IAnd
                | InstructionKind::IOr
                | InstructionKind::ILAnd
                | InstructionKind::ILOr
                | InstructionKind::ILGt
                | InstructionKind::ILGte
                | InstructionKind::ILLTe
                | InstructionKind::ILLt
                | InstructionKind::ILEq
                | InstructionKind::ILNe => {

                    let error = Controls::execute_binary_op(&inst, &mut self.data_stack);
                    if error.is_some() {
                        return Err(error.unwrap());
                    }

                    frame.farword_ip(next);
                }

                // unary operators:
                InstructionKind::ILNot | InstructionKind::INeg => {
                    let error = Controls::execute_unary_op(&inst, &mut self.data_stack);
                    if error.is_some() {
                        return Err(error.unwrap());
                    }

                    frame.farword_ip(next);
                }

                _ => {
                    return Err(VMError::new(
                        format!("{} not yet implemented", inst.as_string()),
                        VMErrorKind::InstructionNotImplemented,
                        Some(inst),
                        0,
                    ));
                }
            }
        }

        return Ok(Rc::new(Object::Noval));
    }

    pub fn dump_globals(&self) -> String {
        let mut result = String::new();
        let mut idx = 0;
        for obj in &self.globals.pool {
            match obj.as_ref() {
                Object::Noval => {}
                _ => {
                    let repr = obj.as_ref().describe();
                    result.push_str(&format!("{:0>8x} {}\n", idx, repr));
                    idx += 1;
                }
            }
        }

        return result;
    }

    pub fn dump_ds(&self) -> String {
        let mut result = String::new();
        let mut idx = 0;
        for obj in &self.data_stack.stack {
            let repr = obj.as_ref().describe();
            result.push_str(&format!("{:0>8x} {}\n", idx, repr));
            idx += 1;
        }

        return result;
    }
}
