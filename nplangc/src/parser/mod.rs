pub mod ast;
pub mod debug;
pub mod exp;

use crate::lexer::KeywordKind;
use crate::lexer::LexerAPI;
use crate::lexer::SymbolKind;
use crate::lexer::TokenKind;

use debug::ParserError;
use debug::ParserErrorKind;
use debug::ParserErrors;

pub struct Parser {
    pub lexer: LexerAPI,
    pub errors: ParserErrors,
}

impl Parser {
    #[allow(dead_code)]
    pub fn new_from_lexer(lexer: LexerAPI) -> Parser {
        Parser {
            lexer: lexer,
            errors: vec![],
        }
    }

    fn is_terminated(&mut self) -> bool {
        match self.lexer.get_next_token().token {
            TokenKind::Operator(SymbolKind::SSemiColon) => {
                self.lexer.iterate();
                true
            }
            _ => false,
        }
    }

    fn get_identifier(&mut self) -> Result<String, ParserError> {
        let next_token = self.lexer.get_next_token();
        let id_name_res = match next_token.token {
            TokenKind::Identifier(name) => Ok(name.to_string()),
            _ => Err(self.new_invalid_token_err(
                format!("invalid identifier {:?}", next_token)
            )),
        };

        self.lexer.iterate();
        id_name_res
    }

    fn new_invalid_token_err(&mut self, msg: String) -> ParserError {
        ParserError::new(
            ParserErrorKind::UnexpectedToken, msg,
            self.lexer.get_current_token()
        )
    }

    fn parse_var_or_const(&mut self, is_const: bool) -> Result<ast::StatementKind, ParserError> {
        let id_name = self.get_identifier();
        let stmt_result = match id_name {
            Err(error) => Err(error),
            Ok(id) => {
                if self.is_terminated() {
                    if is_const {
                        Err(self.new_invalid_token_err(
                            format!("const {} initialized without any value", id),
                        ))
                    } else {
                        let var_stmt = ast::LetType {
                            identifier: ast::IdentifierType { name: id, t: None },
                            expression: None,
                        };

                        Ok(ast::StatementKind::Var(var_stmt))
                    }
                } else {
                    Err(self.new_invalid_token_err(
                        String::from("Expressions cannot be parsed as of now."),
                    ))
                }
            }
        };

        return stmt_result;
    }

    fn parse_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let statement_result = match current_token.token {
            TokenKind::Keyword(KeywordKind::KBreak) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Break)
                } else {
                    Err(self.new_invalid_token_err(
                        String::from("Expected ; after break."),
                    ))
                }
            }
            TokenKind::Keyword(KeywordKind::KContinue) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Continue)
                } else {
                    Err(self.new_invalid_token_err(
                        String::from("Expected ; after continue."),
                    ))
                }
            }
            TokenKind::Keyword(KeywordKind::KVar) => {
                if self.is_terminated() {
                    Err(self.new_invalid_token_err(String::from("Invalid syntax")))
                } else {
                    self.parse_var_or_const(false)
                }
            }
            TokenKind::Keyword(KeywordKind::KConst) => {
                if self.is_terminated() {
                    Err(self.new_invalid_token_err(String::from("Invalid syntax")))
                } else {
                    self.parse_var_or_const(true)
                }
            }
            _ => Err(self.new_invalid_token_err(String::from("Invalid token"))),
        };

        statement_result
    }

    pub fn parse(&mut self) -> Result<ast::Program, &ParserErrors> {
        // parse and return the program ast
        let mut program = ast::Program { statements: vec![] };
        let mut current_token = self.lexer.get_current_token();
        while !self
            .lexer
            .tokens_are_equal(&current_token.token, TokenKind::EOF)
        {
            let stmt_result = self.parse_statement();
            match stmt_result {
                Ok(stmt) => program.statements.push(stmt),
                Err(error) => self.errors.push(error),
            }

            self.lexer.iterate();
            current_token = self.lexer.get_current_token();
        }

        if self.errors.len() > 0 {
            return Err(&self.errors);
        }

        return Ok(program);
    }
}
