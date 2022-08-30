use std::collections::HashMap;

pub enum Kind {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

struct HashTable {
    pub type_map: HashMap<String, String>,
    pub kind_map: HashMap<String, Kind>,
    pub index_map: HashMap<String, usize>,
}

impl HashTable {
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
            kind_map: HashMap::new(),
            index_map: HashMap::new(),
        }
    }
}

pub struct SymbolTable {
    class_table: HashTable,
    subroutine_table: HashTable,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            class_table: HashTable::new(),
            subroutine_table: HashTable::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table.type_map.clear();
        self.subroutine_table.kind_map.clear();
        self.subroutine_table.index_map.clear();
    }

    pub fn define(&mut self, name: &str, type_: &str, kind: Kind) {
        let count = self.var_count(&kind);
        match kind {
            Kind::STATIC | Kind::FIELD => {
                self.class_table.type_map.insert(name.to_owned(), type_.to_owned());
                self.class_table.kind_map.insert(name.to_owned(), kind);
                self.class_table.index_map.insert(name.to_owned(), count);
            },
            Kind::ARG | Kind::VAR => {
                self.subroutine_table.type_map.insert(name.to_owned(), type_.to_owned());
                self.subroutine_table.kind_map.insert(name.to_owned(), kind);
                self.subroutine_table.index_map.insert(name.to_owned(), count);
            },
        }
    }

    pub fn var_count(&self, kind: &Kind) -> usize {
        match kind {
            Kind::STATIC | Kind::FIELD => self.class_table.index_map.len(),
            Kind::ARG | Kind::VAR => self.subroutine_table.index_map.len(),
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<&Kind> {
        let inner_output = self.subroutine_table.kind_map.get(&name.to_owned());
        let output = match inner_output {
            Some(..) => inner_output,
            None => self.class_table.kind_map.get(&name.to_owned()),
        };
        output
    }

    pub fn type_of(&self, name: &str) -> Option<&String> {
        let inner_output = self.subroutine_table.type_map.get(&name.to_owned());
        let output = match inner_output {
            Some(..) => inner_output,
            None => self.class_table.type_map.get(&name.to_owned()),
        };
        output
    }

    pub fn index_of(&self, name: &str) -> Option<&usize> {
        let inner_output = self.subroutine_table.index_map.get(&name.to_owned());
        let output = match inner_output {
            Some(..) => inner_output,
            None => self.class_table.index_map.get(&name.to_owned()),
        };
        output
    }
}