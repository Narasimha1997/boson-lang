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
