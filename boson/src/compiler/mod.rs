use std::rc::Rc;

pub mod errors;
pub mod loader;
pub mod symtab;

use crate::isa;
use crate::parser::ast;
use crate::parser::exp;
use crate::types::object::Object;
use crate::types::subroutine::Subroutine;

use isa::InstructionPacker;
use isa::Operands;

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
    loop_ctls: Vec<LoopControl>,
    n_lambdas: usize,
}

struct LoopControl {
    loop_start_pos: usize,
    pos_after_loop: usize,
    break_pos: Vec<usize>,
}

impl BytecodeCompiler {
    pub fn new() -> BytecodeCompiler {
        let mut symbol_table = symtab::SymbolTable::create_new_root();
        symbol_table.insert_builtins();

        let root_scope = ProgramScope::new_scope();

        return BytecodeCompiler {
            constant_pool: ConstantPool::new(),
            symbol_table: symbol_table,
            scopes: vec![root_scope],
            scope_index: 0,
            loop_ctls: vec![],
            n_lambdas: 0,
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
            loop_ctls: vec![],
            n_lambdas: 0,
        };
    }

    #[allow(dead_code)]
    fn register_constant(&mut self, obj: Object) -> usize {
        self.constant_pool.set_object(Rc::new(obj))
    }

    fn save(&mut self, inst: isa::InstructionKind, operands: &Operands) -> usize {
        let coded_stmt = InstructionPacker::encode_instruction(inst.clone(), operands);
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
            ast::LiteralKind::Array(arr) => {
                let error = self.compile_array(&arr);
                if error.is_some() {
                    return error;
                }
            }
            ast::LiteralKind::HashTable(ht) => {
                let error = self.compile_hash(&ht);
                if error.is_some() {
                    return error;
                }
            }
        }

