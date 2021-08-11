use crate::compiler;
use crate::isa;
use crate::types::closure;
use crate::types::object;
use crate::types::subroutine;
use crate::vm::errors;

use std::rc::Rc;

use closure::ClosureContext;
use compiler::CompiledBytecode;
use errors::VMError;
use errors::VMErrorKind;
use isa::InstructionKind;
use isa::InstructionPacker;
use isa::Operands;
use object::Object;
use subroutine::Subroutine;


#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    exception_handler: Rc<ClosureContext>,
    finally_handler: Option<Rc<ClosureContext>>
}

pub type ExceptionHandleStack = Vec<ExceptionHandler>;

#[derive(Debug, Clone)]
pub struct ExecutionFrame {
    pub context: Rc<ClosureContext>,
    pub instruction_pointer: usize,
    pub base_pointer: usize,
    pub bytecode_size: usize,
    pub handlers: ExceptionHandleStack,
}

impl ExecutionFrame {
    pub fn new(context: Rc<ClosureContext>, base_pointer: usize) -> ExecutionFrame {
        let bytecode_size = context.as_ref().bytecode_size.clone();

        return ExecutionFrame {
            context: context,
            instruction_pointer: 0,
            base_pointer: base_pointer,
            bytecode_size: bytecode_size,
            handlers: vec![]
        };
    }

    pub fn new_closure(func: Rc<Subroutine>, free_objects: Vec<Rc<Object>>) -> Rc<Object> {
        let b_size = func.as_ref().bytecode.len();

        return Rc::new(Object::ClosureContext(Rc::new(ClosureContext {
            compiled_fn: func,
            bytecode_size: b_size,
            free_objects: free_objects,
        })));
    }

    pub fn get_free(&mut self, idx: usize, inst: InstructionKind) -> Result<Rc<Object>, VMError> {

        let free_object = self.context.free_objects.get(idx);
        if free_object.is_some() {
            return Ok(free_object.unwrap().clone());
        }

        return Err(VMError::new(
            format!("Free variable fetch out of bounds for index {}", idx),
            VMErrorKind::UnknownFreeVariable,
            Some(inst),
            0,
        ));
    }

    pub fn new_from_bytecode(
        bytecode: &CompiledBytecode,
        fn_name: String,
        n_locals: usize,
        n_params: usize,
    ) -> ExecutionFrame {
        let closure = ClosureContext {
            compiled_fn: Rc::new(Subroutine {
                name: fn_name,
                bytecode: bytecode.instructions.clone(),
                num_locals: n_locals,
                num_parameters: n_params,
            }),
            free_objects: vec![],
            bytecode_size: bytecode.instructions.len(),
        };

        let frame = ExecutionFrame::new(Rc::new(closure), 0);
        return frame;
    }

    pub fn get_ip(&self) -> usize {
        return self.instruction_pointer;
    }

    pub fn set_ip(&mut self, pos: usize) -> Option<VMError> {
        if pos >= self.bytecode_size {
            return Some(VMError::new(
                "Instruction pointer out of bounds.".to_string(),
                VMErrorKind::IPOutOfBounds,
                None,
                pos,
            ));
        }

        self.instruction_pointer = pos;
        return None;
    }

    pub fn read_current_instruction(&self) -> (InstructionKind, Operands, usize) {
        let bytecode = self.context.compiled_fn.as_ref().get_bytecode();
        let curr_instr_raw = bytecode[self.instruction_pointer];
        let decoded_opcode: InstructionKind = unsafe { ::std::mem::transmute(curr_instr_raw) };

        let (operands, next_offset) = InstructionPacker::decode_instruction(
            &decoded_opcode,
            &bytecode[self.instruction_pointer + 1..],
        );

        return (decoded_opcode, operands, next_offset);
    }

    pub fn farword_ip(&mut self, pos: usize) {
        self.instruction_pointer += pos + 1;
    }

    pub fn has_instructions(&self) -> bool {
        self.instruction_pointer < self.bytecode_size
    }

    pub fn get_bp(&self) -> usize {
        return self.base_pointer;
    }

    pub fn disassemble_ip(&self) -> String {
        let (inst, operands, _) = self.read_current_instruction();
        let encoded_string = inst.disasm_instruction(&operands);
        return encoded_string;
    }

    pub fn get_function_name(&self) -> String {
        self.context.as_ref().compiled_fn.name.clone()
    }
}

impl PartialEq for ExecutionFrame {
    fn eq(&self, other: &ExecutionFrame) -> bool {
        self.get_function_name() == other.get_function_name()
    }
}
