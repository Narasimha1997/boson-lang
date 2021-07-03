

// contains definitions for parsing expressions, pre, post decrement
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

pub enum SuffixExpKind {
    PostIncrement,
    PostDecrement
}

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


