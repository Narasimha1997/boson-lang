use crate::compiler;
use crate::isa;
use crate::types::closure;
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
use subroutine::Subroutine;

pub struct ExecutionFrame {
    pub context: Rc<ClosureContext>,
    pub instruction_pointer: usize,
    pub base_pointer: usize,
    pub bytecode_size: usize,
}

impl ExecutionFrame {
    pub fn new(context: Rc<ClosureContext>) -> ExecutionFrame {
        let bytecode_size = context.as_ref().bytecode_size.clone();

        return ExecutionFrame {
            context: context,
            instruction_pointer: 0,
            base_pointer: 0,
            bytecode_size: bytecode_size,
        };
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

        let frame = ExecutionFrame::new(Rc::new(closure));
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
}
