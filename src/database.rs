pub mod database {
    use std::collections::HashMap;
    // use std::ops::Index;
    use crate::{
        qrtlib::field_types::FieldTypes,
        qrtlib::statements::{QueryResult, Statement},
        qrtlib::table::{Constraint, ConstraintTypes, Record, RecordValue, RecordValueTypes, Table},
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
            // let date = TableField::new2(String::from("date"), FieldTypes::Date(0));
            // let vmajor = TableField::new2(String::from("version_major"), FieldTypes::Integer(0));
            // let vminor = TableField::new2(String::from("version_minor"), FieldTypes::Integer(0));
            // let vpatch = TableField::new2(String::from("version_patch"), FieldTypes::Integer(0));
            // let vname = TableField::new2(
            //     String::from("version_name"),
            //     FieldTypes::Varchar(Varchar::new(24, String::new())),
            // );
            // let fields = vec![date, vmajor, vminor, vpatch, vname];
            // let cst: Vec<Constraint> = Vec::new();

            // let info = String::from("info");
            // if !self.namespaces.contains(&info) {
            //     self.namespaces.push(info);
            // }

            // return self.insert_table(String::from("infotable"), fields, cst, "info");
            return QueryResult::FAILURE;
        }

        pub fn compose_table_name(namespace: &str, name: &str) -> String {
            let mut tname = String::from(namespace);
            tname.push_str("_");
            tname.push_str(name);
            return tname;
        }
        pub fn search_for_value_in_table(&self, tablename: &String, name: String, value: String) -> bool {
            println!("{}", tablename);
            let binding = tablename.replace("@", "");
            let split: Vec<&str> = binding.split("::").collect();

            let mut tablename_parsed: String = String::from(split[1]);
            tablename_parsed.push_str("_");
            tablename_parsed.push_str(split[2]);

            println!("97:{}", &tablename_parsed);

            let index = self.table_indexes.get(&tablename_parsed);
            if index.is_none() {
                println!("table referenced is not found");
                return false;
            }
            let table_index = index.unwrap();
            return self.tables[*table_index as usize].search_for_value(name, value);
        }
        pub fn check_column_referenced(&self, cs: &Constraint, ftype: String) -> bool {
            println!("{}", cs.ref_table.clone());
            let binding = cs.ref_table.clone().replace("@", "");
            let split: Vec<&str> = binding.split("::").collect();

            let mut tablename_parsed: String = String::from(split[1]);
            tablename_parsed.push_str("_");
            tablename_parsed.push_str(split[2]);

            println!("115:{}", &tablename_parsed);

            let index = self.table_indexes.get(&tablename_parsed);

            if index.is_none() {
                println!("table referenced is not found");
                return false;
            }
            let table_index = index.unwrap();
            let fields = self.tables[*table_index as usize].get_columns();
            let mut found = false;
            for f in fields {
                if &f.name() != &cs.col() {
                    continue;
                }
                found = true;
                // println!("{} {}",f.typef(),&ftype);
                // let ft = FieldTypes::from(&ftype);
                //not necessarily great solution
                if FieldTypes::to2(f.typef()) != ftype {
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
        fn validate_constraints_on_insert(&self, name: String, value: String, index: usize) -> bool {
            if self.tables[index].get_constraints_referenced().len() == 0 {
                return true;
            }

            let colindex = self.tables[index].get_column_index(&name);
            if colindex.is_none() {
                println!("field not found, probably Ok");
                return true;
            }
            let colindex = colindex.unwrap();

            for c in self.tables[index].get_constraints_referenced() {
                if c.col() != name {
                    continue;
                }
                match c.ct() {
                    ConstraintTypes::Unique | ConstraintTypes::PrimaryKey => {
                        //check for unique constraint
                        for r in self.tables[index].get_records_referenced() {
                            if let Some(field) = &r.get(colindex as usize) {
                                if field.to_string() == value {
                                    println!("unique constraint violated");
                                    return false;
                                }
                            }
                        }
                    }
                    //weak/partial constraint , only enforced at insertion
                    ConstraintTypes::ColumnMatch => {
                        return self.search_for_value_in_table(&c.ref_table, c.ref_column.clone(), value);
                    }
                    ConstraintTypes::ForeignKey => {}
                    _ => {}
                }
            }
            return true;
        }

        pub fn insert(&mut self, tablename: String, s: Statement) -> QueryResult {
            // println!("{}", tablename);
            let table_index = self.table_indexes.get(&tablename);
            if table_index.is_none() {
                println!("no tables were found with such name");
                return QueryResult::FAILURE;
            }
            let table_index = table_index.unwrap();

            let inserttext = s.verbs[0].clone();
            let binding = inserttext.replace("#", "");
            let mut values: Vec<String> = binding.split(",").map(|e| e.to_string()).collect();
            // let mut target = self.tables[*table_index as usize];
            self.tables[*table_index as usize].increment_recordid();
            let rid = self.tables[*table_index as usize].get_recordid_counter().to_string();
            values.insert(0, rid);

            let mut record = Record::empty();
            if values.len() != self.tables[*table_index as usize].get_columns_referenced().len() {
                println!("value length not matching");
                return QueryResult::FAILURE;
            }

            let len = values.len();
            for i in 0..len {
                let column = self.tables[*table_index as usize].get_column_name(i);
                let valid = self.validate_constraints_on_insert(column, values[i].clone(), *table_index as usize);
                if !valid {
                    return QueryResult::FAILURE;
                }
                let ft = FieldTypes::create_with_value_ta_(
                    self.tables[*table_index as usize].get_columns_referenced()[i].data_type_ref(),
                    &values[i],
                );
                if ft.is_none() {
                    println!("error parsing values");
                    // self.decrement_recordid();
                    return QueryResult::FAILURE;
                }
                let ftvalue = ft.unwrap();
                let rv: RecordValue = RecordValue::new(RecordValueTypes::Value(ftvalue));
                record.fields.push(rv);
            }

            self.tables[*table_index as usize].insert_record(record);
            return QueryResult::SUCCESS;
            // return self.tables[*table_index as usize].insert(s, &mut self);
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
