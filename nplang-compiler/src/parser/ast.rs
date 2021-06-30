use lexer::types::TokenKind;

#[allow(dead_code)]
pub struct Node {
    token: TokenKind,
    pos: usize          // position is stored for debugging and error reporting purposes.
};

pub enum StatementKind {

};

pub enum BlockStatementKind {
    Statements(Vec<StatementKind>)
}

pub enum Program {
    Statements(Vec<StatementKind>)
}