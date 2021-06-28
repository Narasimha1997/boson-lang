use std;
use std::io::Read;
use std::vec::Vec;

#[allow(dead_code)]
pub const SYMBOLS: &'static [&'static str] = &[
    "invalid", "+", "-", "*", "/", "(", ")", "<", ">", "<=", ">=", ";", ",", "%", "!", "=", "==",
    "!=", "{", "}", "&", "|", "~", "&&", "||", "+=", "-=", "*=", "/=", "%=", "[", "]", "=>", ":",
    ".",
];

#[allow(dead_code)]
pub const KEYWORDS: &'static [&'static str] = &[
    "invalid", "if", "else", "while", "for", "break", "continue", "const", "var", "none", "func",
    "return", "try", "catch", "finally", "rethrow", "throw", "as", "true", "false", "foreach",
    "in", "indexed", "pure",
];

#[allow(dead_code)]
pub enum KeywordKind {
    KInvalid = 0,
    KIf = 1,
    KElse = 2,
    KWhile = 3,
    KFor = 4,
    KBreak = 5,
    LContinue = 6,
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
pub enum TokenKind {
    Invalid(char),

    Integer(i128),
    Str(String),
    Float(f64),

    Identifier(String),
    Operator(String),
    Keyword(KeywordKind),

    Unknown(String),
}

#[allow(dead_code)]
pub struct Token {
    pub position: u32,
    pub token: TokenKind,
}

/*
    LexerBuffer: This structure acts as a temp memory region that stores N characters/bytes
    read from the program source file at once. This buffer moves like a sliding window over the
    program source file, why we need this? Because we can reduce the number of reads on the file-system
    by buffering N characters at once instead of reading a single character per read.
*/
#[allow(dead_code)]
pub struct LexerBuffer {
    pub buffer: Vec<u8>,
    pub size: usize,
}

#[allow(dead_code)]
pub struct ChunkedReader {
    pub chunk_size: usize,
    pub file_handle: std::fs::File,
    pub is_end_reached: bool,
}

impl ChunkedReader {
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
pub fn new_chunked_reader(file_name: String, chunk_size: usize) -> ChunkedReader {
    let chunked_reader = ChunkedReader {
        chunk_size: chunk_size,
        file_handle: std::fs::File::open(&file_name).expect("File not found!"),
        is_end_reached: false,
    };

    return chunked_reader;
}

#[allow(dead_code)]
pub struct ProgramBuffer {
    pub buffer: Vec<u8>,
    pub current_pos: usize,
    pub next_pos: usize,
    pub buffer_size: usize
}

impl ProgramBuffer {
    #[allow(dead_code)]
    pub fn peek_next(&mut self) -> u8 {
        if self.next_pos >= self.buffer_size {
            return 0x00;
        } else {
            return self.buffer[self.next_pos]
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
}

#[allow(dead_code)]
pub fn new_program_buffer(buffer: Vec<u8>) -> ProgramBuffer {
    let buffer_length = buffer.len();
    ProgramBuffer{
        buffer: buffer,
        current_pos: 0,
        next_pos: 0,
        buffer_size: buffer_length
    }
}