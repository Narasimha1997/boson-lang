use std;
use std::fs;
use std::io::Read;
use std::vec::Vec;


pub const EOF_BYTE:u8 = 0x00;

#[allow(dead_code)]
pub const SYMBOLS: &'static [&'static str] = &[
    "invalid", "+", "-", "*", "/", "(", ")", "<", ">", "<=", ">=", ";", ",", "%", "!", "=", "==",
    "!=", "{", "}", "&", "|", "~", "&&", "||", "+=", "-=", "++", "--", "*=", "/=", "%=", "[", "]", "=>", ":",
    ".",
];

#[allow(dead_code)]
pub const KEYWORDS: &'static [&'static str] = &[
    "invalid", "if", "else", "while", "for", "break", "continue", "const", "var", "none", "func",
    "return", "try", "catch", "finally", "rethrow", "throw", "as", "true", "false", "foreach",
    "in", "indexed", "pure",
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
    SNeg = 22,
    SLAnd = 23,
    SLOr = 24,
    SPlusEq = 25,
    SMinusEq = 26,
    SIncr = 27,
    SDecr = 28,
    SMulEq = 29,
    SDivEq = 30,
    SModEq = 31,
    SLBox = 32,
    SRBox = 33,
    SImpl = 34,
    SColon = 35,
    SDot = 36
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
        let n_read = self
            .file_handle
            .by_ref()
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

        file_handle
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
