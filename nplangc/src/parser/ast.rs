
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
    pub final_block: Option<BlockStatement>
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierType {
    pub name: String,
    pub t: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetType {
    pub identifier: IdentifierType,
    pub expression: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstType  {
    pub identifier: IdentifierType,
    pub expression: Option<ExpressionKind>
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexType {
    pub left: Box<ExpressionKind>,
    pub right: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaExpType {
    pub parameters: Vec<ExpressionKind>,
    pub expression: Box<StatementKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoopType {
    pub target: Box<ExpressionKind>,
    pub iter: Box<ExpressionKind>,
    pub loop_block: BlockStatement,
    pub else_block: Option<BlockStatement>
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoopType {
    pub target_expr: Box<ExpressionKind>,
    pub loop_block: BlockStatement,
    pub else_block: Option<BlockStatement>
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssertType {
    pub target_expr: Box<ExpressionKind>,
    pub fail_message: Box<ExpressionKind>
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bool(bool),
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
    Infix(InfixExpKind),
    Prefix(PrefixExpKind),
    Suffix(SuffixExpKind),
    Lambda(LambdaExpType),
    Boolean(bool)
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Empty,
    Break,
    Continue,
    Var(LetType),
    Const(ConstType),
    Return(ExpressionKind),
    Throw(ExpressionKind),
    Expression(ExpressionKind),
    TryCatch(TryCatchType),
    Function(FunctionType),
    For(ForLoopType),
    While(WhileLoopType),
    Assert(AssertType)
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<StatementKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<StatementKind> 
}
