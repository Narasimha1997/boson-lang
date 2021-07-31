pub mod parser;
pub mod lexer;
pub mod compiler;
pub mod types;
pub mod config;
pub mod vm;
pub mod isa;

fn main() {
   let lexer_api = lexer::LexerAPI::new_from_file(
      String::from("test.np")
   );

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
         let mut boson_vm = vm::BosonVM::new(&bytecode_result.unwrap());
         let result = boson_vm.eval_bytecode();
         if result.is_err() {
            println!("{:?}", result);
         }

         let globals = boson_vm.dump_globals();
         let ds = boson_vm.dump_ds();
         println!("Globals:\n{}", globals);
         println!("Data Stack:\n{}", ds);
      }
   }
   
}