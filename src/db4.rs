// use crate::field_types::field_types::FieldTypes;
pub mod db4 {
    use std::collections::HashMap;

    use crate::qrtlib::{Database, Namespace, Table,TableField};

    // meta commands

    struct DatabaseManager {
        databases: Vec<Database>,
        tables: Vec<Table>,
        table_indexes: HashMap<String, usize>,
        namespace_indexes: HashMap<String, usize>,
    }

    impl DatabaseManager {
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

        fn ls_tables(&self, name:&str) {}

        fn insert_into_table() {}

        fn select_from_table() {}

        fn update_rows_in_table() {}

        fn delete_rows_in_table() {}
    }
    pub enum MetaCommands {
        EXIT,
        TABLES,
        HELP,
        UnrecognizedCommand,
    }
    impl MetaCommands {
        pub fn from(s: &str) -> MetaCommands {
            let st = s.trim();
            return match st {
                ".exit" | ".EXIT" => MetaCommands::EXIT,
                ".help" | ".HELP" => MetaCommands::HELP,
                ".tables" | ".TABLES" | ".T" => MetaCommands::TABLES,
                _ => MetaCommands::UnrecognizedCommand,
            };
        }
    }

    // main here
    pub fn rundb4() {
        // namespaces: Vec<Namespace>,
        // namespace_indexes:HashMap<String,usize>
        //hasmap
        //table name index in tables
        let mut dblist: Vec<Database> = Vec::new();

        let db1 = Database::new("sys");

        let mut nslist: Vec<Namespace> = Vec::new();

        let ns1 = Namespace::new("accounting", db1.dbname());
        let ns2 = Namespace::new("students", db1.dbname());

        nslist.push(ns1);
        nslist.push(ns2);
        dblist.push(db1);

        let mut tablelist: Vec<Table> = Vec::new();

        // let TableField { name, field_type }

        let tf = TableField::new("name", "vchar");
        let tf2 = TableField::new("age", "int");
        let mut fields: Vec<TableField> = Vec::new();

        fields.push(tf);
        fields.push(tf2);

        let mut table = Table::new("students", fields, nslist[1].name().as_str());

        //fields[1]; hell yeah moved

        tablelist.push(table);

        // struct dbms{
        // databases,namespaces,tables,records
        //
        // } ?

        fn gela() {
            println!(" I am gela");
        }

        gela();
    }
}
