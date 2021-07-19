pub mod isa;
pub mod symtab;
pub mod loader;
pub mod opcode;
pub mod errors;

use symtab::ConstantPool;

pub type CompiledInstructions = Vec<u8>;

pub struct CompiledBytecode {
    pub constants_pool: ConstantPool,
    pub instructions: CompiledInstructions
}