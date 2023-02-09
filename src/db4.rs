pub mod db4 {
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

    struct Nchar {
        length: u64,
        value: String,
    }
    impl Nchar {
        fn new(length: u64, value: String) -> Nchar {
            return Nchar { length, value };
        }
    }
    struct Varchar {
        length: u64,
        value: String,
    }

    struct DBInteger {
        value: u64,
    }
    impl Varchar {
        fn new(length: u64, v: &str) -> Varchar {
            let value = String::from(v);
            if value.len() > length as usize {}
            return Varchar { length, value };
        }
    }

    enum FieldTypes {
        Number(f64),
        Integer(u64),
        Varchar(Varchar),
        Nchar(Nchar),
        Date(u64),
    }

    impl FieldTypes {}
    struct TableField {
        name: String,
        field_type: FieldTypes,
    }

    impl TableField {
        pub fn new(name: &str, field_type: FieldTypes) -> TableField {
            return TableField {
                name: String::from(name),
                field_type,
            };
        }
    }

    struct Record {
        table: String,
        fields: Vec<TableField>,
    }

    impl Record {
        pub fn dummy() -> Record {
            let tname = String::from("Pencils");
            let field1 =
                TableField::new("brand", FieldTypes::Varchar(Varchar::new(20, "KohiNoor")));
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

        let tf = TableField::new("name", FieldTypes::Varchar(Varchar::new(20, "")));
        let tf2 = TableField::new("age", FieldTypes::Integer(16));
        let mut fields: Vec<TableField> = Vec::new();

        fields.push(tf);
        fields.push(tf2);

        let mut table = Table::new("students", fields, nslist[1].name().as_str());

        //fields[1]; hell yeah moved

        tablelist.push(table);

        fn create_table() {}

        fn alter_table() {}

        fn drop_table() {}

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
