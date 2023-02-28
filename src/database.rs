pub mod database {
    use std::collections::HashMap;
    // use std::ops::Index;
    use crate::{
        qrtlib::{Constraint, ConstraintTypes, FieldTypes, QueryResult, Table, TableField, Varchar},
        statements::statements::Statement,
    };

    pub struct Database {
        name: String,
        namespaces: Vec<String>,
        namespace: String,
        tables: Vec<Table>,
        table_indexes: HashMap<String, u64>,
        tindex: u64,
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
                namespace: String::new(),
                tables,
                table_indexes,
                tindex: 0,
            };
        }
        pub fn dbname(&self) -> String {
            return self.name.clone();
        }
        pub fn add_namespace(&mut self, name: &str) {
            let namespace = String::from(name);
            if self.namespaces.contains(&namespace) {
                println!("namespace exists in this database");
                return;
            }
            self.namespaces.push(namespace);
        }
        pub fn remove_namespace(&mut self, name: &str) {
            let namespace = String::from(name);
            if !self.namespaces.contains(&namespace) {
                return;
            }
            self.namespaces.retain(|ns| ns != &namespace);
        }        

        pub fn set_namespace(&mut self, name: &str) {
            if !self.namespaces.contains(&String::from(name)) {
                return;
            }
            self.namespace = String::from(name);
        }
        pub fn get_namespace(&self) -> String {
            return self.namespace.clone();
        }
        pub fn insert_info_table(&mut self) -> QueryResult {
            let date = TableField::new2(String::from("date"), FieldTypes::Date(0));
            let vmajor = TableField::new2(String::from("version_major"), FieldTypes::Integer(0));
            let vminor = TableField::new2(String::from("version_minor"), FieldTypes::Integer(0));
            let vpatch = TableField::new2(String::from("version_patch"), FieldTypes::Integer(0));
            let vname = TableField::new2(
                String::from("version_name"),
                FieldTypes::Varchar(Varchar::new(24, String::new())),
            );
            let fields = vec![date, vmajor, vminor, vpatch, vname];
            let cst: Vec<Constraint> = Vec::new();

            let info = String::from("info");
            if !self.namespaces.contains(&info) {
                self.namespaces.push(info);
            }

            return self.insert_table(String::from("infotable"), fields, cst, "info");
        }

        pub fn compose_table_name(namespace: &str, name: &str) -> String {
            let mut tname = String::from(namespace);
            tname.push_str("_");
            tname.push_str(name);
            return tname;
        }
        pub fn check_column_referenced(&self, cs: &Constraint, ftype: String) -> bool {
            let tablename = cs.ref_table.clone().replace("@", "").replace("::", "_");
            let index = self.table_indexes.get(&tablename);

            if index.is_none() {
                println!("table referenced is not found");
                return false;
            }
            let table_index = index.unwrap();
            let fields = self.tables[*table_index as usize].get_fields();
            let mut found = false;
            for f in fields {
                if &f.name() != &cs.col() {
                    continue;
                }
                found = true;
                if FieldTypes::to(f.typef()) != ftype {
                    println!("Column referenced has different type");
                    return false;
                }
                break;
            }
            if found == false {
                println!("Column referenced is not found");
                return false;
            }
            let exists = true;
            return exists;
        }
        pub fn create_table(&mut self, s: Statement, namespace: &str) -> QueryResult {
            //process statements
            if !self.namespaces.contains(&String::from(namespace)) {
                println!("namespace not found");
                return QueryResult::FAILURE;
            }
            let create_text = s.verbs[0].clone();

            if let Some(table) = Table::build_from_statement(create_text, namespace, &self) {
                self.table_indexes.insert(table.tname(), self.tindex);
                self.tables.push(table);
                self.tindex += 1;
                return QueryResult::SUCCESS;
            }
            return QueryResult::FAILURE;
        }
        fn insert_table(&mut self, name: String, fields: Vec<TableField>, cst: Vec<Constraint>, namespace: &str) -> QueryResult {
            let full_table_name = Database::compose_table_name(namespace, &name);
            let table = Table::new(full_table_name.as_str(), fields, cst);
            
            self.tables.push(table);

            self.table_indexes.insert(String::from(full_table_name), self.tindex);
            self.tindex += 1;
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

        pub fn ls_tables(&self, ns: &str) {
            //if empty all
            //if not by namespace
            if ns == "" {
                for t in &self.tables {
                    println!("{}", t.tname());
                }
                return;
            }
            // else
            for t in &self.tables {
                let tname = t.tname();
                let v: Vec<&str> = tname.split("_").collect();
                if v[0] == ns {
                    println!("{}", t.tname());
                }
            }
        }
        pub fn table_info(&self, tablename: String) -> QueryResult {
            if let Some(table_index) = self.table_indexes.get(&tablename) {
                return self.tables[*table_index as usize].info();
            } else {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            };
        }

        pub fn insert(&mut self, tablename: String, s: Statement) -> QueryResult {
            // println!("{}", tablename);
            if let Some(table_index) = self.table_indexes.get(&tablename) {
                return self.tables[*table_index as usize].insert(s);
            } else {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            };
        }
        pub fn select(&mut self, tablename: String, s: Statement) -> QueryResult {
            // println!("{}", tablename);
            if let Some(table_index) = self.table_indexes.get(&tablename) {
                return self.tables[*table_index as usize].select(s);
            } else {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            };
        }
        pub fn update(&mut self, tablename: String, s: Statement) -> QueryResult {
            if let Some(table_index) = self.table_indexes.get(&tablename) {
                return self.tables[*table_index as usize].update(s);
            } else {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            };
        }
        pub fn delete(&mut self, tablename: String, s: Statement) -> QueryResult {
            if let Some(table_index) = self.table_indexes.get(&tablename) {
                return self.tables[*table_index as usize].delete(s);
            } else {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            };
        }
    }
}
