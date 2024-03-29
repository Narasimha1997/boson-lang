use crate::types::builtins;
use crate::types::object;
use std::collections::HashMap;
use std::rc::Rc;

use builtins::BuiltinKind;
use object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Global,
    Local,
    Builtin,
    Free,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub pos: usize,
    pub is_const: bool,
    pub scope: ScopeKind,
}

pub type SymbolsMap = HashMap<String, Rc<Symbol>>;

#[derive(Debug, Clone)]
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

    pub fn insert_free_symbol(&mut self, symbol: &Rc<Symbol>) -> Rc<Symbol> {
        self.free_symbols.push(Rc::clone(symbol));

        let new_symbol = Symbol {
            name: symbol.name.clone(),
            pos: self.free_symbols.len() - 1,
            is_const: symbol.is_const,
            scope: ScopeKind::Free,
        };

        let ref_c_new_symbol = Rc::new(new_symbol);

        self.symbols
            .insert(symbol.name.clone(), Rc::clone(&ref_c_new_symbol));
        return ref_c_new_symbol;
    }

    pub fn insert_builtins(&mut self) {
        let names = BuiltinKind::get_names();
        for name in names {
            let builtin_symbol = Symbol {
                name: name.clone(),
                pos: self.n_items,
                is_const: true,
                scope: ScopeKind::Builtin,
            };

            self.symbols.insert(name.clone(), Rc::new(builtin_symbol));
            self.n_items += 1;
        }
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
            .insert(name.to_string(), Rc::clone(&ref_counted_symbol));
        self.n_items += 1;
        return ref_counted_symbol;
    }

    pub fn resolve_symbol(&mut self, name: &str) -> Option<Rc<Symbol>> {
        let result = self.__resolve(name);

        if result.is_some() {
            let (unwrapped_sym, is_parent) = result.unwrap();

            if !is_parent {
                return Some(Rc::clone(&unwrapped_sym));
            }

            match unwrapped_sym.scope {
                ScopeKind::Global | ScopeKind::Builtin => return Some(Rc::clone(&unwrapped_sym)),
                ScopeKind::Local | ScopeKind::Free => {
                    let free = self.insert_free_symbol(&unwrapped_sym);
                    return Some(free);
                }
            }
        }

        return None;
    }

    fn __resolve(&self, name: &str) -> Option<(Rc<Symbol>, bool)> {
        let sym_key = name.to_string();
        let mut current_symtab = self;

        let mut is_parent_level = false;

        let mut symbol_res = current_symtab.get_symbol(&sym_key);
        while symbol_res.is_none() && current_symtab.level != 0 {
            current_symtab = current_symtab.parent.as_ref().unwrap();
            symbol_res = current_symtab.get_symbol(&sym_key);
            is_parent_level = true;
        }

        if symbol_res.is_some() {
            return Some((symbol_res.unwrap(), is_parent_level));
        }

        return None;
    }

    pub fn get_free_symbols(&self) -> Vec<Rc<Symbol>> {
        return self.free_symbols.clone();
    }
}

// constant pool: This stores all the literal objects that will be referenced by the VM
#[derive(Debug, Clone)]
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

    pub fn set_object(&mut self, object: Rc<Object>) -> usize {
        self.objects.push(object);
        self.size += 1;
        return self.size - 1;
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}
