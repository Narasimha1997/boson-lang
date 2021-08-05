pub mod compiler;
pub mod config;
pub mod isa;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod vm;

use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();

   if args.len() == 1 {
      println!("Welcome to Bosonlang, provide the file-name as argument.");
   } else {

      let lexer_api = lexer::LexerAPI::new_from_file(args[1].clone());

      let mut parser = parser::Parser::new_from_lexer(lexer_api);
      let parsed_result = parser.parse();

      if parsed_result.is_err() {
         let errors = parser.get_formatted_errors();
         for err in &errors {
            println!("{}", err);
         }
      } else {
         let mut p_compiler = compiler::BytecodeCompiler::new();
         let bytecode_result = p_compiler.compile(&parsed_result.unwrap());
         if bytecode_result.is_err() {
            let err = bytecode_result.unwrap_err();
            println!("Compilation Error: {:?}", err);
         } else {
            let bytecode = bytecode_result.unwrap();
            // println!("{}", compiler::BytecodeDecompiler::disassemble(&bytecode));
            let mut boson_vm = vm::BosonVM::new(&bytecode);
            let result = boson_vm.eval_bytecode();

            if result.is_err() {
               println!("{:?}", result);
               let globals = boson_vm.dump_globals();
               let ds = boson_vm.dump_ds();
               println!("Globals:\n{}", globals);
               println!("Data Stack:\n{}", ds);
               println!("Stack Pointer:\n{}", boson_vm.data_stack.stack_pointer);
            }
         }
      }
   }
}
