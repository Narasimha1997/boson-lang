pub mod ast;
pub mod debug;
pub mod exp;

use crate::lexer::LexerAPI;
use crate::lexer::TokenKind;
use crate::lexer::KeywordKind;
use crate::lexer::SymbolKind;
use crate::lexer::LexedToken;

use debug::ParserErrors;
use debug::ParserError;
use debug::ParserErrorKind;

pub struct Parser {
    pub lexer: LexerAPI,
    pub errors: ParserErrors
}

impl Parser {

    #[allow(dead_code)]
    pub fn new_from_lexer(lexer: LexerAPI) -> Parser {
        Parser {
            lexer: lexer,
            errors: vec!()
        }
    }

    fn is_terminated(&mut self) -> bool {
        match self.lexer.get_next_token().token {
            TokenKind::Operator(SymbolKind::SSemiColon) => {
                self.lexer.iterate();
                true
            }
            _ => false
        }
    }

    fn new_invalid_token_err(&mut self, token: LexedToken, msg: String) -> ParserError {
        ParserError::new(
            ParserErrorKind::UnexpectedToken,
            msg,
            token
        )
    }

    fn parse_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        
        let statement_result = match current_token.token {
            TokenKind::Keyword(KeywordKind::KBreak) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Break)
                } else {
                    Err(self.new_invalid_token_err(
                        current_token, String::from("Expected ; after break.")
                    ))
                }
            }
            TokenKind::Keyword(KeywordKind::KContinue) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Continue)
                } else {
                    Err(self.new_invalid_token_err(
                        current_token, String::from("Expected ; after continue.")
                    ))
                }
            }
            _ => {
                Err(self.new_invalid_token_err(
                   current_token , String::from("Invalid token")
                ))
            }
        };

        statement_result
    }


    pub fn parse(&mut self) -> Result<ast::Program, &ParserErrors> {
        // parse and return the program ast
        let mut program = ast::Program{statements: vec!()};
        let mut current_token = self.lexer.get_current_token();
        while !self.lexer.tokens_are_equal(&current_token.token, TokenKind::EOF) {
            let stmt_result = self.parse_statement();
            match stmt_result {
                Ok(stmt) => program.statements.push(stmt),
                Err(error) => self.errors.push(error)
            }

            self.lexer.iterate();
            current_token = self.lexer.get_current_token();
        }

        if self.errors.len() > 0 {
            return Err(&self.errors)
        }

        return Ok(program);
    }
}