use crate::lexer::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    UnexpectedToken,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub t: ParserErrorKind,
    pub pos: usize,
    pub error_token: TokenKind,
}

impl ParserError {
    pub fn new(t: ParserErrorKind, msg: String, token: LexedToken) -> ParserError {
        ParserError {
            message: msg,
            t: t,
            pos: token.pos,
            error_token: token.token,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type={:?} Position={} Reason={} Error Token={:?}",
            self.t, self.pos, self.message, self.error_token
        )
    }
}

pub type ParserErrors = Vec<ParserError>;
