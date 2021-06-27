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
pub enum TokenKind {
    Invalid (char),
    I8 (i8),
    I16 (i16),
    I32 (i32),
    I64 (i64),

}
