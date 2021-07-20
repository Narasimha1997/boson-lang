use std::rc::Rc;

pub mod errors;
pub mod isa;
pub mod loader;
pub mod opcode;
pub mod symtab;

use crate::parser::ast;
use crate::types::object::Object;

use symtab::ConstantPool;

pub type CompiledInstructions = Vec<u8>;

#[derive(Debug, Clone)]
pub struct CompiledBytecode {
    pub constant_pool: ConstantPool,
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
    size: usize,
}

impl ProgramScope {
    fn new_scope() -> ProgramScope {
        return ProgramScope {
            instructions: vec![],
            last: None,
            previous: None,
            size: 0,
        };
    }

    fn push_compiled_instructions(&mut self, instructions: &CompiledInstructions) {
        self.instructions.extend_from_slice(instructions);
        self.size += instructions.len();
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn set_last(&mut self, inst: isa::InstructionKind, pos: usize) {
        if self.last.is_some() {
            let last_instruction = self.last.as_ref().unwrap();
            self.previous = Some(last_instruction.clone());
        }

        let new_opcode = OpCode {
            instruction: inst,
            pos: pos,
        };

        self.last = Some(new_opcode);
    }

    #[allow(dead_code)]
    fn get_last(&self) -> &Option<OpCode> {
        &self.last
    }

    #[allow(dead_code)]
    fn get_previous(&self) -> &Option<OpCode> {
        &self.previous
    }

    #[allow(dead_code)]
    fn last_instruction_is(&self, inst: isa::InstructionKind) -> bool {
        self.last.is_some() && self.last.as_ref().unwrap().is_equal_to(inst)
    }

    #[allow(dead_code)]
    fn previous_instruction_is(&self, inst: isa::InstructionKind) -> bool {
        self.previous.is_some() && self.previous.as_ref().unwrap().is_equal_to(inst)
    }

    fn get_instructions(&self) -> &CompiledInstructions {
        &self.instructions
    }
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
        let symbol_table = symtab::SymbolTable::create_new_root();

        let root_scope = ProgramScope::new_scope();

        return BytecodeCompiler {
            constant_pool: ConstantPool::new(),
            symbol_table: symbol_table,
            scopes: vec![root_scope],
            scope_index: 0,
        };
    }

    pub fn new_from_previous(
        symbol_table: symtab::SymbolTable,
        constants: ConstantPool,
    ) -> BytecodeCompiler {
        let root_scope = ProgramScope::new_scope();

        return BytecodeCompiler {
            constant_pool: constants,
            symbol_table: symbol_table,
            scopes: vec![root_scope],
            scope_index: 0,
        };
    }

    #[allow(dead_code)]
    fn register_constant(&mut self, obj: Object) -> usize {
        self.constant_pool.set_object(Rc::new(obj))
    }

    fn save(&mut self, inst: isa::InstructionKind, operands: &opcode::Operands) -> usize {
        let coded_stmt = opcode::InstructionPacker::encode_instruction(inst.clone(), operands);
        let current_pos = self.scopes[self.scope_index].get_size();
        self.scopes[self.scope_index].push_compiled_instructions(&coded_stmt);
        self.scopes[self.scope_index].set_last(inst, current_pos);

        return current_pos;
    }

    #[allow(dead_code)]
    fn enter_scope(&mut self) {
        let new_scope = ProgramScope::new_scope();
        self.scopes.push(new_scope);
        self.symbol_table = symtab::SymbolTable::create_new_child(self.symbol_table.clone());
        self.scope_index += 1;
    }

    #[allow(dead_code)]
    fn exit_scope(&mut self) -> Result<CompiledInstructions, errors::CompileError> {
        if self.symbol_table.parent.is_none() {
            return Err(errors::CompileError::new(
                "Compiler Error, invalid top-level scope".to_string(),
                errors::CompilerErrorKind::InvalidScope,
                0,
            ));
        }

        self.symbol_table = self.symbol_table.parent.as_ref().unwrap().as_ref().clone();

        let instructions = self.scopes[self.scope_index].get_instructions().clone();
        self.scope_index -= 1;
        self.scopes.pop();

        return Ok(instructions);
    }

