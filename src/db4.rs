// use crate::field_types::field_types::FieldTypes;
pub mod db4 {
    use std::collections::HashMap;

    use crate::qrtlib::{Database, MetaCommands, Namespace, Table, TableField};

    // meta commands

    struct Database4 {
        databases: Vec<Database>,
        namespaces: Vec<Namespace>,
        tables: Vec<Table>,
        database_indexes: HashMap<String, usize>,
        namespace_indexes: HashMap<String, usize>,
        table_indexes: HashMap<String, usize>,
    }

    impl Database4 {
        pub fn new() -> Database4 {
            let databases: Vec<Database> = Vec::new();
            let namespaces: Vec<Namespace> = Vec::new();
            let tables: Vec<Table> = Vec::new();

            let database_indexes: HashMap<String, usize> = HashMap::new();
            let namespace_indexes: HashMap<String, usize> = HashMap::new();
            let table_indexes: HashMap<String, usize> = HashMap::new();

            return Database4 {
                databases,
                namespaces,
                tables,
                database_indexes,
                namespace_indexes,
                table_indexes,
            };
        }

        fn create_database(&mut self, name: &str) {
            let database = Database::new(name);
            self.databases.push(database);
        }
        fn create_namespace(&mut self, name: &str) {
            let database = Namespace::new(name);
            self.namespaces.push(database);
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

        fn ls_tables(&self, name: &str) {}

        fn insert_into_table() {}

        fn select_from_table() {}

        fn update_rows_in_table() {}

        fn delete_rows_in_table() {}

        fn execute() {}

        pub fn process_statement(&self, line:  &String) {
            let statements:Vec<&str> = line.split(";").collect();
            for stmt in statements{
                
            }
        }
        pub fn help() {}
        pub fn lstables(&self) {}
    }

    // main here
    pub fn rundb4() {
        use std::io::stdin;
        let db4 = Database4::new();

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
        //craete sys database, sys namespace and sys table
        // sys table columns
        // name sys
        // fields version major integer
        // fields version minor integer
        // fields version patch integer
        // computed field version varchar
        // fields date
        //write version in

        // namespaces: Vec<Namespace>,
        // namespace_indexes:HashMap<String,usize>
        //hasmap
        //table name index in tables
        // let mut dblist: Vec<Database> = Vec::new();

        // let db1 = Database::new("sys");

        // let mut nslist: Vec<Namespace> = Vec::new();

        // let ns1 = Namespace::new("accounting", db1.dbname());
        // let ns2 = Namespace::new("students", db1.dbname());

        // nslist.push(ns1);
        // nslist.push(ns2);
        // dblist.push(db1);

        // let mut tablelist: Vec<Table> = Vec::new();

        // let TableField { name, field_type }

        // let tf = TableField::new("name", "vchar");
        // let tf2 = TableField::new("age", "int");
        // let mut fields: Vec<TableField> = Vec::new();

        // fields.push(tf);
        // fields.push(tf2);

        // let mut table = Table::new("students", fields, nslist[1].name().as_str());

        // //fields[1]; hell yeah moved

        // tablelist.push(table);

        //split by ; and then by " " (space)
        //split by , for fields
        fn gela() {
            println!(" I am gela");
        }

        gela();
    }
}
