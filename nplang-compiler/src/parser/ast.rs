mod exp;

use exp::InfixExpKind;
use exp::PrefixExpKind;
use exp::SuffixExpKind;
use lexer::types::TokenKind;


#[allow(dead_code)]
pub struct Node {
    token: TokenKind,
    pos: usize          // position is stored for debugging and error reporting purposes.
};

pub struct FunctionType {
    name: String,
    parameters: Vec<ExpressionKind>,
    body: BlockStatement,
    return_type: Option<String>
}

pub struct CallType {
    function: Box<ExpressionKind>,
    arguments: Vec<ExpressionKind>
}

pub struct IfElseType {
    condition: Box<ExpressionKind>,
    main_condition: BlockStatement,
    alternate_condition: Option<BlockStatement>
}

pub struct TryCatchType {
    try_block: BlockStatement,
    exception_ident: IdentifierType,
    catch_block: BlockStatement
}

pub struct IdentifierType {
    name: String,
    type: Option<String>
}

pub struct LetType {
    identifier: IdentifierKind,
    expression: ExpressionKind
}

pub struct IndexType {
    left: Box<ExpressionKind>,
    right: Box<ExpressionKind>
}

pub struct LambdaExpType {
    parameters: Vec<ExpressionKind>,
    expression: StatementKind; 
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<ExpressionKind>),
    HashTable<Vec<(ExpressionKind, ExpressionKind)>>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Identifier (IdentifierType),
    Literal (LiteralKind),
    Index (IndexType),
    If  (IfElseType),
    Function (FunctionType),
    Call (CallType),
    TryCatch (TryCatchType),
    Infix (InfixExpKind),
    Prefix (PrefixExpKind),
    Suffix (SuffixExpKind)
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Empty,
    Let (LetKind),
    Return (ExpressionKind),
    Throw (ExpressionKind),
    Expression (ExpressionKind)
};

pub struct BlockStatement {
    Statements(Vec<StatementKind>)
}

pub struct Program {
    Statements(Vec<StatementKind>)
}