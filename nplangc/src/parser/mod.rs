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
            _ => Err(self.new_invalid_token_err(format!("invalid identifier {:?}", next_token))),
        };

        self.lexer.iterate();
        id_name_res
    }

    fn new_invalid_token_err(&mut self, msg: String) -> ParserError {
        ParserError::new(
            ParserErrorKind::UnexpectedToken,
            msg,
            self.lexer.get_current_token(),
        )
    }

    fn parse_var_or_const(&mut self, is_const: bool) -> Result<ast::StatementKind, ParserError> {
        let id_name = self.get_identifier();
        let stmt_result = match id_name {
            Err(error) => Err(error),
            Ok(id) => {
                if self.is_terminated() {
                    if is_const {
                        Err(self.new_invalid_token_err(format!(
                            "const {} initialized without any value",
                            id
                        )))
                    } else {
                        let var_stmt = ast::LetType {
                            identifier: ast::IdentifierType { name: id, t: None },
                            expression: None,
                        };

                        Ok(ast::StatementKind::Var(var_stmt))
                    }
                } else {
                    self.lexer.iterate();
                    let current_token = self.lexer.get_current_token();
                    let expr_result = match current_token.token {
                        TokenKind::Operator(op) => {
                            if self.lexer.symbols_are_equal(&op, SymbolKind::SEq) {
                                self.lexer.iterate();
                                self.parse_expression()
                            } else {
                                Err(self.new_invalid_token_err(String::from("Expected =")))
                            }
                        }
                        _ => Err(self.new_invalid_token_err(String::from("Expected ="))),
                    };

                    let let_const_stmt = match expr_result {
                        Ok(expr) => {
                            if is_const {
                                Ok(ast::StatementKind::Const(ast::ConstType {
                                    identifier: ast::IdentifierType { name: id, t: None },
                                    expression: Some(expr),
                                }))
                            } else {
                                Ok(ast::StatementKind::Var(ast::LetType {
                                    identifier: ast::IdentifierType { name: id, t: None },
                                    expression: Some(expr),
                                }))
                            }
                        }
                        Err(error) => Err(error),
                    };

                    let_const_stmt
                }
            }
        };

        return stmt_result;
    }

    fn next_symbol_is(&mut self, compare: SymbolKind) -> bool {
        let next_token = self.lexer.get_next_token();
        let result = match next_token.token {
            TokenKind::Operator(op) => self.lexer.symbols_are_equal(&op, compare),
            _ => false,
        };

        result
    }

    #[allow(dead_code)]
    fn current_symbol_is(&mut self, compare: SymbolKind) -> bool {
        let current_token = self.lexer.get_current_token();
        let result = match current_token.token {
            TokenKind::Operator(op) => self.lexer.symbols_are_equal(&op, compare),
            _ => false,
        };

        result
    }

    fn parse_list_expr(&mut self) -> Result<Vec<ast::ExpressionKind>, ParserError> {
        let mut exp_list = vec![];

        self.lexer.iterate();

        // first entry:
        let matched_expr = self.parse_expression();
        match matched_expr {
            Ok(expr) => exp_list.push(expr),
            Err(error) => return Err(error),
        }

        while self.next_symbol_is(SymbolKind::SComma) {
            self.lexer.iterate();
            self.lexer.iterate();

            let matched_expr = self.parse_expression();
            match matched_expr {
                Ok(expr) => exp_list.push(expr),
                Err(error) => return Err(error),
            }
        }

        Ok(exp_list)
    }

    fn parse_pair(&mut self) -> Result<(ast::ExpressionKind, ast::ExpressionKind), ParserError> {
        let key_exp = self.parse_expression();
        match key_exp {
            Ok(key) => {
                if !self.next_symbol_is(SymbolKind::SColon) {
                    return Err(self
                        .new_invalid_token_err(String::from("Expected : after key declaration")));
                }

                // parse value:
                self.lexer.iterate();
                self.lexer.iterate();

                let value_exp = self.parse_expression();
                match value_exp {
                    Ok(value) => return Ok((key, value)),
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }

    fn parse_hash_literal(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let mut h_pairs = vec![];

        if self.next_symbol_is(SymbolKind::SRBrace) {
            self.lexer.iterate();
            return Ok(ast::ExpressionKind::Literal(ast::LiteralKind::HashTable(
                h_pairs,
            )));
        }

        self.lexer.iterate();

        match self.parse_pair() {
            Ok((key, value)) => h_pairs.push((key, value)),
            Err(error) => return Err(error),
        }
        // iterate over the dict and parse pair
        while self.next_symbol_is(SymbolKind::SComma) {
            self.lexer.iterate();
            self.lexer.iterate();

            match self.parse_pair() {
                Ok((key, value)) => h_pairs.push((key, value)),
                Err(error) => return Err(error),
            }
        }

        // check if is terminated:
        if !self.next_symbol_is(SymbolKind::SRBrace) {
            return Err(
                self.new_invalid_token_err(String::from("Hash-literal is not terminated with }"))
            );
        }
        self.lexer.iterate();
        return Ok(ast::ExpressionKind::Literal(ast::LiteralKind::HashTable(
            h_pairs,
        )));
    }

    fn parse_array_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        if self.next_symbol_is(SymbolKind::SRBox) {
            self.lexer.iterate();
            let array = ast::ArrayType {
                array_values: vec![],
                length: 0,
            };

            return Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Array(array)));
        }

        let list_expr = self.parse_list_expr();
        match list_expr {
            Ok(le) => {
                if self.next_symbol_is(SymbolKind::SRBox) {
                    self.lexer.iterate();

                    let arr_len = le.len();
                    let array = ast::ArrayType {
                        array_values: le,
                        length: arr_len,
                    };

                    return Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Array(array)));
                } else {
                    return Err(
                        self.new_invalid_token_err(String::from("Array literal not terminated"))
                    );
                }
            }
            Err(error) => return Err(error),
        }
    }

    fn parse_integer_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let int_literal = match current_token.token {
            TokenKind::Integer(num) => Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Int(
                num.clone(),
            ))),
            _ => Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        };

        int_literal
    }

    fn parse_floating_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let float_literal = match current_token.token {
            TokenKind::Float(num) => Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Float(
                num.clone(),
            ))),
            _ => Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        };

        float_literal
    }

    fn parse_char_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let char_literal = match current_token.token {
            TokenKind::Char(ch) => Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Char(
                ch.clone(),
            ))),
            _ => Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        };

        char_literal
    }

    fn parse_string_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let string_literal = match current_token.token {
            TokenKind::Str(str) => Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Str(
                str.clone(),
            ))),
            _ => Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        };

        string_literal
    }

    fn parse_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        // when an expression is called, the current_token should be at the starting
        // the next_token should point to the next token after start.

        let current_token = self.lexer.get_current_token();
        let matched_prefix = match current_token.token {
            // literals:
            TokenKind::Integer(_) => self.parse_integer_expression(),
            TokenKind::Float(_) => self.parse_floating_expression(),
            TokenKind::Char(_) => self.parse_char_expression(),
            TokenKind::Str(_) => self.parse_string_expression(),
            TokenKind::Keyword(kw) => {
                // all expressions that start with a keyword:
                let kw_exp_result = match kw {
                    KeywordKind::KTrue => {
                        Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Bool(true)))
                    }
                    KeywordKind::KFalse => {
                        Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Bool(true)))
                    }
                    _ => Err(self
                        .new_invalid_token_err(String::from("Functionality not yet implemented"))),
                };

                kw_exp_result
            }
            // starts with a symbol:
            TokenKind::Operator(op) => {
                let op_expr_result = match op {
                    SymbolKind::SLBox => self.parse_array_expression(),
                    SymbolKind::SLBrace => self.parse_hash_literal(),
                    _ => Err(self.new_invalid_token_err(String::from("Invalid symbol"))),
                };

                op_expr_result
            }
            _ => Err(self.new_invalid_token_err(String::from("Invalid token"))),
        };

        // check if next token is termination:
        if self.is_terminated() {
            return matched_prefix;
        }

        return matched_prefix;
    }

    fn parse_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let statement_result = match current_token.token {
            TokenKind::Keyword(KeywordKind::KBreak) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Break)
                } else {
                    Err(self.new_invalid_token_err(String::from("Expected ; after break.")))
                }
            }
            TokenKind::Keyword(KeywordKind::KContinue) => {
                if self.is_terminated() {
                    Ok(ast::StatementKind::Continue)
                } else {
                    Err(self.new_invalid_token_err(String::from("Expected ; after continue.")))
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
            if self
                .lexer
                .tokens_are_equal(&current_token.token, TokenKind::Empty)
            {
                self.lexer.iterate();
                current_token = self.lexer.get_current_token();
                continue;
            }

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
