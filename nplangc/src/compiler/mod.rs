pub mod errors;
pub mod isa;
pub mod loader;
pub mod opcode;
pub mod symtab;

use symtab::ConstantPool;

pub type CompiledInstructions = Vec<u8>;

pub struct CompiledBytecode {
    pub constants_pool: ConstantPool,
    pub instructions: CompiledInstructions,
}

#[derive(Debug, Clone)]
pub struct OpCode {
    instruction: isa::InstructionKind,
    pos: usize,
}

impl OpCode {
    pub fn is_equal_to(&self, instruction: isa::InstructionKind) -> bool {
        instruction == self.instruction
    }
}

struct ProgramScope {
    instructions: CompiledInstructions,
    last: Option<OpCode>,
    previous: Option<OpCode>,
}

type ProgramScopes = Vec<ProgramScope>;

pub struct BytecodeCompiler {
    pub constant_pool: ConstantPool,
    pub symbol_table: symtab::SymbolTable,
    scopes: ProgramScopes,
    scope_index: usize,
}

impl BytecodeCompiler {
    pub fn new() -> BytecodeCompiler {
        let mut symbol_table = symtab::SymbolTable::create_new_root();

        let mut root_scope = ProgramScope {
            instructions: vec![],
            last: None,
            previous: None,
        };

        return BytecodeCompiler {
            constant_pool: vec![],
            symbol_table: symbol_table,
            scopes: vec![root_scope],
            scope_index: 0,
        };
    }

    pub fn new_from_previous(
        symbol_table: symtab::SymbolTable,
        constants: ConstantPool,
    ) -> BytecodeCompiler {
        let mut root_scope = ProgramScope {
            instructions: vec![],
            last: None,
            previous: None,
        };

        return BytecodeCompiler {
            constant_pool: constants,
            symbol_table: symbol_table,
            scopes: vec![root_scope],
            scope_index: 0,
        };
    }
}
