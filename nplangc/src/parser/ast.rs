
use crate::lexer::*;
use crate::parser::exp::InfixExpKind;
use crate::parser::exp::PrefixExpKind;
use crate::parser::exp::SuffixExpKind;


#[allow(dead_code)]
pub struct Node {
    pub token: TokenKind,
    pub pos: usize, // position is stored for debugging and error reporting purposes.
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    pub name: String,
    pub parameters: Vec<ExpressionKind>,
    pub body: BlockStatement,
    pub return_type: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallType {
    pub function: Box<ExpressionKind>,
    pub arguments: Vec<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfElseType {
    pub condition: Box<ExpressionKind>,
    pub main_condition: BlockStatement,
    pub alternate_condition: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryCatchType {
    pub try_block: BlockStatement,
    pub exception_ident: IdentifierType,
    pub catch_block: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierType {
    pub name: String,
    pub t: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetType {
    pub identifier: IdentifierType,
    pub expression: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexType {
    pub left: Box<ExpressionKind>,
    pub right: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaExpType {
    pub parameters: Vec<ExpressionKind>,
    pub expression: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<ExpressionKind>),
    HashTable(Vec<(ExpressionKind, ExpressionKind)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Identifier(IdentifierType),
    Literal(LiteralKind),
    Index(IndexType),
    If(IfElseType),
    Function(FunctionType),
    Call(CallType),
    TryCatch(TryCatchType),
    Infix(InfixExpKind),
    Prefix(PrefixExpKind),
    Suffix(SuffixExpKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Empty,
    Let(LetType),
    Return(ExpressionKind),
    Throw(ExpressionKind),
    Expression(ExpressionKind),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    statements: Vec<StatementKind>,
}

pub type Program = BlockStatement;