        return None;
    }

    fn compile_index(&mut self, node: &ast::IndexType) -> Option<errors::CompileError> {
        // compile left expression:
        let mut error = self.compile_expression(&node.expression_left);
        if error.is_some() {
            return error;
        }

        error = self.compile_expression(&node.index);
        if error.is_some() {
            return error;
        }

        self.save(isa::InstructionKind::IGetIndex, &vec![]);
        return None;
    }

    fn compile_assert_statement(&mut self, node: &ast::AssertType) -> Option<errors::CompileError> {
        let assert_expr = &node.target_expr;
        let mut error = self.compile_expression(&assert_expr);
        if error.is_some() {
            return error;
        }

        self.save(isa::InstructionKind::ILNot, &vec![]);
        let not_jmp_pos = self.save(isa::InstructionKind::INotJump, &vec![0]);

        // load panic expression:
        error = self.compile_expression(&node.fail_expr);
        if error.is_some() {
            return error;
        }

        self.save(isa::InstructionKind::IAssertFail, &vec![]);

        let no_op_pos = self.save(isa::InstructionKind::INoOp, &vec![]);

        // replace expression:
        error = self.replace_instruction_operands(
            self.scope_index,
            isa::InstructionKind::INotJump,
            &vec![no_op_pos],
            &not_jmp_pos,
        );

        return error;
    }

    fn compile_if_statement(&mut self, node: &ast::IfElseType) -> Option<errors::CompileError> {
        let if_expr = &node.condition;
        // compile the if expr:
        let mut error = self.compile_expression(&if_expr);
        if error.is_some() {
            return error;
        }

        // add a not jump, this will be replaced to either else or NoOp
        let jump_instr_pos = self.save(isa::InstructionKind::INotJump, &vec![0]);

        // compile the if block:
        error = self.compile_block_statement(&node.main_block);
        if error.is_some() {
            return error;
        }

        // check if there is an else block:
        if node.alternate_block.is_none() {
            // no else block, add a NoOp and replace INotJump Pos:
            let no_op_pos = self.save(isa::InstructionKind::INoOp, &vec![]);

            error = self.replace_instruction_operands(
                self.scope_index,
                isa::InstructionKind::INotJump,
                &vec![no_op_pos],
                &jump_instr_pos,
            );

            if error.is_some() {
                return error;
            }

            return None;
        }

        // it has an else statement:
        // add a JUMP in if statement:
        let after_if_pos = self.save(isa::InstructionKind::IJump, &vec![0]);

        let else_pos = self.save(isa::InstructionKind::INoOp, &vec![]);
        error = self.replace_instruction_operands(
            self.scope_index,
            isa::InstructionKind::INotJump,
            &vec![else_pos],
            &jump_instr_pos,
        );

        if error.is_some() {
            return error;
        }

        // compile the else block
        error = self.compile_block_statement(&node.alternate_block.as_ref().unwrap());
        if error.is_some() {
            return error;
        }

        let after_else = self.save(isa::InstructionKind::INoOp, &vec![]);
        error = self.replace_instruction_operands(
            self.scope_index,
            isa::InstructionKind::IJump,
            &vec![after_else],
            &after_if_pos,
        );

        if error.is_some() {
            return error;
        }

        return None;
    }

    fn compile_return_stmt(&mut self, node: &ast::ReturnType) -> Option<errors::CompileError> {
        if node.expression.is_some() {
            // compile the return expression
            let exp = &node.expression.as_ref().unwrap();
            let error = self.compile_expression(exp);

            if error.is_some() {
                return error;
            }

            self.save(isa::InstructionKind::IRetVal, &vec![]);
            return None;
        } else {
            self.save(isa::InstructionKind::IRet, &vec![]);
            return None;
        }
    }

    fn compile_function(
        &mut self,
        node: &ast::FunctionType,
        is_lambda: bool,
    ) -> Option<errors::CompileError> {
        // check function name:
        let resolve_result = self.symbol_table.resolve_symbol(&node.name);
        if resolve_result.is_some() {
            return Some(errors::CompileError::new(
                format!("Name {} already defined", &node.name),
                errors::CompilerErrorKind::SymbolAlreadyExist,
                0,
            ));
        }

        // enter the scope:
        self.enter_scope();

        let args = &node.parameters;
        let error: Option<errors::CompileError>;

        for arg in args {
            match arg {
                ast::ExpressionKind::Identifier(id) => {
                    self.symbol_table.insert_new_symbol(&id.name, false);
                    //self.save(isa::InstructionKind::ILoadLocal, &vec![sym.pos]);
                }
                _ => {
                    return Some(errors::CompileError::new(
                        "Function parameter is a non identifier".to_string(),
                        errors::CompilerErrorKind::InvalidOperand,
                        0,
                    ))
                }
            }
        }

        let func_block = &node.body;
        error = self.compile_block_statement(func_block);
        if error.is_some() {
            return error;
        }

        // check if there is a return statement at last:
        let n_statements = func_block.statements.len();
        let last_stmt = &func_block.statements[n_statements - 1];

        if !is_lambda {
            match last_stmt {
                ast::StatementKind::Return(_) => {}
                _ => {
                    // append a return void statement
                    self.save(isa::InstructionKind::IRet, &vec![]);
                }
            }
        } else {
            self.save(isa::InstructionKind::IRetVal, &vec![]);
        }

        let free_symbols = self.symbol_table.get_free_symbols();
        let n_locals = self.symbol_table.n_items;

        let compiled_result = self.exit_scope();
        if compiled_result.is_err() {
            return Some(compiled_result.unwrap_err());
        }

        let compiled_func = compiled_result.unwrap();

        for sym in &free_symbols {
            self.save(isa::InstructionKind::ILoadFree, &vec![sym.pos]);
        }

        let compiled_func_type = Subroutine {
            name: node.name.clone(),
            bytecode: compiled_func,
            num_locals: n_locals,
            num_parameters: args.len(),
        };

        let func_object = Object::Subroutine(Rc::new(compiled_func_type));

        // register the sub-routine:
        let func_idx = self.register_constant(func_object);

        // create a closure instruction:
        self.save(
            isa::InstructionKind::IClosure,
            &vec![func_idx, free_symbols.len()],
        );

        // store
        if !is_lambda {
            let sym_res = self.symbol_table.insert_new_symbol(&node.name, true);
            match sym_res.scope {
                symtab::ScopeKind::Global => {
                    self.save(isa::InstructionKind::IStoreGlobal, &vec![sym_res.pos]);
                }
                symtab::ScopeKind::Local => {
                    self.save(isa::InstructionKind::IStoreLocal, &vec![sym_res.pos]);
                }
                _ => {}
            }
        }

        return None;
    }

    fn compile_lambda(&mut self, node: &ast::LambdaExpType) -> Option<errors::CompileError> {
        //convert lambda to function, find better method soon
        let func_type = ast::FunctionType {
            name: format!("lambda_{}", self.n_lambdas),
            parameters: node.parameters.clone(),
            body: ast::BlockStatement {
                statements: vec![ast::StatementKind::Expression(
                    node.expression.as_ref().clone(),
                )],
            },
            return_type: None,
        };

        // compile the function:
        let error = self.compile_function(&func_type, true);
        if error.is_some() {
            return error;
        }

        self.n_lambdas += 1;
        return None;
    }

    fn compile_for_loop(&mut self, node: &ast::ForLoopType) -> Option<errors::CompileError> {
        let iter_exp = &node.iter;

        // compile:
        let mut error = self.compile_expression(&iter_exp);
        if error.is_some() {
            return error;
        }

        let current_pos = self.scopes[self.scope_index].get_size();
        let new_loop_ctl = LoopControl {
            loop_start_pos: current_pos.clone(),
            pos_after_loop: 0,
            break_pos: vec![],
        };

        self.loop_ctls.push(new_loop_ctl);
        let current_loop_ctl = self.loop_ctls.len() - 1;

        // register the target variable
        let target = &node.target;
        let registered_sym: Rc<symtab::Symbol>;

        match target.as_ref() {
            ast::ExpressionKind::Identifier(id) => {
                // check if it's constant:
                let resolved_sym = self.symbol_table.resolve_symbol(&id.name);
                if resolved_sym.is_some() && resolved_sym.unwrap().is_const {
                    return Some(errors::CompileError::new(
                        format!("Cannot assign to constant {}", id.name),
                        errors::CompilerErrorKind::ConstantAssignment,
                        0,
                    ));
                }

                registered_sym = self.symbol_table.insert_new_symbol(&id.name, false)
            }
            _ => {
                return Some(errors::CompileError::new(
                    "Invalid expression, loop target must be an identifier".to_string(),
                    errors::CompilerErrorKind::InvalidOperand,
                    0,
                ))
            }
        }

        // Perform iteration, replace the iterator end later
        let loop_start = self.save(isa::InstructionKind::IIter, &vec![0]);
        self.loop_ctls[current_loop_ctl].loop_start_pos = loop_start;

        // load the iter variable:
        match registered_sym.scope {
            symtab::ScopeKind::Global => {
                self.save(
                    isa::InstructionKind::IStoreGlobal,
                    &vec![registered_sym.pos],
                );
            }
            symtab::ScopeKind::Local => {
                self.save(isa::InstructionKind::IStoreLocal, &vec![registered_sym.pos]);
            }
            _ => {}
        }

        // compile the block statement:
        error = self.compile_block_statement(&node.loop_block);
        if error.is_some() {
            return error;
        }

        // put a jump:
        self.save(isa::InstructionKind::IJump, &vec![loop_start]);
        // end the loop with a no-op:
        let loop_end_pos = self.save(isa::InstructionKind::INoOp, &vec![]);

        // replace the IIter with loop end pos:
        error = self.replace_instruction_operands(
            self.scope_index,
            isa::InstructionKind::IIter,
            &vec![loop_end_pos],
            &loop_start,
        );

        if error.is_some() {
            return error;
        }

        // replace all breaks:
        for idx in 0..self.loop_ctls[current_loop_ctl].break_pos.len() {
            let brk_pos = self.loop_ctls[current_loop_ctl].break_pos[idx];
            error = self.replace_instruction_operands(
                self.scope_index,
                isa::InstructionKind::IJump,
                &vec![loop_end_pos],
                &brk_pos,
            );
            if error.is_some() {
                return error;
            }
        }

        // pop the loop control:
        self.loop_ctls.pop();
        return None;
    }

    fn compile_identifier(
        &mut self,
        idt: &ast::IdentifierType,
        check_const: bool,
    ) -> Option<errors::CompileError> {
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

        if check_const && resolved_symbol.is_const {
            return Some(errors::CompileError::new(
                format!("Cannot assign to constant symbol {}", resolved_symbol.name),
                errors::CompilerErrorKind::InvalidAssignment,
                0,
            ));
        }

        match resolved_symbol.scope {
            symtab::ScopeKind::Builtin => {
                self.save(
                    isa::InstructionKind::ILoadBuiltIn,
                    &vec![resolved_symbol.pos],
                );
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

    fn compile_incr_decr(
        &mut self,
        expr: &ast::PrefixType,
        is_pre: bool,
        is_decr: bool,
    ) -> Option<errors::CompileError> {
        match expr.expression.as_ref() {
            ast::ExpressionKind::Identifier(id) => {
                // resolve the identifier:
                let resolve_res = self.symbol_table.resolve_symbol(&id.name);
                if resolve_res.is_none() {
                    return Some(errors::CompileError::new(
                        format!("Unresolved symbol {}", id.name),
                        errors::CompilerErrorKind::InvalidAssignment,
                        0,
                    ));
                }

                let resolved_symbol = resolve_res.unwrap();
                if resolved_symbol.is_const {
                    return Some(errors::CompileError::new(
                        format!("Cannot assign to constant symbol {}", id.name),
                        errors::CompilerErrorKind::InvalidAssignment,
                        0,
                    ));
                }

                // compile the identifier, with constant assignment
                // check
                let res = self.compile_identifier(&id, true);
                if res.is_some() {
                    return res;
                }
            }
            _ => {
                return Some(errors::CompileError::new(
                    format!("Invalid operand, {:?}", expr.expression),
                    errors::CompilerErrorKind::InvalidOperand,
                    0,
                ))
            }
        }

        if is_pre && is_decr {
            self.save(isa::InstructionKind::IPreDecr, &vec![]);
        } else if is_pre && !is_decr {
            self.save(isa::InstructionKind::IPreIncr, &vec![]);
        } else if !is_pre && is_decr {
            self.save(isa::InstructionKind::IPostDecr, &vec![]);
        } else {
            self.save(isa::InstructionKind::IPostIncr, &vec![]);
        }

        return None;
    }

    fn compile_suffix_expression(
        &mut self,
        expr: &ast::SuffixType,
    ) -> Option<errors::CompileError> {
        match expr.suffix {
            exp::SuffixExpKind::PostDecrement => {
                // transform suffix to prefix:
                let transformed_prefix = ast::PrefixType {
                    expression: expr.expression.clone(),
                    prefix: exp::PrefixExpKind::PreDecrement,
                };

                let res = self.compile_incr_decr(&transformed_prefix, false, true);

                if res.is_some() {
                    return res;
                }
            }
            exp::SuffixExpKind::PostIncrement => {
                // transform suffix to prefix:
                let transformed_prefix = ast::PrefixType {
                    expression: expr.expression.clone(),
                    prefix: exp::PrefixExpKind::PreIncrement,
                };

                let res = self.compile_incr_decr(&transformed_prefix, false, false);

                if res.is_some() {
                    return res;
                }
            }
        }

        return None;
    }

    fn compile_prefix_expression(
        &mut self,
        expr: &ast::PrefixType,
    ) -> Option<errors::CompileError> {
        match expr.prefix {
            exp::PrefixExpKind::Neg => {
                let res = self.compile_expression(&expr.expression);
                if res.is_some() {
                    return res;
                }
                self.save(isa::InstructionKind::INeg, &vec![]);
            }
            exp::PrefixExpKind::PreIncrement => {
                let res = self.compile_incr_decr(&expr, true, false);
                if res.is_some() {
                    return res;
                }
            }
            exp::PrefixExpKind::PreDecrement => {
                let res = self.compile_incr_decr(&expr, true, true);
                if res.is_some() {
                    return res;
                }
            }
            exp::PrefixExpKind::Not => {
                let res = self.compile_expression(&expr.expression);
                if res.is_some() {
                    return res;
                }
                self.save(isa::InstructionKind::ILNot, &vec![]);
            }
        }

        return None;
    }

    fn compile_item_assignment(
        &mut self,
        id: &ast::IdentifierType,
    ) -> Option<errors::CompileError> {
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
                format!("Cannot assign to constant symbol {}", id.name),
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
                    let error = self.compile_item_assignment(&id);
                    if error.is_some() {
                        return error;
                    }
                }
                ast::ExpressionKind::Index(idx_type) => {
                    // get the index variable name:
                    let expr_left = &idx_type.expression_left;
                    // compile the left expression:
                    match expr_left.as_ref() {
                        ast::ExpressionKind::Identifier(id) => {
                            // compile the identifier:
                            let mut error = self.compile_identifier(&id, false);
                            if error.is_some() {
                                return error;
                            }

                            // compile the index expression:
                            error = self.compile_expression(&idx_type.index);
                            if error.is_some() {
                                return error;
                            }

                            // since both are loaded on to the stack, call ISetIndex:
                            self.save(isa::InstructionKind::ISetIndex, &vec![]);

                            error = self.compile_item_assignment(&id);
                            if error.is_some() {
                                return error;
                            }
                        }
                        _ => {
                            return Some(errors::CompileError::new(
                                "Assignment not possible".to_string(),
                                errors::CompilerErrorKind::InvalidOperand,
                                0,
                            ));
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
            exp::InfixExpKind::LesserThan => {
                self.save(isa::InstructionKind::ILLt, &vec![]);
            }
            exp::InfixExpKind::LesserThanEqual => {
                self.save(isa::InstructionKind::ILLTe, &vec![]);
            }
            exp::InfixExpKind::GreaterThan => {
                self.save(isa::InstructionKind::ILGt, &vec![]);
            }
            exp::InfixExpKind::GreaterThanEqual => {
                self.save(isa::InstructionKind::ILGte, &vec![]);
            }
            exp::InfixExpKind::EEqual => {
                self.save(isa::InstructionKind::ILEq, &vec![]);
            }
            exp::InfixExpKind::NotEqual => {
                self.save(isa::InstructionKind::ILNe, &vec![]);
            }
            exp::InfixExpKind::LogicalOr => {
                self.save(isa::InstructionKind::ILOr, &vec![]);
            }
            exp::InfixExpKind::LogicalAnd => {
                self.save(isa::InstructionKind::ILAnd, &vec![]);
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
            ast::ExpressionKind::Noval => {
                let reg_constant = self.register_constant(Object::Noval);
                self.save(isa::InstructionKind::IConstant, &vec![reg_constant]);
            }
            ast::ExpressionKind::Literal(lt) => {
                let result = self.compile_literal(&lt);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Identifier(id) => {
                let result = self.compile_identifier(&id, false);
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
            ast::ExpressionKind::Prefix(expr) => {
                let result = self.compile_prefix_expression(&expr);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Suffix(expr) => {
                let result = self.compile_suffix_expression(&expr);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Call(ct) => {
                let result = self.compile_call(&ct);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Lambda(lm) => {
                let result = self.compile_lambda(&lm);
                if result.is_some() {
                    return Some(result.unwrap());
                }
            }
            ast::ExpressionKind::Index(idx) => {
                let result = self.compile_index(&idx);
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

    fn compile_array(&mut self, arr: &ast::ArrayType) -> Option<errors::CompileError> {
        let elements = &arr.array_values;

        let mut error: Option<errors::CompileError>;
        for idx in 0..elements.len() {
            let expr = elements[idx].clone();
            error = self.compile_expression(&expr);
            if error.is_some() {
                return error;
            }
        }

        // save the array:
        self.save(isa::InstructionKind::IArray, &vec![arr.length]);
        return None;
    }

    fn compile_hash(&mut self, ht: &ast::HashTableType) -> Option<errors::CompileError> {
        let pairs = &ht.pairs;

        let mut error: Option<errors::CompileError>;
        for idx in 0..pairs.len() {
            let pair = pairs[idx].clone();
            let (key_expr, value_expr) = pair;

            error = self.compile_expression(&key_expr);
            if error.is_some() {
                return error;
            }

            error = self.compile_expression(&value_expr);
            if error.is_some() {
                return error;
            }
        }

        // set hash:
        let scope_length = pairs.len() * 2;
        self.save(isa::InstructionKind::IHash, &vec![scope_length]);
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

    fn replace_instruction_operands(
        &mut self,
        scope_idx: usize,
        inst: isa::InstructionKind,
        operands: &Operands,
        pos: &usize,
    ) -> Option<errors::CompileError> {
        if self.scopes.len() <= scope_idx {
            return Some(errors::CompileError::new(
                "Reached out of scope while replacing opcode".to_string(),
                errors::CompilerErrorKind::BytecodeError,
                0,
            ));
        }

        let opcodes_length = self.scopes[scope_idx].get_size();
        if opcodes_length <= *pos {
            return Some(errors::CompileError::new(
                "Reached out of buffer while replacing opcode".to_string(),
                errors::CompilerErrorKind::BytecodeError,
                0,
            ));
        }

        // substitute:
        let compiled_opcode = InstructionPacker::encode_instruction(inst, &operands);

        for idx in 0..compiled_opcode.len() {
            self.scopes[scope_idx].instructions[*pos + idx] = compiled_opcode[idx];
        }

        return None;
    }

    fn compile_call(&mut self, node: &ast::CallType) -> Option<errors::CompileError> {
        let args = &node.arguments;

        // compile all arguments:
        for idx in 0..args.len() {
            let expr = &args[idx];

            // compile the expression:
            let error = self.compile_expression(expr);
            if error.is_some() {
                return error;
            }
        }

        // resolve function name:
        let fn_expr = &node.function;
        let error = self.compile_expression(fn_expr);
        if error.is_some() {
            return error;
        }

        // place the call instruction:
        self.save(isa::InstructionKind::ICall, &vec![args.len()]);

        return None;
    }

    fn compile_break_stmt(&mut self) -> Option<errors::CompileError> {
        let n_loop_ctls = self.loop_ctls.len();
        if n_loop_ctls == 0 {
            return Some(errors::CompileError::new(
                "break encountered outside loop".to_string(),
                errors::CompilerErrorKind::InvalidBreak,
                0,
            ));
        }

        self.save(isa::InstructionKind::IBlockEnd, &vec![]);
        let break_pos = self.save(isa::InstructionKind::IJump, &vec![0]);

        self.loop_ctls[n_loop_ctls - 1].break_pos.push(break_pos);
        return None;
    }

    fn compile_continue_stmt(&mut self) -> Option<errors::CompileError> {
        let n_loop_ctls = self.loop_ctls.len();
        if n_loop_ctls == 0 {
            return Some(errors::CompileError::new(
                "continue encountered outside loop".to_string(),
                errors::CompilerErrorKind::InvalidContinue,
                0,
            ));
        }

        let jump_pos = self.loop_ctls[n_loop_ctls - 1].loop_start_pos;

        self.save(isa::InstructionKind::IBlockEnd, &vec![]);
        self.save(isa::InstructionKind::IJump, &vec![jump_pos]);

        return None;
    }

    fn compile_while_loop(&mut self, node: &ast::WhileLoopType) -> Option<errors::CompileError> {
        let while_expr = &node.target_expr;
        let current_pos = self.scopes[self.scope_index].get_size();
        let new_loop_ctl = LoopControl {
            loop_start_pos: current_pos.clone(),
            pos_after_loop: 0,
            break_pos: vec![],
        };

        self.loop_ctls.push(new_loop_ctl);
        let current_loop_ctl = self.loop_ctls.len() - 1;

        // compile the loop expression:
        let expr_error = self.compile_expression(&while_expr);
        if expr_error.is_some() {
            return expr_error;
        }

        // append a jump statement if loop fails:
        let jump_inst_pos = self.save(isa::InstructionKind::INotJump, &vec![0]);

        // compile loop block statement:
        let block_error = self.compile_block_statement(&node.loop_block);
        if block_error.is_some() {
            return block_error;
        }

        // append a jump back command
        self.save(isa::InstructionKind::IJump, &vec![current_pos]);
        self.loop_ctls[current_loop_ctl].pos_after_loop =
            self.save(isa::InstructionKind::INoOp, &vec![]);

        // replace loop instructions:
        let error = self.replace_instruction_operands(
            self.scope_index,
            isa::InstructionKind::INotJump,
            &vec![self.loop_ctls[current_loop_ctl].pos_after_loop],
            &jump_inst_pos,
        );

        if error.is_some() {
            return error;
        }

        // replace all break instructions:
        for idx in 0..self.loop_ctls[current_loop_ctl].break_pos.len() {
            let pos = self.loop_ctls[current_loop_ctl].break_pos[idx];
            let error = self.replace_instruction_operands(
                self.scope_index,
                isa::InstructionKind::IJump,
                &vec![self.loop_ctls[current_loop_ctl].pos_after_loop],
                &pos,
            );
            if error.is_some() {
                return error;
            }
        }

        self.loop_ctls.pop();

        return None;
    }

    fn compile_statement(&mut self, stmt: &ast::StatementKind) -> Option<errors::CompileError> {
        let error = match stmt {
            ast::StatementKind::Expression(node) => self.compile_expression(&node),
            ast::StatementKind::Var(node) => self.compile_variable_declr(&node),
            ast::StatementKind::Const(node) => self.compile_const_declr(&node),
            ast::StatementKind::While(node) => self.compile_while_loop(&node),
            ast::StatementKind::Break => self.compile_break_stmt(),
            ast::StatementKind::Continue => self.compile_continue_stmt(),
            ast::StatementKind::If(node) => self.compile_if_statement(&node),
            ast::StatementKind::Assert(node) => self.compile_assert_statement(&node),
            ast::StatementKind::For(node) => self.compile_for_loop(&node),
            ast::StatementKind::Function(node) => self.compile_function(&node, false),
            ast::StatementKind::Return(node) => self.compile_return_stmt(&node),
            _ => {
                return Some(errors::CompileError::new(
                    "Not yet implemented".to_string(),
                    errors::CompilerErrorKind::UnresolvedSymbol,
                    0,
                ))
            }
        };

        if error.is_some() {
            let unwrapped_error = error.unwrap();
            return Some(unwrapped_error);
        }

        return None;
    }

    fn compile_block_statement(
        &mut self,
        node: &ast::BlockStatement,
    ) -> Option<errors::CompileError> {
        self.save(isa::InstructionKind::IBlockStart, &vec![]);

        for stmt in &node.statements {
            let error = self.compile_statement(&stmt);
            if error.is_some() {
                return error;
            }
        }

        self.save(isa::InstructionKind::IBlockEnd, &vec![]);

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
            let error = self.compile_statement(&stmt);
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
    pub fn disassemble_function(instructions: &CompiledInstructions) -> String {
        let length = instructions.len();

        let mut decoded_string = String::new();
        let mut idx = 0;

        while idx < length {
            let op = instructions[idx];
            let op_kind: isa::InstructionKind = unsafe { ::std::mem::transmute(op) };
            let (operands, next_offset) =
                InstructionPacker::decode_instruction(&op_kind, &instructions[idx + 1..]);
            decoded_string.push_str(&format!("{:0>8x} ", idx));
            decoded_string.push_str(&op_kind.disasm_instruction(&operands));
            decoded_string.push('\n');

            idx = idx + next_offset + 1;
        }

        return decoded_string;
    }

    pub fn disassemble_instructions(bytecode: &CompiledBytecode) -> String {
        let instructions = &bytecode.instructions;
        let decoded_string = BytecodeDecompiler::disassemble_function(instructions);
        return decoded_string;
    }

    pub fn disassemble_constants(bytecode: &CompiledBytecode) -> String {
        // constant pool:
        let constant_pool = &bytecode.constant_pool;
        let mut decoded_string = String::new();
        let mut idx = 0;
        for item in &constant_pool.objects {
            match item.as_ref() {
                Object::Subroutine(sub) => {
                    decoded_string.push_str(&format!("{:0>8x} {}\n", idx, sub.describe()));
                    let repr =
                        BytecodeDecompiler::disassemble_function(sub.as_ref().get_bytecode());
                    decoded_string.push_str("Subroutine Start:\n");
                    decoded_string.push_str(&repr);
                    decoded_string.push_str("Subroutine End\n");
                    idx = idx + 1;
                }
                _ => {
                    let repr = item.describe();
                    decoded_string.push_str(&format!("{:0>8x} {}\n", idx, repr));
                    idx = idx + 1;
                }
            }
        }

        return decoded_string;
    }

    pub fn disassemble(bytecode: &CompiledBytecode) -> String {
        let mut decoded_string = String::new();

        decoded_string.push_str("Instructions: \n");

        decoded_string.push_str(&BytecodeDecompiler::disassemble_instructions(&bytecode));

        decoded_string.push_str("\nConstants: \n");

        decoded_string.push_str(&BytecodeDecompiler::disassemble_constants(&bytecode));

        return decoded_string;
    }
}
