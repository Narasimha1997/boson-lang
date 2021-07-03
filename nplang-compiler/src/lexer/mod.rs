pub mod types;

use types::KeywordKind;
use types::SymbolKind;
use types::TokenKind;
use types::LexedToken;
use types::EOF_BYTE;

use std::fs;
use std::io::Write;

#[allow(dead_code)]
pub struct StreamingLexer {
    file_name: String,
}

#[allow(dead_code)]
pub struct ProgramLexer {
    pub buffer: types::ProgramBuffer,
    pub current_char: u8,
}

impl ProgramLexer {
    #[allow(dead_code)]
    pub fn new_from_file(file_name: String) -> ProgramLexer {
        let mut lexer = ProgramLexer {
            buffer: types::ProgramBuffer::new_from_file(file_name),
            current_char: 0,
        };

        // read the first character from program buffer and return.
        lexer.read_next();
        return lexer;
    }

    fn read_next(&mut self) {
        self.current_char = self.buffer.next_char();
    }

    fn look_next_byte(&mut self) -> u8 {
        return self.buffer.peek_next();
    }

    fn find_number_literal(&mut self) -> TokenKind {
        let start_pos = self.buffer.current_pos;

        let mut is_float = false;

        loop {
            match self.current_char {
                b'0'..=b'9' | b'.' => {
                    if self.current_char == b'.' {
                        is_float = true;
                    }
                    self.read_next();
                }
                _ => {
                    break;
                }
            }
        }

        if is_float {
            return TokenKind::Float(self.buffer.get_as_f64(start_pos, self.buffer.current_pos));
        } else {
            return TokenKind::Integer(self.buffer.get_as_i64(start_pos, self.buffer.current_pos));
        }
    }

    fn find_keyword_or_identifier(&mut self) -> TokenKind {
        let start_pos = self.buffer.current_pos;

        loop {
            match self.current_char {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                    self.read_next();
                    continue;
                }
                _ => {
                    break;
                }
            }
        }

        // get the string view of the buffer:
        let id_string = self
            .buffer
            .get_as_string(start_pos, self.buffer.current_pos);

        // match against the keywords:
        let token = match id_string.as_str() {
            "if" => TokenKind::Keyword(KeywordKind::KIf),
            "else" => TokenKind::Keyword(KeywordKind::KElse),
            "while" => TokenKind::Keyword(KeywordKind::KWhile),
            "for" => TokenKind::Keyword(KeywordKind::KFor),
            "break" => TokenKind::Keyword(KeywordKind::KBreak),
            "continue" => TokenKind::Keyword(KeywordKind::KContinue),
            "const" => TokenKind::Keyword(KeywordKind::KConst),
            "var" => TokenKind::Keyword(KeywordKind::KVar),
            "none" => TokenKind::Keyword(KeywordKind::KNone),
            "func" => TokenKind::Keyword(KeywordKind::KFunc),
            "return" => TokenKind::Keyword(KeywordKind::KReturn),
            "try" => TokenKind::Keyword(KeywordKind::KTry),
            "catch" => TokenKind::Keyword(KeywordKind::KCatch),
            "finally" => TokenKind::Keyword(KeywordKind::KFinally),
            "rethrow" => TokenKind::Keyword(KeywordKind::KRethrow),
            "throw" => TokenKind::Keyword(KeywordKind::KThrow),
            "as" => TokenKind::Keyword(KeywordKind::KAs),
            "true" => TokenKind::Keyword(KeywordKind::KTrue),
            "false" => TokenKind::Keyword(KeywordKind::KFalse),
            "foreach" => TokenKind::Keyword(KeywordKind::KForEach),
            "in" => TokenKind::Keyword(KeywordKind::KIn),
            "indexed" => TokenKind::Keyword(KeywordKind::KIndexed),
            "pure" => TokenKind::Keyword(KeywordKind::KPure),
            "lambda" => TokenKind::Keyword(KeywordKind::KLambda),

            _ => TokenKind::Identifier(id_string),
        };

