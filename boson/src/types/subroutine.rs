use std::fmt;
use std::hash::{Hash, Hasher};

use crate::compiler::CompiledInstructions;

#[derive(Clone, Debug, PartialOrd)]
pub struct Subroutine {
    pub name: String,
    pub bytecode: CompiledInstructions,
    pub num_locals: usize,
    pub num_parameters: usize,
    pub is_local_scope: bool,
}

impl Subroutine {
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_bytecode(&self) -> &CompiledInstructions {
        return &self.bytecode;
    }

    pub fn get_n_locals(&self) -> usize {
        return self.num_locals;
    }

    pub fn gen_n_parameters(&self) -> usize {
        return self.num_parameters;
    }

    pub fn describe(&self) -> String {
        return format!("Function<{}>", self.name);
    }

    pub fn get_scope(&self) -> bool {
        return self.is_local_scope;
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