    #[allow(unused_variables)]
    fn compile_expression(
        &mut self,
        expression: &ast::ExpressionKind,
    ) -> Option<errors::CompileError> {
        None
    }

    fn compile_const_declr(&mut self, stmt: &ast::ConstType) -> Option<errors::CompileError> {
        let var_name = &stmt.identifier.name;
        // resolve the name:
        let resolve_result = self.symbol_table.resolve_symbol(&var_name);
        if resolve_result.is_some() {
            return Some(errors::CompileError::new(
                format!("{} already declared", var_name),
                errors::CompilerErrorKind::SymbolAlreadyExist,
                0,
            ));
        }

        let registered_symbol = self.symbol_table.insert_new_symbol(&var_name, true);

        if stmt.expression.is_some() {
            self.compile_expression(stmt.expression.as_ref().unwrap());
        }

        match registered_symbol.scope {
            symtab::ScopeKind::Global => {
                self.save(
                    isa::InstructionKind::IStoreGlobal,
                    &vec![registered_symbol.pos],
                );
            }
            symtab::ScopeKind::Local => {
                self.save(
                    isa::InstructionKind::IStoreLocal,
                    &vec![registered_symbol.pos],
                );
            }
            symtab::ScopeKind::Builtin => {
                return Some(errors::CompileError::new(
                    format!("Cannot assign {} as built-in.", var_name),
                    errors::CompilerErrorKind::BuiltinAssignment,
                    0,
                ));
            }
            _ => {}
        }

        return None;
    }

    fn compile_variable_declr(&mut self, stmt: &ast::LetType) -> Option<errors::CompileError> {
        let var_name = &stmt.identifier.name;
        // resolve the name:
        let resolve_result = self.symbol_table.resolve_symbol(&var_name);
        if resolve_result.is_some() {
            return Some(errors::CompileError::new(
                format!("{} already declared", var_name),
                errors::CompilerErrorKind::SymbolAlreadyExist,
                0,
            ));
        }

        let registered_symbol = self.symbol_table.insert_new_symbol(&var_name, false);

        if stmt.expression.is_some() {
            self.compile_expression(stmt.expression.as_ref().unwrap());
        }

        match registered_symbol.scope {
            symtab::ScopeKind::Global => {
                self.save(
                    isa::InstructionKind::IStoreGlobal,
                    &vec![registered_symbol.pos],
                );
            }
            symtab::ScopeKind::Local => {
                self.save(
                    isa::InstructionKind::IStoreLocal,
                    &vec![registered_symbol.pos],
                );
            }
            symtab::ScopeKind::Builtin => {
                return Some(errors::CompileError::new(
                    format!("Cannot assign {} as built-in.", var_name),
                    errors::CompilerErrorKind::BuiltinAssignment,
                    0,
                ));
            }
            _ => {}
        }

        return None;
    }

    fn get_bytecode(&self) -> CompiledBytecode {
        return CompiledBytecode {
            constant_pool: self.constant_pool.clone(),
            instructions: self.scopes[self.scope_index].get_instructions().clone(),
        };
    }

    pub fn compile(
        &mut self,
        program_ast: &ast::Program,
    ) -> Result<CompiledBytecode, errors::CompileError> {
        let statements = &program_ast.statements;
        for stmt in statements {
            let error = match stmt {
                ast::StatementKind::Expression(node) => self.compile_expression(&node),
                ast::StatementKind::Var(node) => self.compile_variable_declr(&node),
                ast::StatementKind::Const(node) => self.compile_const_declr(&node),
                _ => {
                    return Err(errors::CompileError::new(
                        "Not yet implemented".to_string(),
                        errors::CompilerErrorKind::UnresolvedSymbol,
                        0,
                    ))
                }
            };

            if error.is_some() {
                let unwrapped_error = error.unwrap();
                return Err(unwrapped_error);
            }
        }

        return Ok(self.get_bytecode());
    }
}