        return token;
    }

    fn find_string_literal(&mut self) -> TokenKind {
        self.read_next();

        let mut prev_char = self.current_char;
        let start_pos = self.buffer.current_pos;
        loop {
            match self.current_char {
                b'"' => {
                    if prev_char == b'\\' {
                        prev_char = self.current_char;
                        self.read_next();
                        continue;
                    }

                    break;
                }
                _ => {
                    prev_char = self.current_char;
                    self.read_next();
                    continue;
                }
            }
        }

        let string_literal = self
            .buffer
            .get_as_string(start_pos, self.buffer.current_pos);
        self.read_next();

        return TokenKind::Str(string_literal);
    }

    fn find_char_literal(&mut self) -> TokenKind {
        self.read_next();
        let ch_read = self.buffer.get_as_char(self.buffer.current_pos);
        self.read_next();
        return TokenKind::Char(ch_read);
    }

    pub fn next_lexed_token(&mut self) -> LexedToken {
        let current_pos = self.buffer.current_pos;
        let token = self.next_token();

        LexedToken{
            token: token,
            pos: current_pos
        }
    }

    #[allow(dead_code)]
    pub fn next_token(&mut self) -> TokenKind {
        // handle whitespace

        let mut incr_next_char = true;

        loop {
            match self.current_char {
                b' ' | b'\t' => {
                    self.read_next();
                }
                _ => {
                    break;
                }
            }
        }

        // handle comment:
        if self.current_char == b'#' {
            loop {
                self.read_next();
                match self.current_char {
                    b'\n' => {
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        let token = match self.current_char {
            // basic arithmetic and comparision operator:
            b'+' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'+' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SIncr)
                    }
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SPlusEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SPlus),
                };
                combined_token
            }

            b'-' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'-' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SDecr)
                    }
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SMinusEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SMinus),
                };
                combined_token
            }

            b'*' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SMinusEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SMul),
                };
                combined_token
            }

            b'/' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SModEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SDivEq),
                };
                combined_token
            }

            b'%' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SModEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SMod),
                };
                combined_token
            }

            // comparision operators
            b'<' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SLte)
                    }
                    _ => TokenKind::Operator(SymbolKind::SLt),
                };
                combined_token
            }

            b'>' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SGte)
                    }
                    _ => TokenKind::Operator(SymbolKind::SGt),
                };
                combined_token
            }

            // Equality operators:
            b'=' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SEeq)
                    }
                    b'>' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SImpl)
                    }
                    _ => TokenKind::Operator(SymbolKind::SEq),
                };
                combined_token
            }

            b'!' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SNe)
                    }
                    _ => TokenKind::Operator(SymbolKind::SExcl),
                };
                combined_token
            }

            // logical operators
            b'&' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'&' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SLAnd)
                    }
                    _ => TokenKind::Operator(SymbolKind::SAnd),
                };
                combined_token
            }

            b'|' => {
                let next_char = self.look_next_byte();
                let combined_token = match next_char {
                    b'|' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SLOr)
                    }
                    _ => TokenKind::Operator(SymbolKind::SOr),
                };
                combined_token
            }

            b'~' => TokenKind::Operator(SymbolKind::SNeg),

            // parenthesis and braces
            b'(' => TokenKind::Operator(SymbolKind::SLParen),

            b')' => TokenKind::Operator(SymbolKind::SRparen),

            b'[' => TokenKind::Operator(SymbolKind::SLBox),

            b']' => TokenKind::Operator(SymbolKind::SRBox),

            b'{' => TokenKind::Operator(SymbolKind::SLBrace),

            b'}' => TokenKind::Operator(SymbolKind::SRBrace),

            // Misc symbols
            b',' => TokenKind::Operator(SymbolKind::SComma),

            b';' => TokenKind::Operator(SymbolKind::SSemiColon),

            b':' => TokenKind::Operator(SymbolKind::SColon),

            b'.' => TokenKind::Operator(SymbolKind::SDot),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                incr_next_char = false;
                self.find_keyword_or_identifier()
            }

            b'0'..=b'9' => {
                incr_next_char = false;
                self.find_number_literal()
            }

            b'"' => {
                incr_next_char = false;
                self.find_string_literal()
            }

            b'\'' => {
                incr_next_char = false;
                self.find_char_literal()
            }

            b'\n' => {
                let next_char = self.look_next_byte();
                match next_char {
                    b'\n' => TokenKind::Empty,
                    _ => {
                        self.read_next();
                        return self.next_token();
                    }
                }
            }
            EOF_BYTE => TokenKind::EOF,
            _ => TokenKind::Invalid,
        };

        if incr_next_char {
            self.read_next();
        }

        return token;
    }

    #[allow(dead_code)]
    pub fn dump_tokens(&mut self, file_name: String) {
        let mut f_handle = fs::File::create(file_name).expect(
            "Failed to open file while dumping tokens."
        );

        let mut l_token: LexedToken;
        loop {
            l_token = self.next_lexed_token();
            f_handle.write_fmt(format_args!("{:?} at {}\n", l_token.token, l_token.pos)).expect(
                "Failed to write token into the file"
            );

            if l_token.token == TokenKind::EOF {
                break;
            }
        }
    }
}
