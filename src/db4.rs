use crate::field_types::field_types::FieldTypes;
pub mod db4 {
    use crate::field_types::field_types::FieldTypes;

    pub struct TableField {
        name: String,
        tf_type: FieldTypes,
    }

    impl TableField {
        pub fn new(name: &str, ftype: &str) -> TableField {
            let tf_type = FieldTypes::from(ftype);
            return TableField {
                name: String::from(name),
                tf_type,
            };
        }
    }

    struct Database {
        name: String,
    }

    impl Database {
        fn new(name: &str) -> Database {
            return Database {
                name: String::from(name),
            };
        }
        fn dbname(&self) -> String {
            return self.name.clone();
        }
        fn execute() {}
    }

    struct Namespace {
        name: String,
        dbname: String, //database name
    }
    impl Namespace {
        fn new(name: &str, dbname: String) -> Namespace {
            return Namespace {
                name: String::from(name),
                dbname,
            };
        }
        fn name(&self) -> String {
            return self.name.clone();
        }

        fn dbname(&self) -> String {
            return self.dbname.clone();
        }
    }

    struct Table {
        name: String,
        fields: Vec<TableField>,
        namespace: String,
    }
    impl Table {
        pub fn new(name: &str, fields: Vec<TableField>, namespace: &str) -> Table {
            return Table {
                name: String::from(name),
                fields,
                namespace: String::from(namespace),
            };
        }
        pub fn tname(&self) -> String {
            return self.name.clone();
        }
        pub fn serialize() {}
        pub fn deserialize() {}
    }

    struct Record {
        table: String,
        fields: Vec<TableField>,
    }

    impl Record {
        pub fn dummy() -> Record {
            let tname = String::from("Pencils");
            let field1 = TableField::new("brand", "vchar");
            let mut fields: Vec<TableField> = Vec::new();
            fields.push(field1);
            return Record {
                table: tname,
                fields,
            };
        }
        pub fn serialize_record(record: Record) -> Vec<u8> {
            let data: Vec<u8> = Vec::new();
            return data;
        }
        pub fn deserialize_record(data: Vec<u8>, table: Table) -> Record {
            let fields: Vec<TableField> = Vec::new();
            return Record {
                table: table.tname(),
                fields,
            };
        }
    }
    // meta commands

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

    pub enum PrepareResult {
        SUCCESS,
        UnrecognizedStatement,
    }

    #[derive(Clone, Copy)]
    pub enum StatementType {
        INSERT,
        SELECT,
    }

    pub struct Statement {
        st_type: StatementType,
    }

    impl Statement {
        pub fn new() -> Statement {
            let st_type = StatementType::SELECT;
            return Statement { st_type };
        }
        pub fn prepare(&mut self, line: &String) -> PrepareResult {
            if line.contains("insert") {
                self.st_type = StatementType::INSERT;
                return PrepareResult::SUCCESS;
            } else if line.contains("select") {
                self.st_type = StatementType::SELECT;
                return PrepareResult::SUCCESS;
            }
            return PrepareResult::UnrecognizedStatement;
        }
    }
    // main here
    pub fn rundb4() {
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

        fn create_table() {}

        fn alter_table() {}

        fn drop_table(name: String) {
            let index = 0;
            // for table in &tablelist{
            //cant capture
            // }
        }

        fn table_info() {}

        fn ls_tables() {}

        fn insert_into_table() {}

        fn select_from_table() {}

        fn update_rows_in_table() {}

        fn delete_rows_in_table() {}

        fn gela() {
            println!(" I am gela");
        }

        gela();
    }
}
