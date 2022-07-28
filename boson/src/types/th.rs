use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone, PartialOrd)]
pub struct ThreadBlock {
    // handle_id is a unique 64 bit ID for a thread.
    // handle_id will be assiociated with the thread handle stored as a hash-map
    // inside the current VM scope.
    pub handle_id: u64,
    pub name: String,
}

impl ThreadBlock {
    pub fn new(handle_id: u64, name: String) -> ThreadBlock {
        return ThreadBlock { handle_id, name };
    }

    pub fn describe(&self) -> String {
        return format!("Thread(func={}, id={})", self.name, self.handle_id);
    }
}

impl PartialEq for ThreadBlock {
    fn eq(&self, other: &ThreadBlock) -> bool {
        self.name == other.name
    }
}

impl Hash for ThreadBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
