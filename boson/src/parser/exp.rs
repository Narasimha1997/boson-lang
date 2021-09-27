use crate::lexer::SymbolKind;

// contains definitions for parsing expressions, pre, post decrement
#[derive(Debug, PartialEq, Clone)]
pub enum PrefixExpKind {
    PreIncrement,
    PreDecrement,
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixExpKind {
    PostIncrement,
    PostDecrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixExpKind {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Equal,
    EEqual,
    NotEqual,
    GreaterThanEqual,
    GreaterThan,
    LesserThanEqual,
    LesserThan,
    LogicalOr,
    LogicalAnd,
    PlusEq,
    MinusEq,
    MulEq,
    DivEq,
    AndEq,
    OrEq,
    ModEq
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ExpOrder {
    Zero,
    Equals,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseAnd,
    Equality,
    LessGreater,
    AddSub,
    DivMulMod,
    Unary,
    IncrDecr,
    Call,
    Index,
    Dot,
}

#[allow(dead_code)]
pub fn get_eval_order(sym: &SymbolKind) -> ExpOrder {
    match sym {
        SymbolKind::SEq
        | SymbolKind::SOrEq
        | SymbolKind::SAndEq
        | SymbolKind::SPlusEq
        | SymbolKind::SMinusEq
        | SymbolKind::SMulEq
        | SymbolKind::SDivEq
        | SymbolKind::SModEq => return ExpOrder::Equals,

        SymbolKind::SLOr => return ExpOrder::LogicalOr,
        SymbolKind::SLAnd => return ExpOrder::LogicalAnd,

        SymbolKind::SOr => return ExpOrder::BitwiseOr,
        SymbolKind::SAnd => return ExpOrder::BitwiseAnd,

        SymbolKind::SEeq | SymbolKind::SNe => return ExpOrder::Equality,

        SymbolKind::SGt | SymbolKind::SLt | SymbolKind::SLte | SymbolKind::SGte => {
            return ExpOrder::LessGreater
        }

        SymbolKind::SPlus | SymbolKind::SMinus => return ExpOrder::AddSub,

        SymbolKind::SMul | SymbolKind::SDiv | SymbolKind::SMod => return ExpOrder::DivMulMod,

        SymbolKind::SNeg | SymbolKind::SExcl => return ExpOrder::Unary,

        SymbolKind::SIncr | SymbolKind::SDecr => return ExpOrder::IncrDecr,

        SymbolKind::SLParen => return ExpOrder::Call,
        SymbolKind::SLBox => return ExpOrder::Index,
        SymbolKind::SDot => return ExpOrder::Dot,
        _ => return ExpOrder::Zero,
    }
}
