use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::parser::ast;
use crate::types::context;
use context::Context;

#[derive(Clone, Debug)]
pub struct Subroutine {
    pub parameters: Vec<ast::ExpressionKind>,
    pub body: ast::BlockStatement,
    pub name: String,
    pub context: Rc<RefCell<Context>>,
}

impl Subroutine {
    fn describe(&self) -> String {
        return format!("Function<{}>", self.name);
    }
}

/*
    Functions can be compared by their names alone.
*/
impl PartialEq for Subroutine {
    fn eq(&self, other: &Subroutine) -> bool {
        other.name == self.name
    }
}

impl Eq for Subroutine {}

/*
    Hash for subroutine:
        When you hash a sub-routine, they are hashed by their names.
        For example, if you have a function x, then we consider the string
        "x" for hashing, because of this, you can use functions as keys for
        hashing.
*/
impl Hash for Subroutine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Subroutine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fn_desc = self.describe();
        write!(f, "{}", fn_desc)
    }
}
