use std::rc::Rc;

pub mod errors;
pub mod isa;
pub mod loader;
pub mod opcode;
pub mod symtab;

use crate::parser::ast;
use crate::parser::exp;
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

    fn compile_literal(&mut self, literal: &ast::LiteralKind) -> Option<errors::CompileError> {
        match literal {
            ast::LiteralKind::Str(st) => {
                let idx = self.register_constant(Object::Str(st.to_string()));

                self.save(isa::InstructionKind::IConstant, &vec![idx]);
            }
            ast::LiteralKind::Float(f) => {
                let idx = self.register_constant(Object::Float(f.clone()));

                self.save(isa::InstructionKind::IConstant, &vec![idx]);
            }
            ast::LiteralKind::Int(i) => {
                let idx = self.register_constant(Object::Int(i.clone()));
                self.save(isa::InstructionKind::IConstant, &vec![idx]);
            }
            ast::LiteralKind::Char(c) => {
                let idx = self.register_constant(Object::Char(c.clone()));
                self.save(isa::InstructionKind::IConstant, &vec![idx]);
            }
            ast::LiteralKind::Bool(b) => {
                let idx = self.register_constant(Object::Bool(b.clone()));
                self.save(isa::InstructionKind::IConstant, &vec![idx]);
            }
            _ => return None,
        }

        return None;
    }

    fn compile_identifier(&mut self, idt: &ast::IdentifierType) -> Option<errors::CompileError> {
        let id_name = &idt.name;
        // resolve it
        let resolve_result = self.symbol_table.resolve_symbol(id_name);
        if resolve_result.is_none() {
            return Some(errors::CompileError::new(
                format!("Unresolved symbol {}", id_name),
                errors::CompilerErrorKind::UnresolvedSymbol,
                0,
            ));
        }

        let resolved_symbol = resolve_result.unwrap();
        match resolved_symbol.scope {
            symtab::ScopeKind::Builtin => {
                return None; // as of now
            }
            symtab::ScopeKind::Free => {
                return None;
            }
            symtab::ScopeKind::Global => {
                self.save(
                    isa::InstructionKind::ILoadGlobal,
                    &vec![resolved_symbol.pos],
                );
            }
            symtab::ScopeKind::Local => {
                self.save(isa::InstructionKind::ILoadLocal, &vec![resolved_symbol.pos]);
            }
        }

        return None;
    }

    fn compile_infix_expression(&mut self, expr: &ast::InfixType) -> Option<errors::CompileError> {
        // parse the expression:
        if expr.infix != exp::InfixExpKind::Equal {
            let mut res = self.compile_expression(&expr.expression_left);
            if res.is_some() {
                return res;
            }
            res = self.compile_expression(&expr.expression_right);
            if res.is_some() {
                return res;
            }
        } else {
            let res = self.compile_expression(&expr.expression_right);
            if res.is_some() {
                return res;
            }
            let left = &expr.expression_left;
            match left.as_ref() {
                ast::ExpressionKind::Identifier(id) => {
                    let resolve_result = self.symbol_table.resolve_symbol(&id.name);
                    if resolve_result.is_none() {
                        return Some(errors::CompileError::new(
                            format!("Unresolved symbol {}", id.name),
                            errors::CompilerErrorKind::InvalidAssignment,
                            0,
                        ));
                    }

                    let resolved_symbol = resolve_result.unwrap();
                    if resolved_symbol.is_const {
                        return Some(errors::CompileError::new(
                            format!("Cannot assign to symbol {}", id.name),
                            errors::CompilerErrorKind::InvalidAssignment,
                            0,
                        ));
                    }

                    // the symbol is resolved without any errors, store it back:
                    match resolved_symbol.scope {
                        symtab::ScopeKind::Global => {
                            self.save(
                                isa::InstructionKind::IStoreGlobal,
                                &vec![resolved_symbol.pos],
                            );
                        }
                        symtab::ScopeKind::Local => {
                            self.save(
                                isa::InstructionKind::IStoreLocal,
                                &vec![resolved_symbol.pos],
                            );
                        }
                        _ => {
                            return Some(errors::CompileError::new(
                                format!("Invalid assignment {}", id.name),
                                errors::CompilerErrorKind::InvalidAssignment,
                                0,
                            ))
                        }
                    }
                }
                _ => {
                    return Some(errors::CompileError::new(
                        "Invalid assignment".to_string(),
                        errors::CompilerErrorKind::InvalidAssignment,
                        0,
                    ))
                }
            }

            return None;
        }

        // check the operator in the middle:
        match expr.infix {
            exp::InfixExpKind::Plus => {
                self.save(isa::InstructionKind::IAdd, &vec![]);
            }
            exp::InfixExpKind::Minus => {
                self.save(isa::InstructionKind::ISub, &vec![]);
            }
            exp::InfixExpKind::Mul => {
                self.save(isa::InstructionKind::IMul, &vec![]);
            }
            exp::InfixExpKind::Div => {
                self.save(isa::InstructionKind::IDiv, &vec![]);
            }
            exp::InfixExpKind::Mod => {
                self.save(isa::InstructionKind::IMod, &vec![]);
            }
            exp::InfixExpKind::And => {
                self.save(isa::InstructionKind::IAnd, &vec![]);
            }
            exp::InfixExpKind::Or => {
                self.save(isa::InstructionKind::IOr, &vec![]);
            }
            _ => {}
        }

        return None;
    }

    #[allow(unused_variables)]
    fn compile_expression(
        &mut self,
        expression: &ast::ExpressionKind,
    ) -> Option<errors::CompileError> {
        match expression {
            ast::ExpressionKind::Literal(lt) => {
                let result = self.compile_literal(&lt);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Identifier(id) => {
                let result = self.compile_identifier(&id);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Infix(expr) => {
                let result = self.compile_infix_expression(&expr);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            _ => return None,
        }
        return None;
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

pub struct BytecodeDecompiler {}

impl BytecodeDecompiler {
    pub fn disassemble(bytecode: &CompiledBytecode) -> String {
        let instructions = &bytecode.instructions;
        let length = instructions.len();

        let mut decoded_string = String::new();
        let mut idx = 0;

        while idx < length {
            let op = instructions[idx];
            let op_kind: isa::InstructionKind = unsafe { ::std::mem::transmute(op) };
            let (operands, next_offset) =
                opcode::InstructionPacker::decode_instruction(&op_kind, &instructions[idx + 1..]);

            decoded_string.push_str(&op_kind.disasm_instruction(&operands));
            decoded_string.push('\n');

            idx = idx + next_offset + 1;
        }

        return decoded_string;
    }
}
