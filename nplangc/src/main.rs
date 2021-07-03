pub mod parser;
pub mod lexer;

fn main() {
   let mut lexer_api = lexer::LexerAPI::new_from_file(
      String::from("test.np")
   );

   let err_string = lexer_api.get_line_by_pos(100000);
   println!("{}", err_string);
}