use std::collections::HashMap;
use std::rc::Rc;

use crate::compiler::errors::CompileError;
use crate::compiler::errors::CompilerErrorKind;
use crate::types::object::Object;

pub enum ScopeKind {
    Global,
    Local,
    Builtin,
    Free,
}

pub struct Symbol {
    pub name: String,
    pub pos: usize,
    pub is_const: bool,
    pub scope: ScopeKind,
}

pub type SymbolsMap = HashMap<String, Rc<Symbol>>;

pub struct SymbolTable {
    pub parent: Option<Box<SymbolTable>>,
    pub symbols: SymbolsMap,
    pub n_items: usize,
    pub free_symbols: Vec<Rc<Symbol>>,
    pub level: usize,
}

impl SymbolTable {
    pub fn create_new_root() -> SymbolTable {
        return SymbolTable {
            parent: None,
            symbols: HashMap::new(),
            n_items: 0,
            free_symbols: vec![],
            level: 0,
        };
    }

    pub fn create_new_child(parent: SymbolTable) -> SymbolTable {
        let new_level = parent.level + 1;
        let boxed_table = Some(Box::new(parent));
        return SymbolTable {
            parent: boxed_table,
            symbols: HashMap::new(),
            n_items: 0,
            free_symbols: vec![],
            level: new_level,
        };
    }

    pub fn get_symbol(&self, name: &String) -> Option<Rc<Symbol>> {
        let result = self.symbols.get(name).cloned();
        return result;
    }

    pub fn insert_new_symbol(&mut self, name: &str, is_const: bool) -> Rc<Symbol> {
        // select scope
        let current_scope = if self.level == 0 {
            ScopeKind::Global
        } else {
            ScopeKind::Local
        };

        // create a new symbol in this scope:
        let symbol = Symbol {
            name: name.to_string(),
            pos: self.n_items,
            is_const: is_const,
            scope: current_scope,
        };

        let ref_counted_symbol = Rc::new(symbol);
        // enter into the hash map:
        self.symbols
            .insert(name.to_string(), ref_counted_symbol.clone());
        self.n_items += 1;
        return ref_counted_symbol;
    }

    pub fn resolve(name: &str, symtab: &SymbolTable) -> Option<Rc<Symbol>> {
        let sym_key = name.to_string();
        let mut current_symtab = symtab;

        let mut symbol_res = current_symtab.get_symbol(&sym_key);
        while symbol_res.is_none() && current_symtab.level != 0 {
            current_symtab = current_symtab.parent.as_ref().unwrap();
            symbol_res = current_symtab.get_symbol(&sym_key);
        }

        return symbol_res;
    }
}

// constant pool: This stores all the literal objects that will be referenced by the VM
pub struct ConstantPool {
    pub objects: Vec<Rc<Object>>,
    pub size: usize,
}

impl ConstantPool {
    pub fn new() -> ConstantPool {
        return ConstantPool {
            objects: vec![],
            size: 0,
        };
    }

    pub fn get_object(&self, idx: usize) -> Option<Rc<Object>> {
        if idx < self.size {
            return Some(self.objects[idx].clone());
        }
        return None;
    }

    pub fn set_object(&mut self, object: Rc<Object>) {
        self.objects.push(object);
        self.size += 1;
    }
}
