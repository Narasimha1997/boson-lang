pub mod frames;
pub mod errors;
pub mod stack;
pub mod global;

use crate::compiler::CompiledBytecode;
use crate::types::closure::ClosureContext;
use crate::types::subroutine::Subroutine;

use std::rc::Rc;

pub fn test_reading(bytecode: &CompiledBytecode) {
    // create a new closure
    let closure = ClosureContext{
        compiled_fn: Rc::new(Subroutine{
            name: "main".to_string(),
            bytecode: bytecode.instructions.clone(),
            num_locals: 0,
            num_parameters: 0
        }),
        free_objects: vec![],
        bytecode_size: bytecode.instructions.len()
    };

    let mut frame = frames::ExecutionFrame::new(Rc::new(closure));

    while frame.has_instructions() {
        let (_, _, next) = frame.read_current_instruction();
        println!("{}", frame.disassemble_ip());
        frame.farword_ip(next);
    }
}