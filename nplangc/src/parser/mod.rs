pub mod ast;
pub mod debug;
pub mod exp;

use crate::lexer::LexerAPI;
use debug::ParserErrors;

pub struct Parser {
    pub lexer: LexerAPI,
    pub errors: ParserErrors
}

impl Parser {

    #[allow(dead_code)]
    fn new_from_lexer(&mut self, lexer: LexerAPI) -> Parser {
        Parser {
            lexer: lexer,
            errors: vec!()
        }
    }
}