
// contains definitions for parsing expressions, pre, post decrement
#[derive(Debug, PartialEq, Clone)]
pub enum PrefixExpKind {
    PreIncrement,
    PreDecrement,
    PlusEq,
    MinusEq,
    ModEq,
    MulEq,
    DivEq,
    AndEq,
    OrEq,
    Not
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixExpKind {
    PostIncrement,
    PostDecrement
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
    Not,
    Equal,
    NotEqual,
    GreaterThanEqual,
    GreaterThan,
    LesserThanEqual,
    LesserThan
}


