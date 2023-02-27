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
        pub fn new(name: &str, ftype: &str) -> Option<TableField> {
            if name == "" {
                return None;
            }
            if let Some(tf_type) = FieldTypes::from(ftype) {
                return Some(TableField {
                    name: String::from(name),
                    tf_type,
                });
            } else {
                return None;
            }
        }
        pub fn new2(name: String, tf_type: FieldTypes) -> TableField {
            return TableField { name, tf_type };
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
        pub fn to_string(&self) -> String {
            return match &self.tf_type {
                FieldTypes::Number(v) => v.to_string(),
                FieldTypes::Integer(v) => v.to_string(),
                FieldTypes::SignedInteger(v) => v.to_string(),
                FieldTypes::Varchar(v) => v.get(),
                FieldTypes::Fxchar(v) => v.get(),
                FieldTypes::Date(v) => v.to_string(),
            };
        }

        pub fn equal(t: TableField, t2: TableField) -> bool {
            return t.typef() == t2.typef();
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
    #[derive(Clone, PartialEq)]
    pub enum ConstraintTypes {
        ColumnMatch,
        PrimaryKey,
        ForeignKey,
        Unique,
        NoConstraint,
    }
    impl ConstraintTypes {
        pub fn from(ctype: &str) -> ConstraintTypes {
            return match ctype {
                "=m>" => ConstraintTypes::ColumnMatch,
                "=p>" => ConstraintTypes::PrimaryKey,
                "=f>" | "=fk>" => ConstraintTypes::ForeignKey,
                "=u>" => ConstraintTypes::Unique,
                "==>" => ConstraintTypes::NoConstraint,
                _ => ConstraintTypes::NoConstraint,
            };
        }
        pub fn to(ct: &ConstraintTypes) -> String {
            return match ct {
                ConstraintTypes::ColumnMatch => String::from("=m>"),
                ConstraintTypes::PrimaryKey => String::from("=p>"),
                ConstraintTypes::ForeignKey => String::from("=f>"),
                ConstraintTypes::Unique => String::from("=u>"),
                ConstraintTypes::NoConstraint => String::from("==>"),
            };
        }
        pub fn describe(ct: &ConstraintTypes) -> String {
            return match ct {
                ConstraintTypes::ColumnMatch => String::from("Column Match"),
                ConstraintTypes::PrimaryKey => String::from("Primary Key"),
                ConstraintTypes::ForeignKey => String::from("Foreign Key"),
                ConstraintTypes::Unique => String::from("Unique"),
                ConstraintTypes::NoConstraint => String::from("NoConstraint"),
            };
        }
    }
    #[derive(Clone)]
    pub struct Constraint {
        ctype: ConstraintTypes,
        column: String,
        pub ref_table: String,
        pub ref_column: String,
    }
    impl Constraint {
        pub fn new(ct: &str, col: &str, reft: &str, refcol: &str) -> Constraint {
            let ctype = ConstraintTypes::from(ct);
            let column = String::from(col);
            let ref_table = String::from(reft);
            let ref_column = String::from(refcol);
            return Constraint {
                ctype,
                column,
                ref_table,
                ref_column,
            };
        }
        pub fn col(&self) -> String {
            return self.column.clone();
        }
        pub fn from_token(col: &str, token: &str) -> Option<Constraint> {
            let mut pat = "";

            if token.contains("=u>") {
                pat = "=u>";
            }

            if token.contains("=p>") {
                pat = "=p>";
            }

            if token.contains("=p>") || token.contains("=u>") {
                let ctype = ConstraintTypes::from(pat);
                return Some(Constraint {
                    ctype,
                    column: String::from(col),
                    ref_table: String::new(),
                    ref_column: String::new(),
                });
            } else {
                if token.contains("=m>") {
                    pat = "=m>";
                }

                if token.contains("=f>") {
                    pat = "=f>";
                }

                if token.contains("=fk>") {
                    pat = "=fk>";
                }
                if pat.len() > 0 {
                    let column = String::from(col);
                    let split: Vec<String> = token.trim().replace(pat, "").split(".").map(|e| String::from(e)).collect();
                    if split.len() != 2 {
                        println!("bad reference");
                        return None;
                    }
                    let ref_table = split[0].clone();
                    let ref_column = split[1].clone();
                    let ctype = ConstraintTypes::from(pat);

                    return Some(Constraint {
                        ctype,
                        column,
                        ref_table,
                        ref_column,
                    });
                }
                return None;
            }
        }

        pub fn ct(&self) -> ConstraintTypes {
            return self.ctype.clone();
        }
    }
    pub struct Table {
        name: String,
        fields: Vec<TableField>,
        records: Vec<Record>,
        rid_counter: u64,
        // depends on tables
        // tables which depend on it
        constraints: Vec<Constraint>,
        relatives: Vec<String>,
    }
    impl Table {
        pub fn new(name: &str, fields: Vec<TableField>, constraints: Vec<Constraint>) -> Table {
            let records: Vec<Record> = Vec::new();
            let relatives: Vec<String> = Vec::new();
            return Table {
                name: String::from(name),
                fields,
                records,
                rid_counter: 0,
                constraints,
                relatives,
            };
        }
        pub fn insert_id_column(&mut self) {
            if let Some(id) = TableField::new("id", "int") {
                self.fields.push(id);
            }
        }
        pub fn add_constraint(&mut self) {}
        pub fn remove_constraint(&mut self) {}

        pub fn add_relative(&mut self, s: String) {
            self.relatives.push(s);
        }

        fn auto_primary_key(&mut self) {}
        pub fn tname(&self) -> String {
            return self.name.clone();
        }
        pub fn get_fields(&self) -> Vec<TableField> {
            return self.fields.clone();
        }
        pub fn get_recordid_counter(&self) -> u64 {
            return self.rid_counter;
        }
        pub fn increment_recordid(&mut self) {
            return self.rid_counter += 1;
        }
        pub fn info(&self) -> QueryResult {
            for f in &self.fields {
                println!("{}-{}", f.name(), FieldTypes::describe(f.typef()));
            }
            for c in &self.constraints {
                println!("{}-{}", &c.col(), ConstraintTypes::describe(&c.ct()));
            }
            for r in &self.relatives {
                println!("relative:{}", &r);
            }
            return QueryResult::SUCCESS;
        }

        fn validate_constraints_on_insert(&mut self, name: String, value: String) -> bool {
            if self.constraints.len() == 0 {
                return true;
            }
            for c in &self.constraints {
                if c.col() != name {
                    continue;
                }
                match c.ct() {
                    ConstraintTypes::Unique => {
                        //check for unique constraint
                        for r in &self.records {
                            if let Some(field) = r.get(c.col()) {
                                if field.to_string() == value {
                                    println!("unique constraint violated");
                                    return false;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            return true;
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
                println!("value length not matching");
                return QueryResult::FAILURE;
            }
            self.increment_recordid();

            let len = values.len();
            for i in 0..len {
                let valid = self.validate_constraints_on_insert(fields[i].name(), values[i].clone());
                if !valid {
                    return QueryResult::FAILURE;
                }
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
