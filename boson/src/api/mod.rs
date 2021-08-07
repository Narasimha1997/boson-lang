use crate::compiler::errors::CompileError;
use crate::compiler::BytecodeCompiler;
use crate::compiler::BytecodeDecompiler;
use crate::compiler::CompiledBytecode;
use crate::lexer::LexerAPI;
use crate::parser::debug::ParserError;
use crate::parser::Parser;
use crate::types::object::Object;
use crate::vm::errors::VMError;
use crate::vm::BosonVM;

use std::rc::Rc;

pub struct BosonLang {
    pub parser: Parser,
    pub compiler: BytecodeCompiler,
    pub vm: Option<BosonVM>
}

#[derive(Debug)]
pub enum ErrorKind {
    CompileError(CompileError),
    ParserError(Vec<ParserError>),
    VMError(VMError),
}

impl BosonLang {
    pub fn new_from_file(file: String) -> BosonLang {
        let lexer = LexerAPI::new_from_file(file);
        let parser = Parser::new_from_lexer(lexer);
        let compiler = BytecodeCompiler::new();

        return BosonLang {
            parser: parser,
            compiler: compiler,
            vm: None,
        };
    }

    pub fn new_from_buffer(buffer: Vec<u8>) -> BosonLang {
        let lexer = LexerAPI::new_from_buffer(buffer);
        let parser = Parser::new_from_lexer(lexer);
        let compiler = BytecodeCompiler::new();

        return BosonLang {
            parser: parser,
            compiler: compiler,
            vm: None
        };
    }

    fn __get_bytecode(&mut self) -> Result<CompiledBytecode, ErrorKind> {
        let parsed_res = self.parser.parse();
        if parsed_res.is_err() {
            return Err(ErrorKind::ParserError(parsed_res.unwrap_err().clone()));
        }

        let ast = parsed_res.unwrap();
        self.compiler.clear_previous();
        let compiler_result = self.compiler.compile(&ast);
        if compiler_result.is_err() {
            return Err(ErrorKind::CompileError(compiler_result.unwrap_err()));
        }

        let bytecode = compiler_result.unwrap();
        return Ok(bytecode);
    }

    fn __display_error(&mut self, error: ErrorKind) {
        match error {
            ErrorKind::ParserError(_) => {
                let error_strings = self.parser.get_formatted_errors();
                println!("Parser Error:");
                for err in error_strings {
                    println!("{}", err);
                }
            }
            ErrorKind::CompileError(c_error) => {
                println!("Compiler Error:");
                println!("{:?}: {}, at: {}", c_error.t, c_error.message, c_error.pos);
            }
            ErrorKind::VMError(vm_error) => {
                println!("Runtime Error:");
                println!(
                    "{:?}: {} at {}, Instruction: {:?}",
                    vm_error.t, vm_error.message, vm_error.pos, vm_error.instruction
                );
            }
        }
    }

    pub fn update(&mut self, new_buffer: Vec<u8>) {
        self.parser.lexer = LexerAPI::new_from_buffer(new_buffer);
    }

    pub fn eval_buffer(buffer: Vec<u8>) -> Option<Rc<Object>> {
        let mut lang = BosonLang::new_from_buffer(buffer);
        let result = lang.eval_state();
        return result;
    }

    pub fn eval_file(filename: String) -> Option<Rc<Object>> {
        let mut lang = BosonLang::new_from_file(filename);
        let result = lang.eval_state();
        return result;
    }

    pub fn disasm_file(filename: String) -> Option<String> {
        let mut lang = BosonLang::new_from_file(filename);
        let result = lang.disasm_state();
        return result;
    }

    pub fn disasm_buffer(buffer: Vec<u8>) -> Option<String> {
        let mut lang = BosonLang::new_from_buffer(buffer);
        let result = lang.disasm_state();
        return result;
    }

    pub fn eval_state(&mut self) -> Option<Rc<Object>> {
        let bytecode = self.__get_bytecode();
        if bytecode.is_err() {
            self.__display_error(bytecode.unwrap_err());
            return None;
        }

        if self.vm.is_none() {
            self.vm = Some(BosonVM::new(&bytecode.unwrap()));
        } else {
            self.vm = Some(
                BosonVM::new_state(&bytecode.unwrap(),
                self.vm.as_mut().unwrap().globals.clone()
            ));
        }
        
        let result = self.vm.as_mut().unwrap().eval_bytecode(true);
        if result.is_err() {
            self.__display_error(ErrorKind::VMError(result.unwrap_err()));
            return None;
        }

        return Some(result.unwrap());
    }

    pub fn disasm_state(&mut self) -> Option<String> {
        let bytecode = self.__get_bytecode();
        if bytecode.is_err() {
            self.__display_error(bytecode.unwrap_err());
            return None;
        }

        let disasm_string = BytecodeDecompiler::disassemble(&bytecode.unwrap());
        return Some(disasm_string);
    }
}