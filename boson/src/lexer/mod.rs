use std::fs;
use std::io::Read;
use std::io::Write;
use std::vec::Vec;

pub const EOF_BYTE: u8 = 0x00;

#[allow(dead_code)]
pub const SYMBOLS: &'static [&'static str] = &[
    "invalid", "+", "-", "*", "/", "(", ")", "<", ">", "<=", ">=", ";", ",", "%", "!", "=", "==",
    "!=", "{", "}", "&", "|", "~", "&&", "||", "+=", "-=", "++", "--", "*=", "/=", "%=", "[", "]",
    "=>", ":", ".",
];

#[allow(dead_code)]
pub const KEYWORDS: &'static [&'static str] = &[
    "invalid", "if", "else", "while", "for", "break", "continue", "const", "var", "none", "func",
    "return", "try", "catch", "finally", "rethrow", "throw", "as", "true", "false", "foreach",
    "in", "indexed", "pure", "lambda", "assert",
];

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    SInvalid = 0,
    SPlus = 1,
    SMinus = 2,
    SMul = 3,
    SDiv = 4,
    SLParen = 5,
    SRparen = 6,
    SLt = 7,
    SGt = 8,
    SLte = 9,
    SGte = 10,
    SSemiColon = 11,
    SComma = 12,
    SMod = 13,
    SExcl = 14,
    SEq = 15,
    SEeq = 16,
    SNe = 17,
    SLBrace = 18,
    SRBrace = 19,
    SAnd = 20,
    SOr = 21,
    SAndEq = 22,
    SOrEq = 23,
    SNeg = 24,
    SLAnd = 25,
    SLOr = 26,
    SPlusEq = 27,
    SMinusEq = 28,
    SIncr = 29,
    SDecr = 30,
    SMulEq = 31,
    SDivEq = 32,
    SModEq = 33,
    SLBox = 34,
    SRBox = 35,
    SImpl = 36,
    SColon = 37,
    SDot = 38,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    KInvalid = 0,
    KIf = 1,
    KElse = 2,
    KWhile = 3,
    KFor = 4,
    KBreak = 5,
    KContinue = 6,
    KConst = 7,
    KVar = 8,
    KNone = 9,
    KFunc = 10,
    KReturn = 11,
    KTry = 12,
    KCatch = 13,
    KFinally = 14,
    KRethrow = 15,
    KThrow = 16,
    KAs = 17,
    KTrue = 18,
    KFalse = 19,
    KForEach = 20,
    KIn = 21,
    KIndexed = 22,
    KPure = 23,
    KLambda = 24,
    KAssert = 25,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Invalid,
    Empty,
    EOF,

    Integer(i64),
    Str(String),
    Float(f64),
    Char(char),

    Identifier(String),
    Operator(SymbolKind),
    Keyword(KeywordKind),

    Unknown(String),
}

/*
    the return value from Lexer
*/
#[derive(Debug, Clone)]
pub struct LexedToken {
    pub token: TokenKind,
    pub pos: usize,
}

/*
    LexerBuffer: This structure acts as a temp memory region that stores N characters/bytes
    read from the program source file at once. This buffer moves like a sliding window over the
    program source file, why we need this? Because we can reduce the number of reads on the file-system
    by buffering N characters at once instead of reading a single character per read. More work will take place
    on this in future.
*/
#[allow(dead_code)]
pub struct LexerBuffer {
    pub buffer: Vec<u8>,
    pub size: usize,
}

#[allow(dead_code)]
pub struct ChunkedBuffer {
    pub chunk_size: usize,
    pub file_handle: fs::File,
    pub is_end_reached: bool,
}

