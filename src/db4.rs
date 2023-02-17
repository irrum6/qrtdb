// use crate::field_types::field_types::FieldTypes;
pub mod db4 {
    use std::collections::HashMap;

    use crate::{
        qrtlib::{Database, MetaCommands, PrepareResult, QueryResult, Table, TableField},
        statements::statements::{DMLStatementTypes, Statement, StatementCategory},
    };

    // meta commands

    struct Database4 {
        databases: Vec<Database>,
        database_indexes: HashMap<String, u64>,
        dbindex: u64,
        working_database_index: u64,
    }

    impl Database4 {
        pub fn new() -> Database4 {
            let databases: Vec<Database> = Vec::new();
            let tables: Vec<Table> = Vec::new();

            let database_indexes: HashMap<String, u64> = HashMap::new();
            let dbindex = 0;
            let working_database_index = 0;
            return Database4 {
                databases,
                database_indexes,
                dbindex,
                working_database_index,
            };
        }

        fn create_database(&mut self, name: &str) {
            let database = Database::new(name);
            self.databases.push(database);
            self.dbindex += 1;
            self.database_indexes.insert(String::from(name), self.dbindex);
        }

        pub fn set_working_database(&mut self, name: String) {
            let dab_index = self.database_indexes.get(&name).unwrap();
            self.working_database_index = *dab_index;
        }

        fn create_table() {}

        fn alter_table() {}

        fn drop_table(name: String) {
            let index = 0;
            // for table in &tablelist||{
            // cant capture
            // table.name;
            // }
        }

        fn table_info() {}

        fn ls_tables(&self, name: &str) {
            let dab_index = self.database_indexes.get(name).unwrap();
            self.databases[*dab_index as usize].ls_tables();
        }

        fn insert_into_table(&mut self, dbindex: u64, tablename: String, s: Statement) -> QueryResult {
            return self.databases[dbindex as usize].insert(tablename, s);
        }

        fn select_from_table() {}

        fn update_rows_in_table() {}

        fn delete_rows_in_table() {}

        fn execute(&mut self, s: Statement) -> QueryResult {
            match s.sttype() {
                StatementCategory::DMLStatement(DMLStatementTypes::INSERT) => {
                    // identify table
                    let nouns = s.get_nouns();
                    if nouns.len() > 2 {
                        let dbname = nouns[0].clone();
                        let namespace = nouns[1].clone();
                        let tablename = nouns[2].clone();

                        if let Some(dab_index) = self.database_indexes.get(&dbname) {
                            let tablename_full = Database::compose_table_name(&namespace, &tablename);
                            return self.insert_into_table(*dab_index, tablename_full, s);
                        } else {
                            println!("no database were found with such name");
                            return QueryResult::FAILURE;
                        };
                        // drop(nouns);
                    }

                    if nouns.len() > 1 {}
                    // single identifier table

                    //get database index
                    //get table full name
                    // insert

                    //get fields
                    //get values
                    //make record
                    //inserts
                    // let values = self
                }
                StatementCategory::DMLStatement(DMLStatementTypes::SELECT) => {}
                _ => {}
            }
            return QueryResult::FAILURE;
        }

        pub fn process_statement(&mut self, line: &String) {
            let statements: Vec<&str> = line.split(";").collect();
            for stmt in statements {
                let mut st = Statement::new(stmt);
                let result = st.prepare();
                match result {
                    PrepareResult::UnrecognizedStatement => {
                        println!("Some of the statements failed, aborting");
                        break;
                    }
                    PrepareResult::SUCCESS => {
                        // execute staments
                        self.execute(st);
                    }
                };
            }
        }
        pub fn help() {}
        pub fn lstables(&self) {}
    }

    // main here
    pub fn rundb4() {
        use std::io::stdin;
        let mut db4 = Database4::new();

        db4.create_database("sys");

        db4.databases[0].add_namespace("sys");

        let date = TableField::new("date", "vchar");
        let vmajor = TableField::new("version_major", "int");
        let vminor = TableField::new("version_minor", "int");
        let vpatch = TableField::new("version_patch", "int");
        let vname = TableField::new("version_name", "int");
        let fields = vec![date, vmajor, vminor, vpatch, vname];
        
        db4.databases[0].create_table("sysinfo", fields, "sys");

        let mut line = String::new();

        loop {
            println!("SQRite > ");
            stdin().read_line(&mut line).unwrap();
            // process line
            if line.contains(".") {
                let mc = MetaCommands::from(&line);
                match mc {
                    MetaCommands::EXIT => return,
                    MetaCommands::HELP => Database4::help(),
                    MetaCommands::TABLES => db4.lstables(),
                    MetaCommands::UnrecognizedCommand => {
                        println!("Unrecognized meta command")
                    }
                }
                line.truncate(0);
                continue;
            }
            db4.process_statement(&line);
            line.truncate(0);
        }

        // fn gela() {
        //     println!(" I am gela");
        // }

        // gela();
    }
}
