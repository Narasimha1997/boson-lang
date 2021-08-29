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
pub struct ShellType {
    pub shell: Box<ExpressionKind>,
    pub is_raw: bool,
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
    pub is_thread: bool,
    pub is_async: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfElseType {
    pub condition: Box<ExpressionKind>,
    pub main_block: BlockStatement,
    pub alternate_block: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryCatchType {
    pub try_block: BlockStatement,
    pub exception_ident: Box<ExpressionKind>,
    pub catch_block: BlockStatement,
    pub final_block: Option<BlockStatement>,
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
pub struct ConstType {
    pub identifier: IdentifierType,
    pub expression: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexType {
    pub expression_left: Box<ExpressionKind>,
    pub index: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaExpType {
    pub parameters: Vec<ExpressionKind>,
    pub expression: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoopType {
    pub target: Box<ExpressionKind>,
    pub iter: Box<ExpressionKind>,
    pub loop_block: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoopType {
    pub target_expr: Box<ExpressionKind>,
    pub loop_block: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssertType {
    pub target_expr: Box<ExpressionKind>,
    pub fail_expr: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayType {
    pub array_values: Vec<ExpressionKind>,
    pub length: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnType {
    pub expression: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThrowType {
    pub expression: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForEachType {
    pub iterator_exp: Box<ExpressionKind>,
    pub index: Box<ExpressionKind>,
    pub element: Box<ExpressionKind>,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixType {
    pub prefix: PrefixExpKind,
    pub expression: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuffixType {
    pub suffix: SuffixExpKind,
    pub expression: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HashTableType {
    pub pairs: Vec<(ExpressionKind, ExpressionKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixType {
    pub infix: InfixExpKind,
    pub expression_left: Box<ExpressionKind>,
    pub expression_right: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bool(bool),
    Array(ArrayType),
    HashTable(HashTableType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Noval,
    Identifier(IdentifierType),
    Literal(LiteralKind),
    Index(IndexType),
    Call(CallType),
    Infix(InfixType),
    Prefix(PrefixType),
    Suffix(SuffixType),
    Lambda(LambdaExpType),
    Boolean(bool),
    Shell(ShellType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Empty,
    Break,
    Continue,
    Var(LetType),
    Const(ConstType),
    Return(ReturnType),
    Throw(ThrowType),
    Expression(ExpressionKind),
    TryCatch(TryCatchType),
    Function(FunctionType),
    For(ForLoopType),
    While(WhileLoopType),
    Assert(AssertType),
    If(IfElseType),
    ForEach(ForEachType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<StatementKind>,
    pub pos: Vec<usize>,
}

pub type Program = BlockStatement;
