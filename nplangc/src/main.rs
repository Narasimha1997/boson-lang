pub mod parser;
pub mod lexer;

fn main() {
   let lexer_api = lexer::LexerAPI::new_from_file(
      String::from("test.np")
   );

   let mut parser = parser::Parser::new_from_lexer(lexer_api);
   let parsed_result = parser.parse();

   println!("{:?}", parsed_result);
}