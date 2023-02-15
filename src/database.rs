pub mod database {
    use std::collections::HashMap;
    // use std::ops::Index;
    use crate::qrtlib::{QueryResult, Table, TableField};

    pub struct Database {
        name: String,
        namespaces: Vec<String>,
        tables: Vec<Table>,
        table_indexes: HashMap<String, usize>,
        tindex: u64,
        nsIndex: u64,
    }

    impl Database {
        pub fn new(_name: &str) -> Database {
            let namespaces: Vec<String> = Vec::new();
            let tables: Vec<Table> = Vec::new();
            let table_indexes: HashMap<String, usize> = HashMap::new();
            let name = String::from(_name);
            return Database {
                name,
                namespaces,
                tables,
                table_indexes,
                tindex: 0,
                nsIndex: 0,
            };
        }
        pub fn dbname(&self) -> String {
            return self.name.clone();
        }
        pub fn add_namespace(&mut self, name: &str) {
            let namespace = String::from(name);
            if self.namespaces.contains(&namespace) {
                return;
            }
            self.namespaces.push(namespace);
        }
        pub fn remove_namespace(&mut self, name: &str) {
            let namespace = String::from(name);
        }
        fn create_table(
            &mut self,
            name: &str,
            fields: Vec<TableField>,
            namespace: &str,
        ) -> QueryResult {
            if !self.namespaces.contains(&String::from(namespace)) {
                println!("namespace not found");
                return QueryResult::FAILURE;
            }
            let table = Table::new(name, fields, namespace);
            self.tables.push(table);
            self.tindex += 1;
            self.table_indexes
                .insert(String::from(name), self.tindex as usize);
            return QueryResult::SUCCESS;
        }
        fn remove_table(&mut self, name: &str) {
            //get index
            //swap remove by index
            //since swap remove changes indexes
            //update table_indexes
            // all these string froms are not probably great idea
            let index = self.table_indexes.get(&String::from(name)).unwrap();
            //get name of the last element in tables for this to work
            let tlen = self.tables.len();
            let tname = self.tables[tlen - 1].tname();
            self.tables.swap_remove(*index as usize);
            self.table_indexes.insert(tname, *index);
        }
    }
}
