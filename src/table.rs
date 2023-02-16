pub mod table {
    use crate::qrtlib::FieldTypes;

    #[derive(Clone)]
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
        pub fn name(&self) -> String {
            return self.name.clone();
        }
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
        pub fn get(&self, name: String) -> Option<TableField> {
            for field in &self.fields.clone() {
                if field.name == name {
                    return Some(field.clone());
                }
            }
            return None;
        }
        pub fn set(&self, name: String, t: TableField) {

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

    pub struct Table {
        name: String,
        fields: Vec<TableField>,
        namespace: String,
        records: Vec<Record>,
    }
    impl Table {
        pub fn new(name: &str, fields: Vec<TableField>, namespace: &str) -> Table {
            let records: Vec<Record> = Vec::new();
            return Table {
                name: String::from(name),
                fields,
                namespace: String::from(namespace),
                records,
            };
        }
        pub fn tname(&self) -> String {
            return self.name.clone();
        }
        pub fn get_fields(&self)-> Vec<TableField>{
            return self.fields.clone();
        }
        pub fn insert(&mut self) {}
        pub fn select(&mut self) {}
        pub fn update(&mut self) {}
        pub fn delete(&mut self, index: usize) {
            self.records.swap_remove(index);
        }
        pub fn serialize() {}
        pub fn deserialize() {}
    }
}
