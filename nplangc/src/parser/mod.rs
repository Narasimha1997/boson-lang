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

    fn is_empty_statement(&mut self, stmt: &ast::StatementKind) -> bool {
        stmt == &ast::StatementKind::Empty
    }

    fn parse_block_statement(&mut self) -> Result<ast::BlockStatement, ParserError> {
        self.lexer.iterate();

        let mut block_statement = ast::BlockStatement { statements: vec![] };

        if self.next_symbol_is(SymbolKind::SRBrace) {
            self.lexer.iterate();
            return Ok(block_statement);
        }

        self.lexer.iterate();

        // parse the first statement:
        match self.parse_statement() {
            Ok(stmt) => {
                if !self.is_empty_statement(&stmt) {
                    block_statement.statements.push(stmt);
                }
            }
            Err(error) => return Err(error),
        }

        self.lexer.iterate();

        // parse each statement, until a '}' is encountered
        while !self.current_symbol_is(SymbolKind::SRBrace) {
            match self.parse_statement() {
                Ok(stmt) => {
                    if !self.is_empty_statement(&stmt) {
                        block_statement.statements.push(stmt);
                    }
                    self.lexer.iterate();
                }
                Err(error) => return Err(error),
            }
        }

        return Ok(block_statement);
    }

    fn parse_assert_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        // parse the left expression:
        match self.parse_expression() {
            Ok(expr_target) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from(
                        "Assertion requires an action to be defined on failure",
                    )));
                }

                // parse the alternative expresion:
                if !self.next_symbol_is(SymbolKind::SComma) {
                    return Err(self.new_invalid_token_err(String::from(
                        "Expected , after assert expression",
                    )));
                }

                self.lexer.iterate();
                self.lexer.iterate();

                // parse the expression:
                match self.parse_expression() {
                    Ok(fail_expr) => {
                        return Ok(ast::StatementKind::Assert(ast::AssertType {
                            target_expr: Box::new(expr_target),
                            fail_expr: Box::new(fail_expr),
                        }))
                    }
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }

    fn parse_if_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        if !self.current_symbol_is(SymbolKind::SLParen) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();

        // parse the expression:
        let if_expr = self.parse_expression();
        match if_expr {
            Ok(expr) => {
                // parse the block statement and else expression if present:
                if !self.next_symbol_is(SymbolKind::SRparen) {
                    return Err(self.new_invalid_token_err(String::from("Invalid token")));
                }

                self.lexer.iterate();
                if !self.next_symbol_is(SymbolKind::SLBrace) {
                    return Err(
                        self.new_invalid_token_err(String::from("If statement without body"))
                    );
                }

                // parse the block statement:
                println!("{:?}", self.lexer.get_current_token());

                let if_block_stmts = self.parse_block_statement();
                match if_block_stmts {
                    Ok(m_stmts) => {
                        // check if the current token is else:
                        if !self.next_keyword_is(KeywordKind::KElse) {
                            return Ok(ast::StatementKind::If(ast::IfElseType {
                                condition: Box::new(expr),
                                main_block: m_stmts,
                                alternate_block: None,
                            }));
                        }

                        // else is present:
                        self.lexer.iterate();
                        if !self.next_symbol_is(SymbolKind::SLBrace) {
                            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                        }

                        let block_statement = self.parse_block_statement();
                        match block_statement {
                            Ok(a_stmts) => {
                                return Ok(ast::StatementKind::If(ast::IfElseType {
                                    condition: Box::new(expr),
                                    main_block: m_stmts,
                                    alternate_block: Some(a_stmts),
                                }))
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }

    fn get_identifier(&mut self) -> Result<String, ParserError> {
        let current_token = self.lexer.get_current_token();
        let id_name_res = match current_token.token {
            TokenKind::Identifier(name) => Ok(name.to_string()),
            _ => Err(self.new_invalid_token_err(format!("invalid identifier {:?}", current_token))),
        };
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
        self.lexer.iterate();

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

    fn next_keyword_is(&mut self, compare: KeywordKind) -> bool {
        let next_token = self.lexer.get_next_token();
        let result = match next_token.token {
            TokenKind::Keyword(kw) => self.lexer.keywords_are_equal(&kw, compare),
            _ => false,
        };

        result
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

    fn parse_while_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        if !self.next_symbol_is(SymbolKind::SLParen) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        self.lexer.iterate();

        // parse the expression:
        match self.parse_expression() {
            Ok(expr) => {
                // parse the block statement:
                if !self.next_symbol_is(SymbolKind::SRparen) {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                }

                self.lexer.iterate();

                // parse the block statement:
                match self.parse_block_statement() {
                    Ok(block) => {
                        return Ok(ast::StatementKind::While(ast::WhileLoopType {
                            target_expr: Box::new(expr),
                            loop_block: block,
                        }))
                    }
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }

    fn parse_lambda_exp(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        if self.next_symbol_is(SymbolKind::SSemiColon) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        if self.next_symbol_is(SymbolKind::SImpl) {
            return Err(self.new_invalid_token_err(String::from(
                "Lambdas with zero parameters are not accepted",
            )));
        }

        // parse parameters list:
        match self.parse_list_expr() {
            Ok(lparams) => {
                if !self.next_symbol_is(SymbolKind::SImpl) {
                    return Err(self.new_invalid_token_err(String::from(
                        "Expected => after parameters declaration",
                    )));
                }

                self.lexer.iterate();
                self.lexer.iterate();
                // parse the right expression:
                match self.parse_expression() {
                    Ok(expr) => {
                        return Ok(ast::ExpressionKind::Lambda(ast::LambdaExpType {
                            expression: Box::new(expr),
                            parameters: lparams,
                        }))
                    }

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
            TokenKind::Identifier(_) => match self.get_identifier() {
                Ok(ident) => Ok(ast::ExpressionKind::Identifier(ast::IdentifierType {
                    name: ident,
                    t: None,
                })),
                _ => Err(self.new_invalid_token_err(String::from("Invalid identifier"))),
            },
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
                    KeywordKind::KLambda => self.parse_lambda_exp(),
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
        match current_token.token {
            TokenKind::Keyword(KeywordKind::KBreak) => {
                if self.is_terminated() {
                    return Ok(ast::StatementKind::Break);
                } else {
                    return Err(self.new_invalid_token_err(String::from("Expected ; after break.")));
                }
            }
            TokenKind::Keyword(KeywordKind::KContinue) => {
                if self.is_terminated() {
                    return Ok(ast::StatementKind::Continue);
                } else {
                    return Err(
                        self.new_invalid_token_err(String::from("Expected ; after continue."))
                    );
                }
            }
            TokenKind::Keyword(KeywordKind::KVar) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_var_or_const(false);
                }
            }
            TokenKind::Keyword(KeywordKind::KConst) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_var_or_const(true);
                }
            }
            TokenKind::Keyword(KeywordKind::KIf) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_if_statement();
                }
            }
            TokenKind::Keyword(KeywordKind::KAssert) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_assert_statement();
                }
            }
            TokenKind::Keyword(KeywordKind::KWhile) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_while_statement();
                }
            }
            TokenKind::Empty => return Ok(ast::StatementKind::Empty),
            _ => return Err(self.new_invalid_token_err(String::from("Invalid token"))),
        }
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
