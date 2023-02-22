pub mod table {
    use crate::{
        qrtlib::{FieldTypes, Fixedchar, QueryResult, Varchar},
        statements::statements::Statement,
    };

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

        pub fn set(&mut self, s: &String) {
            // println!("{}",self.name());
            let tf = match &self.tf_type {
                FieldTypes::Number(_) => FieldTypes::Number(s.parse().expect("f64")),
                FieldTypes::Integer(_) => FieldTypes::Integer(s.parse().expect("u64")),
                FieldTypes::SignedInteger(_) => FieldTypes::SignedInteger(s.parse().expect("i64")),
                FieldTypes::Varchar(v) => FieldTypes::Varchar(Varchar::new(v.len(), s.clone())),
                FieldTypes::Fxchar(v) => FieldTypes::Fxchar(Fixedchar::new(v.len(), s.clone())),
                FieldTypes::Date(_) => FieldTypes::Date(s.parse().expect("number")),
            };
            self.tf_type = tf;
        }

        pub fn compare(t: TableField, t2: TableField) -> bool {
            return true;
        }
        pub fn typef(&self) -> FieldTypes {
            return self.tf_type.clone();
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
        d: bool,
    }

    impl Record {
        fn dummy() -> Record {
            let tname = String::from("Pencils");
            let field1 = TableField::new("brand", "vchar");
            let mut fields: Vec<TableField> = Vec::new();
            fields.push(field1);
            return Record {
                table: tname,
                fields,
                d: false,
            };
        }
        // pub fn new(fields: Vec<TableField>, values: Vec<String>) -> Record {
        //     return Record::dummy();
        // }
        pub fn new(fields: Vec<TableField>, table: String) -> Record {
            return Record { table, fields, d: false };
        }

        pub fn setd(&mut self, d: bool) {
            self.d = d;
        }

        pub fn d(&self) -> bool {
            return self.d;
        }

        pub fn get(&self, name: String) -> Option<TableField> {
            for field in &self.fields.clone() {
                if field.name == name {
                    return Some(field.clone());
                }
            }
            return None;
        }
        pub fn set(&mut self, name: String, v: &String) {
            for field in &mut self.fields {
                if field.name == name {
                    field.set(v);
                    break;
                }
            }
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
                d: true,
            };
        }
    }

    pub struct Table {
        name: String,
        fields: Vec<TableField>,
        namespace: String,
        records: Vec<Record>,
        ridCounter: u64,
    }
    impl Table {
        pub fn new(name: &str, fields: Vec<TableField>, namespace: &str) -> Table {
            let records: Vec<Record> = Vec::new();
            return Table {
                name: String::from(name),
                fields,
                namespace: String::from(namespace),
                records,
                ridCounter: 0,
            };
        }
        pub fn insert_id_column(&mut self) {
            let id = TableField::new("id", "int");
            self.fields.push(id);
        }
        pub fn tname(&self) -> String {
            return self.name.clone();
        }
        pub fn get_fields(&self) -> Vec<TableField> {
            return self.fields.clone();
        }
        pub fn get_recordid_counter(&self) -> u64 {
            return self.ridCounter;
        }
        pub fn increment_recordid(&mut self) {
            return self.ridCounter += 1;
        }

        pub fn insert(&mut self, s: Statement) -> QueryResult {
            let inserttext = s.verbs[0].clone();
            let binding = inserttext.replace("#", "");
            let mut values: Vec<String> = binding.split(",").map(|e| e.to_string()).collect();
            let rid = self.get_recordid_counter().to_string();
            values.push(rid);
            // println!("{:?}",values);
            // return QueryResult::SUCCESS;
            let mut fields = self.get_fields();
            if values.len() != fields.len() {
                return QueryResult::FAILURE;
            }
            self.increment_recordid();

            let len = values.len();
            for i in 0..len {
                fields[i].set(&values[i]);
            }

            let record = Record::new(fields, self.tname());
            self.records.push(record);

            return QueryResult::SUCCESS;
        }
        pub fn select(&mut self, s: Statement) -> QueryResult {
            let selecttext = s.verbs[0].clone();
            let fields: Vec<String> = selecttext.replace("$", "").split(",").map(|e| String::from(e)).collect();

            let crit = s.get_crit();
            println!("{:?}", crit.len());
            println!("{:?}", fields);
            println!("records:{}", &self.records.len());

            for r in &self.records {
                let mut applies = true;
                if crit.len() > 0 {
                    // break;
                    for c in &crit {
                        let pname = c.get_pname();
                        if let Some(v) = &r.get(pname) {
                            if !c.apply(&v) {
                                applies = false;
                                break;
                            }
                        }
                    }
                }
                if !applies {
                    continue;
                }
                for f in &fields {
                    if let Some(v) = &r.get(f.to_string()) {
                        match &v.tf_type {
                            FieldTypes::Number(x) => println!("{}:{}", f, x),
                            FieldTypes::Integer(x) => println!("{}:{}", f, x),
                            FieldTypes::SignedInteger(x) => println!("{}:{}", f, x),
                            FieldTypes::Varchar(x) => println!("{}:{}", f, x.get()),
                            FieldTypes::Fxchar(x) => println!("{}:{}", f, x.get()),
                            FieldTypes::Date(x) => println!("{}:{}", f, x),
                        }
                    } else {
                    }
                }
            }

            return QueryResult::FAILURE;
        }
        pub fn update(&mut self, s: Statement) -> QueryResult {
            let crit = s.get_crit();
            let update_text = s.verbs[0].clone();
            let fields: Vec<String> = update_text.replace("*", "").split(",").map(|e| String::from(e)).collect();

            for r in &mut self.records {
                let mut applies = true;
                if crit.len() > 0 {
                    // break;
                    for c in &crit {
                        let pname = c.get_pname();
                        if let Some(v) = &r.get(pname) {
                            if !c.apply(&v) {
                                applies = false;
                                break;
                            }
                        }
                    }
                }
                if !applies {
                    continue;
                }
                for f in &fields {
                    let split_updater: Vec<String> = f.split("=").map(|e| String::from(e)).collect();
                    if split_updater.len() != 2 {
                        // ignore and continue
                        continue;
                    }
                    let pname = split_updater[0].clone();
                    let value = split_updater[1].clone();
                    r.set(pname, &value);
                }
            }
            return QueryResult::FAILURE;
        }
        pub fn delete(&mut self, s: Statement) -> QueryResult {
            let crit = s.get_crit();

            if crit.len() == 0 {
                println!("delete:empty");
                return QueryResult::FAILURE;
            }
            for r in &mut self.records {
                let mut applies = true;
                if crit.len() > 0 {
                    // break;
                    for c in &crit {
                        let pname = c.get_pname();
                        if let Some(v) = &r.get(pname) {
                            if !c.apply(&v) {
                                applies = false;
                                break;
                            }
                        }
                    }
                }
                if !applies {
                    continue;
                }
                r.setd(true);
            }
            self.records.retain(|x| !x.d());

            return QueryResult::FAILURE;
        }
        pub fn serialize() {}
        pub fn deserialize() {}
    }
}
