pub mod table {
    use crate::{qrtlib::{FieldTypes, Fixedchar, QueryResult, Varchar}, statements::statements::Statement};

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

        pub fn serialize(t: TableField) -> Vec<u8> {
            return match t.tf_type {
                FieldTypes::Number(num) => num.to_be_bytes().to_vec(),
                FieldTypes::Integer(int) => int.to_be_bytes().to_vec(),
                FieldTypes::Varchar(var) => var.get().into_bytes(),
                FieldTypes::Fxchar(var) => var.get().into_bytes(),
                FieldTypes::Date(int) => int.to_be_bytes().to_vec(),
                FieldTypes::SignedInteger(int) => int.to_be_bytes().to_vec(),
            };
        }
        pub fn deserialize(f: FieldTypes, name: String, v: Vec<u8>) -> TableField {
            let tf_type: FieldTypes = match f {
                FieldTypes::Number(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Number(f64::from_be_bytes(x))
                }
                FieldTypes::Integer(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Integer(u64::from_be_bytes(x))
                }
                FieldTypes::SignedInteger(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::SignedInteger(i64::from_be_bytes(x))
                }
                FieldTypes::Varchar(vchar) => FieldTypes::Varchar(Varchar::new(vchar.len(), String::from_utf8(v).unwrap())),
                FieldTypes::Fxchar(vchar) => FieldTypes::Fxchar(Fixedchar::new(vchar.len(), String::from_utf8(v).unwrap())),
                FieldTypes::Date(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Date(u64::from_be_bytes(x))
                }
            };
            return TableField { name, tf_type };
            // return TableField::new("age", "integer");
        }
    }

    struct Record {
        table: String,
        fields: Vec<TableField>,
    }

    impl Record {
        fn dummy() -> Record {
            let tname = String::from("Pencils");
            let field1 = TableField::new("brand", "vchar");
            let mut fields: Vec<TableField> = Vec::new();
            fields.push(field1);
            return Record { table: tname, fields };
        }
        pub fn new(fields: Vec<TableField>, values: Vec<String>) -> Record {
            return Record::dummy();
        }
        pub fn get(&self, name: String) -> Option<TableField> {
            for field in &self.fields.clone() {
                if field.name == name {
                    return Some(field.clone());
                }
            }
            return None;
        }
        pub fn set(&self, name: String, t: TableField) {}
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
        pub fn get_fields(&self) -> Vec<TableField> {
            return self.fields.clone();
        }
        pub fn insert(&mut self, s: Statement) -> QueryResult {
            let inserttext = s.verbs[0].clone();
            let binding = inserttext.replace("#", "");
            let values: Vec<&str> = binding.split(",").collect();
            println!("{:?}",values);
            return QueryResult::SUCCESS;
            let fields = self.get_fields();
            if values.len() != fields.len() {
                return QueryResult::FAILURE;
            }
            for f in fields {
                match f.tf_type {
                    FieldTypes::Number(f) => {
                        // f.from_string();
                    }
                    _ => {}
                }
            }
            //get fields
            //create record
            //push int records
            return QueryResult::SUCCESS;
        }
        pub fn select(&mut self, data: String) -> QueryResult {
            return QueryResult::FAILURE;
        }
        pub fn update(&mut self) {}
        pub fn delete(&mut self, index: usize) {
            self.records.swap_remove(index);
        }
        pub fn serialize() {}
        pub fn deserialize() {}
    }
}
