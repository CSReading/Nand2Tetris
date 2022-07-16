// SymbolTable module
use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn init() -> SymbolTable {
        let mut st: SymbolTable = SymbolTable {
            table: HashMap::new(),
        };
        st.table.insert("SP".to_string(), 0);
        st.table.insert("LCL".to_string(), 1);
        st.table.insert("ARG".to_string(), 2);
        st.table.insert("THIS".to_string(), 3);
        st.table.insert("THAT".to_string(), 4);
        for i in 0..16 {
            st.table.insert(format!("R{}", i), i as u16);
        }
        st.table.insert("SCREEN".to_string(), 16384);
        st.table.insert("KBD".to_string(), 24576);
        st
    }

    pub fn add_entry(&mut self, symbol: &str, address: u16) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        match self.table.get(&symbol.to_string()) {
            None => false,
            _ => true,
        }
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        *self.table.get(&symbol.to_string()).unwrap()
    }
}