impl ChunkedBuffer {
    #[allow(dead_code)]
    pub fn new(file_name: String, chunk_size: usize) -> ChunkedBuffer {
        ChunkedBuffer {
            chunk_size: chunk_size,
            file_handle: fs::File::open(&file_name).expect("File not found!"),
            is_end_reached: false,
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<LexerBuffer> {
        if self.is_end_reached {
            return None;
        }

        let mut chunk = Vec::with_capacity(self.chunk_size);
        let n_read = (&mut self.file_handle)
            .take(self.chunk_size as u64)
            .read_to_end(&mut chunk)
            .expect("Failed to read file");
        if self.chunk_size > n_read {
            self.is_end_reached = true;
        }

        return Some(LexerBuffer {
            buffer: chunk,
            size: n_read,
        });
    }

    #[allow(dead_code)]
    pub fn is_end(&mut self) -> bool {
        return self.is_end_reached;
    }
}

#[allow(dead_code)]
pub struct ProgramBuffer {
    pub buffer: Vec<u8>,
    pub current_pos: usize,
    pub next_pos: usize,
    pub buffer_size: usize,
}

impl ProgramBuffer {
    #[allow(dead_code)]
    pub fn new_from_buffer(buffer: Vec<u8>) -> ProgramBuffer {
        let l = buffer.len();
        ProgramBuffer {
            buffer: buffer,
            current_pos: 0,
            next_pos: 0,
            buffer_size: l,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_file(file_name: String) -> ProgramBuffer {
        let mut file_handle =
            fs::File::open(&file_name).expect("Unable to read the source file, file not found.");
        let file_metadata = fs::metadata(&file_name).expect("Unable to get metadata of the file.");
        let f_len = file_metadata.len() as usize;
        let mut buffer = vec![0; f_len];

        (&mut file_handle)
            .read(&mut buffer)
            .expect("Failed to read file into buffer");

        ProgramBuffer {
            buffer: buffer,
            current_pos: 0,
            next_pos: 0,
            buffer_size: f_len,
        }
    }

    #[allow(dead_code)]
    pub fn peek_next(&mut self) -> u8 {
        if self.next_pos >= self.buffer_size {
            return 0x00;
        } else {
            return self.buffer[self.next_pos];
        }
    }

    #[allow(dead_code)]
    pub fn next_char(&mut self) -> u8 {
        if self.next_pos >= self.buffer_size {
            return 0x00;
        } else {
            let current_char = self.buffer[self.next_pos];
            self.current_pos = self.next_pos;
            self.next_pos = self.next_pos + 1;
            return current_char;
        }
    }

    pub fn get_as_i64(&mut self, start: usize, end: usize) -> i64 {
        let sub_buffer = self.buffer[start..end].to_vec();
        String::from_utf8(sub_buffer).unwrap().parse().unwrap()
    }

    pub fn get_as_f64(&mut self, start: usize, end: usize) -> f64 {
        let sub_buffer = self.buffer[start..end].to_vec();
        String::from_utf8(sub_buffer).unwrap().parse().unwrap()
    }

    pub fn get_as_string(&mut self, start: usize, end: usize) -> String {
        let sub_buffer = self.buffer[start..end].to_vec();
        String::from_utf8(sub_buffer).unwrap()
    }

    pub fn get_as_char(&mut self, pos: usize) -> char {
        self.buffer[pos] as char
    }
}

#[allow(dead_code)]
pub struct StreamingLexer {
    file_name: String,
}

#[allow(dead_code)]
pub struct ProgramLexer {
    pub buffer: ProgramBuffer,
    pub current_char: u8,
}

impl ProgramLexer {
    #[allow(dead_code)]
    pub fn new_from_file(file_name: String) -> ProgramLexer {
        let mut lexer = ProgramLexer {
            buffer: ProgramBuffer::new_from_file(file_name),
            current_char: 0,
        };

        lexer.append_eof_newline();
        // read the first character from program buffer and return.
        lexer.read_next();
        return lexer;
    }

    #[allow(dead_code)]
    pub fn new_from_buffer(buffer: Vec<u8>) -> ProgramLexer {
        let mut lexer = ProgramLexer {
            buffer: ProgramBuffer::new_from_buffer(buffer),
            current_char: 0,
        };

        // read the first character from program buffer and return.
        lexer.append_eof_newline();
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
            "assert" => TokenKind::Keyword(KeywordKind::KAssert),

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

    fn append_eof_newline(&mut self) {
        if self.buffer.buffer[self.buffer.buffer_size - 1] != b'\n' {
            self.buffer.buffer.push(b'\n');
            self.buffer.buffer_size += 1;
        }
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

        LexedToken {
            token: token,
            pos: current_pos,
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
                        TokenKind::Operator(SymbolKind::SDivEq)
                    }
                    _ => TokenKind::Operator(SymbolKind::SDiv),
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
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SAndEq)
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
                    b'=' => {
                        self.read_next();
                        TokenKind::Operator(SymbolKind::SOrEq)
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
                incr_next_char = true;
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
        let mut f_handle =
            fs::File::create(file_name).expect("Failed to open file while dumping tokens.");

        let mut l_token: LexedToken;
        loop {
            l_token = self.next_lexed_token();
            (&mut f_handle)
                .write_fmt(format_args!("{:?} at {}\n", l_token.token, l_token.pos))
                .expect("Failed to write token into the file");

            if l_token.token == TokenKind::EOF {
                break;
            }
        }
    }
    pub fn reset(&mut self) {
        self.buffer.current_pos = 0;
    }
}

pub struct LexerAPI {
    pub lexer: ProgramLexer,
    pub current_token: LexedToken,
    pub next_token: LexedToken,
}

impl LexerAPI {
    pub fn new_from_file(file_name: String) -> LexerAPI {
        let mut lexer = ProgramLexer::new_from_file(file_name);
        let tok1 = lexer.next_lexed_token().clone();
        let tok2 = lexer.next_lexed_token().clone();
        LexerAPI {
            lexer: lexer,
            current_token: tok1,
            next_token: tok2,
        }
    }

    pub fn new_from_buffer(buffer: Vec<u8>) -> LexerAPI {
        let mut lexer = ProgramLexer::new_from_buffer(buffer);
        let tok1 = lexer.next_lexed_token().clone();
        let tok2 = lexer.next_lexed_token().clone();
        LexerAPI {
            lexer: lexer,
            current_token: tok1,
            next_token: tok2,
        }
    }

    pub fn iterate(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_lexed_token().clone();
    }

    pub fn get_tokens(&mut self) -> (LexedToken, LexedToken) {
        (self.current_token.clone(), self.next_token.clone())
    }

    pub fn get_current_token(&mut self) -> LexedToken {
        self.current_token.clone()
    }

    pub fn get_next_token(&mut self) -> LexedToken {
        self.next_token.clone()
    }

    pub fn tokens_are_equal(&mut self, token1: &TokenKind, token2: TokenKind) -> bool {
        token1 == (&token2)
    }

    pub fn keywords_are_equal(&mut self, kw1: &KeywordKind, kw2: KeywordKind) -> bool {
        kw1 == (&kw2)
    }

    pub fn symbols_are_equal(&mut self, sym1: &SymbolKind, sym2: SymbolKind) -> bool {
        sym1 == (&sym2)
    }

    pub fn get_line_by_pos(&mut self, pos: usize) -> (String, usize, usize) {
        let mut back_iter = pos;
        let mut front_iter = pos;

        let mut read_char: u8;
        let back_pos: usize;
        let front_pos: usize;

        if pos >= self.lexer.buffer.buffer_size {
            return (String::from(""), 0, 0);
        }

        loop {
            if back_iter == 0 {
                back_pos = 0;
                break;
            }

            read_char = self.lexer.buffer.buffer[back_iter];
            if read_char != b'\n' {
                back_iter -= 1;
                continue;
            } else {
                back_pos = back_iter;
                break;
            }
        }

        loop {
            if front_iter >= self.lexer.buffer.buffer_size {
                front_pos = self.lexer.buffer.buffer_size - 1;
                break;
            }

            read_char = self.lexer.buffer.buffer[front_iter];
            if read_char != b'\n' {
                front_iter += 1;
                continue;
            } else {
                front_pos = front_iter;
                break;
            }
        }

        (
            self.lexer.buffer.get_as_string(back_pos, front_pos),
            back_pos,
            front_iter,
        )
    }
}
