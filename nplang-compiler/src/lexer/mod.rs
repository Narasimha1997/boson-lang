pub mod types;

use types::SymbolKind;
use types::TokenKind;

#[allow(dead_code)]
pub struct StreamingLexer {
    file_name: String,
}

#[allow(dead_code)]
struct ProgramLexer {
    pub buffer: types::ProgramBuffer,
    pub current_char: u8,
}

impl ProgramLexer {
    #[allow(dead_code)]
    fn new_from_file(file_name: String) -> ProgramLexer {
        let mut lexer = ProgramLexer {
            buffer: types::ProgramBuffer::new_from_file(file_name),
            current_char: 0,
        };

        // read the first character from program buffer and return.
        lexer.read_next();
        return lexer;
    }

    #[allow(dead_code)]
    fn read_next(&mut self) {
        self.current_char = self.buffer.next_char();
    }

    #[allow(dead_code)]
    fn look_next_char(&mut self) -> u8 {
        return self.buffer.peek_next();
    }

    #[allow(dead_code)]
    fn is_next_char_equals(&mut self, ch: u8) -> bool {
        self.look_next_char() == ch
    }

    #[allow(dead_code)]
    pub fn next_token(&mut self) -> TokenKind {
        // handle whitespace
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
                        self.read_next();
                        break;
                    }
                    _ => continue,
                }
            }
        }

        let token = match self.current_char {
            // basic arithmetic and comparision operator:
            b'+' => {
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
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
                let next_char = self.look_next_char();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SEeq)
                    }
                    b'>' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SImpl)
                    }
                    _ => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SEq)
                    }
                };
                combined_token
            }

            b'!' => {
                let next_char = self.look_next_char();
                let combined_token = match next_char {
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SNe)
                    }
                    _ => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SExcl)
                    }
                };
                combined_token
            }

            // logical operators
            b'&' => {
                let next_char = self.look_next_char();
                let combined_token = match next_char {
                    b'&' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SLAnd)
                    }
                    _ => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SAnd)
                    }
                };
                combined_token
            }

            b'|' => {
                let next_char = self.look_next_char();
                let combined_token = match next_char {
                    b'|' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SLOr)
                    }
                    _ => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SOr)
                    }
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

            _ => TokenKind::Invalid,
        };

        return token;
    }
}
