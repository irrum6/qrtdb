pub mod database {
    use std::collections::HashMap;
    // use std::ops::Index;
    use crate::{
        qrtlib::{QueryResult, Table, TableField},
        statements::statements::Statement,
    };

    pub struct Database {
        name: String,
        namespaces: Vec<String>,
        tables: Vec<Table>,
        table_indexes: HashMap<String, u64>,
        tindex: u64,
        nsIndex: u64,
    }

    impl Database {
        pub fn new(_name: &str) -> Database {
            let namespaces: Vec<String> = Vec::new();
            let tables: Vec<Table> = Vec::new();
            let table_indexes: HashMap<String, u64> = HashMap::new();
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
        pub fn compose_table_name(namespace: &str, name: &str) -> String {
            let mut tname = String::from(namespace);
            tname.push_str("_");
            tname.push_str(name);
            return tname;
        }
        pub fn create_table(&mut self, name: &str, fields: Vec<TableField>, namespace: &str) -> QueryResult {
            if !self.namespaces.contains(&String::from(namespace)) {
                println!("namespace not found");
                return QueryResult::FAILURE;
            }

            let full_table_name = Database::compose_table_name(namespace, name);
            let table = Table::new(full_table_name.as_str(), fields, namespace);
            self.tables.push(table);
            self.tindex += 1;
            self.table_indexes.insert(String::from(name), self.tindex);
            return QueryResult::SUCCESS;
        }
        pub fn remove_table(&mut self, name: &str) {
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


        pub fn ls_tables(&self) {

        }
        fn get_table_index(&self, tablename: String) -> u64 {
            let index = self.table_indexes.get(&tablename).unwrap();
            return *index;
        }
        pub fn insert(&mut self, tablename: String, s: Statement) -> QueryResult {
            let table_index = self.get_table_index(tablename);
            return self.tables[table_index as usize].insert(s);
        }
        pub fn select(&mut self) {}
        pub fn update(&mut self) {}
        pub fn delete(&mut self, index: usize) {}
    }
}
