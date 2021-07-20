use crate::types::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

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
            let unwrapped_sym = result.unwrap();
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

    fn __resolve(&self, name: &str) -> Option<Rc<Symbol>> {
        let sym_key = name.to_string();
        let mut current_symtab = self;

        let mut symbol_res = current_symtab.get_symbol(&sym_key);
        while symbol_res.is_none() && current_symtab.level != 0 {
            current_symtab = current_symtab.parent.as_ref().unwrap();
            symbol_res = current_symtab.get_symbol(&sym_key);
        }
        return symbol_res;
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
