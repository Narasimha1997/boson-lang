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

use exp::get_eval_order;
use exp::ExpOrder;
use exp::InfixExpKind;
use exp::SuffixExpKind;

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

    fn get_next_order(&mut self) -> ExpOrder {
        let token = self.lexer.get_next_token().token;
        match token {
            TokenKind::Operator(sym) => return get_eval_order(&sym),
            _ => return ExpOrder::Zero,
        }
    }

    fn parse_block_statement(&mut self) -> Result<ast::BlockStatement, ParserError> {
        self.lexer.iterate();

        let mut block_statement = ast::BlockStatement {
            statements: vec![],
            pos: vec![],
        };

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
            if self.current_symbol_is(SymbolKind::SSemiColon) {
                self.lexer.iterate();
                continue;
            }

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
        match self.parse_expression(ExpOrder::Zero) {
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
                match self.parse_expression(ExpOrder::Zero) {
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

    fn parse_shell_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        self.lexer.iterate();

        let mut is_raw = false;
        if self.current_symbol_is(SymbolKind::SDot) {
            is_raw = true;
            self.lexer.iterate();
        }

        // parse the next expression:
        let right_expr_res = self.parse_expression(exp::ExpOrder::Zero);
        if right_expr_res.is_err() {
            return right_expr_res;
        }

        return Ok(ast::ExpressionKind::Shell(ast::ShellType {
            shell: Box::new(right_expr_res.unwrap()),
            is_raw,
        }));
    }

    fn parse_function_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        let current_token = self.lexer.get_current_token();
        match current_token.token {
            TokenKind::Identifier(_) => {
                let id_result = self.get_identifier();
                match id_result {
                    Ok(id) => {
                        if !self.next_symbol_is(SymbolKind::SLParen) {
                            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                        }

                        self.lexer.iterate();
                        // parse expression list:
                        match self.parse_list_expr() {
                            Ok(expr_list) => {
                                if !self.next_symbol_is(SymbolKind::SRparen) {
                                    return Err(
                                        self.new_invalid_token_err(String::from("Invalid syntax"))
                                    );
                                }

                                self.lexer.iterate();
                                if !self.next_symbol_is(SymbolKind::SLBrace) {
                                    return Err(
                                        self.new_invalid_token_err(String::from("Invalid syntax"))
                                    );
                                }

                                match self.parse_block_statement() {
                                    Ok(block) => {
                                        return Ok(ast::StatementKind::Function(
                                            ast::FunctionType {
                                                name: id,
                                                parameters: expr_list,
                                                body: block,
                                                return_type: None,
                                            },
                                        ));
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
            _ => {
                return Err(self
                    .new_invalid_token_err(String::from("Expected identifier after func keyword")))
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        if !self.current_symbol_is(SymbolKind::SLParen) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();

        // parse the expression:
        let if_expr = self.parse_expression(ExpOrder::Zero);
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
                                self.parse_expression(ExpOrder::Zero)
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

        if self.next_symbol_is(SymbolKind::SRparen) {
            return Ok(exp_list);
        }

        self.lexer.iterate();

        // first entry:
        let matched_expr = self.parse_expression(ExpOrder::Zero);
        match matched_expr {
            Ok(expr) => exp_list.push(expr),
            Err(error) => return Err(error),
        }

        while self.next_symbol_is(SymbolKind::SComma) {
            self.lexer.iterate();
            self.lexer.iterate();

            let matched_expr = self.parse_expression(ExpOrder::Zero);
            match matched_expr {
                Ok(expr) => exp_list.push(expr),
                Err(error) => return Err(error),
            }
        }

        Ok(exp_list)
    }

    fn parse_pair(&mut self) -> Result<(ast::ExpressionKind, ast::ExpressionKind), ParserError> {
        let key_exp = self.parse_expression(ExpOrder::Zero);
        match key_exp {
            Ok(key) => {
                if !self.next_symbol_is(SymbolKind::SColon) {
                    return Err(self
                        .new_invalid_token_err(String::from("Expected : after key declaration")));
                }

                // parse value:
                self.lexer.iterate();
                self.lexer.iterate();

                let value_exp = self.parse_expression(ExpOrder::Zero);
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
        match self.parse_expression(ExpOrder::Zero) {
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
                match self.parse_expression(ExpOrder::Zero) {
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

    fn parse_return_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        if self.is_terminated() {
            return Ok(ast::StatementKind::Return(ast::ReturnType {
                expression: None,
            }));
        }

        self.lexer.iterate();
        // parse the expression:
        match self.parse_expression(ExpOrder::Zero) {
            Ok(expr) => {
                return Ok(ast::StatementKind::Return(ast::ReturnType {
                    expression: Some(expr),
                }))
            }
            Err(error) => return Err(error),
        }
    }

    fn parse_hash_literal(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let mut h_pairs = vec![];

        if self.next_symbol_is(SymbolKind::SRBrace) {
            self.lexer.iterate();
            return Ok(ast::ExpressionKind::Literal(ast::LiteralKind::HashTable(
                ast::HashTableType { pairs: h_pairs },
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
            ast::HashTableType { pairs: h_pairs },
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

    fn parse_prefix_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        match current_token.token {
            TokenKind::Operator(sym) => {
                let matched_prefix = match sym {
                    SymbolKind::SExcl => exp::PrefixExpKind::Not,
                    SymbolKind::SIncr => exp::PrefixExpKind::PreIncrement,
                    SymbolKind::SDecr => exp::PrefixExpKind::PreDecrement,
                    SymbolKind::SNeg => exp::PrefixExpKind::Neg,
                    _ => {
                        return Err(self.new_invalid_token_err(format!("Invalid prefix {:?}", sym)))
                    }
                };

                self.lexer.iterate();
                let exp_result = self.parse_expression(ExpOrder::Zero);
                if exp_result.is_err() {
                    return Err(exp_result.unwrap_err());
                }

                return Ok(ast::ExpressionKind::Prefix(ast::PrefixType {
                    prefix: matched_prefix,
                    expression: Box::new(exp_result.unwrap()),
                }));
            }
            _ => return Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        }
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

    fn parse_for_each_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        // parse the expression:
        let parsed_exp_result = self.parse_expression(ExpOrder::Zero);
        if parsed_exp_result.is_err() {
            return Err(parsed_exp_result.unwrap_err());
        }

        let parsed_exp = parsed_exp_result.unwrap();

        if !self.next_symbol_is(SymbolKind::SComma) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        self.lexer.iterate();

        let parsed_idx_result = self.parse_expression(ExpOrder::Zero);
        if parsed_idx_result.is_err() {
            return Err(parsed_idx_result.unwrap_err());
        }

        let parsed_idx = parsed_idx_result.unwrap();

        if !self.next_symbol_is(SymbolKind::SComma) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        self.lexer.iterate();

        let parsed_element_result = self.parse_expression(ExpOrder::Zero);
        if parsed_element_result.is_err() {
            return Err(parsed_element_result.unwrap_err());
        }

        let parsed_element = parsed_element_result.unwrap();

        if !self.next_symbol_is(SymbolKind::SImpl) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        let block_result = self.parse_block_statement();
        if block_result.is_err() {
            return Err(block_result.unwrap_err());
        }

        let block = block_result.unwrap();

        // return the forEach type:
        return Ok(ast::StatementKind::ForEach(ast::ForEachType {
            iterator_exp: Box::new(parsed_exp),
            index: Box::new(parsed_idx),
            element: Box::new(parsed_element),
            block: block,
        }));
    }

    fn has_infix(&mut self) -> bool {
        let next_tok = self.lexer.get_next_token();
        match next_tok.token {
            TokenKind::Operator(op) => match op {
                SymbolKind::SPlus
                | SymbolKind::SMinus
                | SymbolKind::SMul
                | SymbolKind::SDiv
                | SymbolKind::SMod
                | SymbolKind::SAnd
                | SymbolKind::SOr
                | SymbolKind::SEq
                | SymbolKind::SEeq
                | SymbolKind::SNe
                | SymbolKind::SGte
                | SymbolKind::SGt
                | SymbolKind::SLte
                | SymbolKind::SLt
                | SymbolKind::SPlusEq
                | SymbolKind::SMinusEq
                | SymbolKind::SMulEq
                | SymbolKind::SDivEq
                | SymbolKind::SModEq
                | SymbolKind::SAndEq
                | SymbolKind::SOrEq
                | SymbolKind::SLOr
                | SymbolKind::SLAnd => return true,
                _ => return false,
            },
            _ => return false,
        }
    }

    #[allow(dead_code)]
    fn has_suffix(&mut self) -> bool {
        let next_tok = self.lexer.get_next_token();
        match next_tok.token {
            TokenKind::Operator(op) => match op {
                SymbolKind::SIncr | SymbolKind::SDecr => return true,
                _ => return false,
            },
            _ => return false,
        }
    }

    fn parse_async_expr(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        self.lexer.iterate();
        let suffix_exp_res = self.parse_expression(exp::ExpOrder::Zero);
        if suffix_exp_res.is_err() {
            return suffix_exp_res;
        }

        let exp = suffix_exp_res.unwrap();
        match exp {
            ast::ExpressionKind::Call(mut ct) => {
                ct.is_async = true;
                return Ok(ast::ExpressionKind::Call(ct));
            }
            _ => {
                return Err(self.new_invalid_token_err(
                    "thread expression requires a callable prefix".to_string(),
                ));
            }
        }
    }

    fn parse_infix_expression(
        &mut self,
        expr_left: ast::ExpressionKind,
    ) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let current_precedence: ExpOrder;

        let infix_sym = match current_token.token {
            TokenKind::Operator(op) => {
                let matched_op_kind = match op {
                    SymbolKind::SPlus => InfixExpKind::Plus,
                    SymbolKind::SMinus => InfixExpKind::Minus,
                    SymbolKind::SMul => InfixExpKind::Mul,
                    SymbolKind::SDiv => InfixExpKind::Div,
                    SymbolKind::SMod => InfixExpKind::Mod,
                    SymbolKind::SAnd => InfixExpKind::And,
                    SymbolKind::SOr => InfixExpKind::Or,
                    SymbolKind::SEq => InfixExpKind::Equal,
                    SymbolKind::SEeq => InfixExpKind::EEqual,
                    SymbolKind::SNe => InfixExpKind::NotEqual,
                    SymbolKind::SGte => InfixExpKind::GreaterThanEqual,
                    SymbolKind::SGt => InfixExpKind::GreaterThan,
                    SymbolKind::SLte => InfixExpKind::LesserThanEqual,
                    SymbolKind::SLt => InfixExpKind::LesserThan,
                    SymbolKind::SPlusEq => InfixExpKind::PlusEq,
                    SymbolKind::SMinusEq => InfixExpKind::MinusEq,
                    SymbolKind::SMulEq => InfixExpKind::MulEq,
                    SymbolKind::SModEq => InfixExpKind::ModEq,
                    SymbolKind::SDivEq => InfixExpKind::DivEq,
                    SymbolKind::SAndEq => InfixExpKind::AndEq,
                    SymbolKind::SOrEq => InfixExpKind::OrEq,
                    SymbolKind::SLOr => InfixExpKind::LogicalOr,
                    SymbolKind::SLAnd => InfixExpKind::LogicalAnd,
                    _ => return Err(self.new_invalid_token_err(String::from("Invalid operator"))),
                };

                current_precedence = get_eval_order(&op);
                matched_op_kind
            }
            _ => {
                return Err(self
                    .new_invalid_token_err(String::from("Expected an operator, invalid syntax")))
            }
        };

        // parse the remaining expression:
        self.lexer.iterate();

        let expr_right = self.parse_expression(current_precedence);

        if expr_right.is_err() {
            return Err(expr_right.unwrap_err());
        }

        let infix_type = ast::InfixType {
            infix: infix_sym,
            expression_right: Box::new(expr_right.unwrap()),
            expression_left: Box::new(expr_left),
        };

        return Ok(ast::ExpressionKind::Infix(infix_type));
    }

    fn parse_suffix_expression(
        &mut self,
        exp_left: ast::ExpressionKind,
    ) -> Result<ast::ExpressionKind, ParserError> {
        let current_token = self.lexer.get_current_token();
        let matched_suffix = match current_token.token {
            TokenKind::Operator(op) => {
                let matched_symbol = match op {
                    SymbolKind::SIncr => SuffixExpKind::PostIncrement,
                    SymbolKind::SDecr => SuffixExpKind::PostDecrement,
                    _ => return Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
                };

                matched_symbol
            }
            _ => return Err(self.new_invalid_token_err(String::from("Invalid syntax"))),
        };

        let suffix_type = ast::SuffixType {
            expression: Box::new(exp_left),
            suffix: matched_suffix,
        };

        return Ok(ast::ExpressionKind::Suffix(suffix_type));
    }

    fn parse_for_loop_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        if self.next_symbol_is(SymbolKind::SSemiColon) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();

        // parse the expression
        let mut parsed_exp_result = self.parse_expression(ExpOrder::Zero);
        if parsed_exp_result.is_err() {
            return Err(parsed_exp_result.unwrap_err());
        }

        let target_expression = parsed_exp_result.unwrap();

        if !self.next_keyword_is(KeywordKind::KIn) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        self.lexer.iterate();

        // parse the iterator expression:
        parsed_exp_result = self.parse_expression(ExpOrder::Zero);
        if parsed_exp_result.is_err() {
            return Err(parsed_exp_result.unwrap_err());
        }

        let iterator_expression = parsed_exp_result.unwrap();

        if !self.next_symbol_is(SymbolKind::SImpl) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();

        if !self.next_symbol_is(SymbolKind::SLBrace) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        // parse the loop block:
        let block_result = self.parse_block_statement();
        if block_result.is_err() {
            return Err(block_result.unwrap_err());
        }

        let block = block_result.unwrap();

        return Ok(ast::StatementKind::For(ast::ForLoopType {
            target: Box::new(target_expression),
            iter: Box::new(iterator_expression),
            loop_block: block,
        }));
    }

    fn parse_thread_exp(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        self.lexer.iterate();
        let suffix_exp_res = self.parse_expression(exp::ExpOrder::Zero);
        if suffix_exp_res.is_err() {
            return suffix_exp_res;
        }

        let exp = suffix_exp_res.unwrap();
        match exp {
            ast::ExpressionKind::Call(mut ct) => {
                ct.is_thread = true;
                return Ok(ast::ExpressionKind::Call(ct));
            }
            _ => {
                return Err(self.new_invalid_token_err(
                    "thread expression requires a callable prefix".to_string(),
                ));
            }
        }
    }

    fn parse_expression(&mut self, pre: ExpOrder) -> Result<ast::ExpressionKind, ParserError> {
        // when an expression is called, the current_token should be at the starting
        // the next_token should point to the next token after start.

        let current_token = self.lexer.get_current_token();
        let mut matched_prefix = match current_token.token {
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
                        Ok(ast::ExpressionKind::Literal(ast::LiteralKind::Bool(false)))
                    }
                    KeywordKind::KLambda => self.parse_lambda_exp(),
                    KeywordKind::KNone => Ok(ast::ExpressionKind::Noval),
                    KeywordKind::KThread => self.parse_thread_exp(),
                    KeywordKind::KAsync => self.parse_async_expr(),
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
                    SymbolKind::SIncr
                    | SymbolKind::SDecr
                    | SymbolKind::SNeg
                    | SymbolKind::SExcl => self.parse_prefix_expression(),
                    SymbolKind::SLParen => self.parse_sub_expression(),
                    SymbolKind::SDollar => self.parse_shell_expression(),
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

        while !self.next_symbol_is(SymbolKind::SSemiColon) && pre < self.get_next_order() {
            if matched_prefix.is_err() {
                return Err(matched_prefix.unwrap_err());
            }

            if self.has_infix() {
                self.lexer.iterate();
                matched_prefix = self.parse_infix_expression(matched_prefix.unwrap());
            } else if self.has_suffix() {
                self.lexer.iterate();
                matched_prefix = self.parse_suffix_expression(matched_prefix.unwrap());
            } else if self.next_symbol_is(SymbolKind::SLParen) {
                self.lexer.iterate();
                matched_prefix = self.parse_call_expression(matched_prefix.unwrap());
            } else if self.next_symbol_is(SymbolKind::SLBox) {
                self.lexer.iterate();
                matched_prefix = self.parse_index_expression(matched_prefix.unwrap());
            } else {
                break;
            }
        }

        return matched_prefix;
    }

    fn parse_index_expression(
        &mut self,
        prefix_exp: ast::ExpressionKind,
    ) -> Result<ast::ExpressionKind, ParserError> {
        self.lexer.iterate();
        let exp_result = self.parse_expression(ExpOrder::Zero);
        if exp_result.is_err() {
            return Err(exp_result.unwrap_err());
        }

        if !self.next_symbol_is(SymbolKind::SRBox) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        let index_type = ast::IndexType {
            expression_left: Box::new(prefix_exp),
            index: Box::new(exp_result.unwrap()),
        };

        return Ok(ast::ExpressionKind::Index(index_type));
    }

    fn parse_call_expression(
        &mut self,
        caller_expr: ast::ExpressionKind,
    ) -> Result<ast::ExpressionKind, ParserError> {
        if self.next_symbol_is(SymbolKind::SRparen) {
            self.lexer.iterate();
            return Ok(ast::ExpressionKind::Call(ast::CallType {
                function: Box::new(caller_expr),
                arguments: vec![],
                is_thread: false,
                is_async: false
            }));
        }

        let args_list_result = self.parse_list_expr();
        if args_list_result.is_err() {
            return Err(args_list_result.unwrap_err());
        }

        self.lexer.iterate();
        let call_type = ast::CallType {
            function: Box::new(caller_expr),
            arguments: args_list_result.unwrap(),
            is_thread: false,
            is_async: false,
        };

        return Ok(ast::ExpressionKind::Call(call_type));
    }

    fn parse_throw_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        self.lexer.iterate();

        // parse the throw expression:
        let parsed_exp_result = self.parse_expression(ExpOrder::Zero);
        if parsed_exp_result.is_err() {
            return Err(parsed_exp_result.unwrap_err());
        }

        let throw_expression = parsed_exp_result.unwrap();
        return Ok(ast::StatementKind::Throw(ast::ThrowType {
            expression: Box::new(throw_expression),
        }));
    }

    fn parse_sub_expression(&mut self) -> Result<ast::ExpressionKind, ParserError> {
        self.lexer.iterate();
        if self.next_symbol_is(SymbolKind::SRparen) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        let sub_exp_result = self.parse_expression(ExpOrder::Zero);
        if sub_exp_result.is_err() {
            return Err(sub_exp_result.unwrap_err());
        }

        if !self.next_symbol_is(SymbolKind::SRparen) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        self.lexer.iterate();
        return Ok(sub_exp_result.unwrap());
    }

    fn parse_try_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        if !self.next_symbol_is(SymbolKind::SLBrace) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        // parse the block statement of try:
        let try_block_result = self.parse_block_statement();
        if try_block_result.is_err() {
            return Err(try_block_result.unwrap_err());
        }

        let try_block = try_block_result.unwrap();

        if !self.next_keyword_is(KeywordKind::KCatch) {
            return Err(
                self.new_invalid_token_err(String::from("try without catch is not accepted"))
            );
        }

        self.lexer.iterate();
        self.lexer.iterate();

        // parse catch block exp:
        let catch_exp_result = self.parse_expression(ExpOrder::Zero);
        if catch_exp_result.is_err() {
            return Err(catch_exp_result.unwrap_err());
        }

        let catch_exp = catch_exp_result.unwrap();

        if !self.next_symbol_is(SymbolKind::SLBrace) {
            return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
        }

        // parse the catch block:
        let catch_block_result = self.parse_block_statement();
        if catch_block_result.is_err() {
            return Err(catch_block_result.unwrap_err());
        }

        let catch_block = catch_block_result.unwrap();

        if !self.next_keyword_is(KeywordKind::KFinally) {
            return Ok(ast::StatementKind::TryCatch(ast::TryCatchType {
                try_block: try_block,
                catch_block: catch_block,
                exception_ident: Box::new(catch_exp),
                final_block: None,
            }));
        }

        self.lexer.iterate();

        // parse finally block:
        let final_block_result = self.parse_block_statement();
        if final_block_result.is_err() {
            return Err(final_block_result.unwrap_err());
        }

        let final_block = final_block_result.unwrap();

        return Ok(ast::StatementKind::TryCatch(ast::TryCatchType {
            try_block: try_block,
            catch_block: catch_block,
            exception_ident: Box::new(catch_exp),
            final_block: Some(final_block),
        }));
    }

    fn parse_expression_statement(&mut self) -> Result<ast::StatementKind, ParserError> {
        let parsed_exp = self.parse_expression(ExpOrder::Zero);
        if parsed_exp.is_err() {
            return Err(parsed_exp.unwrap_err());
        }

        return Ok(ast::StatementKind::Expression(parsed_exp.unwrap()));
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

            TokenKind::Keyword(KeywordKind::KReturn) => {
                return self.parse_return_statement();
            }

            TokenKind::Keyword(KeywordKind::KFunc) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_function_statement();
                }
            }
            TokenKind::Keyword(KeywordKind::KFor) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_for_loop_statement();
                }
            }

            TokenKind::Keyword(KeywordKind::KThrow) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_throw_statement();
                }
            }

            TokenKind::Keyword(KeywordKind::KTry) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_try_statement();
                }
            }

            TokenKind::Keyword(KeywordKind::KForEach) => {
                if self.is_terminated() {
                    return Err(self.new_invalid_token_err(String::from("Invalid syntax")));
                } else {
                    return self.parse_for_each_statement();
                }
            }

            TokenKind::Empty => return Ok(ast::StatementKind::Empty),
            _ => return self.parse_expression_statement(),
        }
    }

    pub fn parse(&mut self) -> Result<ast::Program, &ParserErrors> {
        // parse and return the program ast
        let mut program = ast::Program {
            statements: vec![],
            pos: vec![],
        };
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

            if self.current_symbol_is(SymbolKind::SSemiColon) {
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

    pub fn get_formatted_errors(&mut self) -> Vec<String> {
        let mut error_strings = vec![];

        for err in &self.errors {
            let (err_line, b_pos, _) = self.lexer.get_line_by_pos(err.pos);
            let pointed_line = format!("{:0pos$}", "^^^", pos = (err.pos - b_pos));

            error_strings.push(format!(
                "Parsing Error at position: {}, Token: {:?}\n\t{}\n\t{}\n{}",
                err.pos, err.error_token, err_line, pointed_line, err.message
            ))
        }

        return error_strings;
    }

    pub fn reset_errors(&mut self) {
        self.errors.clear();
    }
}
