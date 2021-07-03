pub mod parser;
pub mod lexer;

fn main() {
   let mut l =  lexer::ProgramLexer::new_from_file(
      String::from("test.np")
   );

   let token = l.next_lexed_token();

   let pe = parser::debug::ParserError::new(
      parser::debug::ParserErrorKind::UnexpectedToken,
      String::from("Test error"),
      token
   );

   println!("{:?}", pe)
}