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
pub enum KeywordKind  {
    KInvalid  = 0,
    KIf       = 1,
    KElse     = 2,
    KWhile    = 3,
    KFor      = 4,
    KBreak    = 5,
    LContinue = 6,
    KConst    = 7,
    KVar      = 8,
    KNone     = 9,
    KFunc     = 10,
    KReturn   = 11,
    KTry      = 12,
    KCatch    = 13,
    KFinally  = 14,
    KRethrow  = 15,
    KThrow    = 16,
    KAs       = 17,
    KTrue     = 18,
    KFalse    = 19,
    KForEach  = 20,
    KIn       = 21,
    KIndexed  = 22,
    KPure     = 23,
}

#[allow(dead_code)]
pub enum TokenKind {
    Invalid (char),

    Integer (i128),
    Str (String),
    Float (f64),

    Identifier (String),
    Operator (String),
    Keyword (KeywordKind),

    Unknown (String)
}

#[allow(dead_code)]
pub struct Token {
    pub position: u32,
    pub token: TokenKind
}

/*
    LexerBuffer: This structure acts as a temp memory region that stores N characters/bytes
    read from the program source file at once. This buffer moves like a sliding window over the
    program source file, why we need this? Because we can reduce the number of reads on the file-system
    by buffering N characters at once instead of reading a single character per read. The end marker 
    i.e has_end_marker is true if we move over the last portion of the program that could not fill the
    buffer completely, this is done in order to avoid scanning over null region.
*/
#[allow(dead_code)]
pub struct LexerBuffer {
    pub buffer: Vec<char>,
    pub size: u32,
    pub has_end_marker: bool,
    pub ends_at: u32
}
