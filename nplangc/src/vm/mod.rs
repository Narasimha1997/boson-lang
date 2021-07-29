pub mod frames;
pub mod errors;
pub mod stack;
pub mod global;

use crate::compiler::CompiledBytecode;


pub fn test_reading(bytecode: &CompiledBytecode) {
    // create a new closure
    let mut frame = frames::ExecutionFrame::new_from_bytecode(
        bytecode, "main".to_string(), 0, 0 
    );

    while frame.has_instructions() {
        let (_, _, next) = frame.read_current_instruction();
        println!("{}", frame.disassemble_ip());
        frame.farword_ip(next);
    }
}