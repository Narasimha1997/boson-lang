use lexer::types::TokenKind;

#[allow(dead_code)]
pub struct Node {
    token: TokenKind,
    pos: usize          // position is stored for debugging and error reporting purposes.
};

pub struct FunctionType {

}

pub struct CallType {

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

pub struct Let {
    Identifier (IdentifierKind)
    Expression (ExpressionKind)
}


pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<ExpressionKind>),
    HashTable<Vec<(ExpressionKind, ExpressionKind)>>
}

pub enum ExpressionKind {

}

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