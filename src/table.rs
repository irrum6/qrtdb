pub mod table {
    // use nom::{
    //     branch::alt,
    //     bytes,
    //     bytes::complete::{is_not, tag, tag_no_case, take_till, take_until, take_while, take_while1},
    //     character::complete::{self, char as ncchar, line_ending, multispace0, newline},
    //     character::{is_alphabetic, is_newline, is_space},
    //     combinator::{self, all_consuming, map, map_parser, opt, recognize},
    //     error::context,
    //     multi::separated_list1,
    //     sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    //     IResult,
    // };

    use crate::{
        qrtlib::field_types::FieldTypes,
        qrtlib::statements::Statement,
        qrtlib::statements::{Criteria, QueryResult},
        qrtlib::Database,
    };
    #[derive(Clone)]
    pub struct TableColumn {
        name: String,
        data_type: FieldTypes,
    }
    impl TableColumn {
        pub fn new(name: String, data_type: FieldTypes) -> Option<TableColumn> {
            let taco = TableColumn { name, data_type };
            return Some(taco);
        }

        pub fn new2(name: String, data_type: FieldTypes) -> TableColumn {
            return TableColumn { name, data_type };
        }

        pub fn from_text(input: &str) -> Option<TableColumn> {
            if input.is_empty() {
                return None;
            }
            let split: Vec<&str> = input.split(" ").collect();

            if split.len() < 2 {
                println!("no type was provided for column");
                return None;
            }
            return TableColumn::from(split[0], split[1]);
        }

        pub fn from(name: &str, ftype: &str) -> Option<TableColumn> {
            if name == "" {
                return None;
            }
            if let Some(data_type) = FieldTypes::from(ftype) {
                return Some(TableColumn {
                    name: String::from(name),
                    data_type,
                });
            } else {
                return None;
            }
        }
        pub fn data_type_ref(&self) -> &FieldTypes {
            return &self.data_type;
        }

        pub fn name(&self) -> String {
            return self.name.clone();
        }
        pub fn equal(t: TableColumn, t2: TableColumn) -> bool {
            return t.typef() == t2.typef();
        }

        pub fn self_equal(&self, tc: TableColumn) -> bool {
            return self.data_type == tc.typef();
        }
        pub fn typef(&self) -> FieldTypes {
            return self.data_type.clone();
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct RecordValue {
        pub(super) value: FieldTypes,
    }
    impl RecordValue {
        pub fn new(value: FieldTypes) -> RecordValue {
            return RecordValue { value };
        }
        pub fn to_string(&self) -> String {
            return FieldTypes::tostr(&self.value);
        }
        pub fn set(&mut self, r: FieldTypes) {
            self.value = r;
        }
        pub fn get(&self) -> FieldTypes {
            return self.value.clone();
        }
        pub fn get_referenced(&self) -> &FieldTypes {
            return &self.value;
        }

        pub fn from(f: String) -> Option<RecordValue> {
            if let Some(ftype) = FieldTypes::from(&f) {
                return Some(RecordValue { value: ftype });
            }
            return None;
        }
    }
    pub struct Record {
        pub(crate) fields: Vec<RecordValue>,
    }

    impl Record {
        pub fn new(fields: Vec<RecordValue>) -> Record {
            return Record { fields };
        }
        pub fn empty() -> Record {
            let fields: Vec<RecordValue> = Vec::new();
            return Record { fields };
        }

        pub fn get(&self, index: usize) -> Option<RecordValue> {
            if index > self.fields.len() {
                return None;
            }
            return Some(self.fields[index].clone());
        }
        pub fn set(&mut self, index: usize, v: &String) {
            if index > self.fields.len() {
                return;
            }
            // self.fields[index].set(v);
        }
        pub fn serialize_record(record: Record) -> Vec<u8> {
            let data: Vec<u8> = Vec::new();
            return data;
        }
        pub fn deserialize_record(data: Vec<u8>, table: Table) -> Record {
            let fields: Vec<RecordValue> = Vec::new();
            return Record { fields };
        }
    }
    #[derive(Clone, PartialEq)]
    pub enum ConstraintTypes {
        PrimaryKey,
        ForeignKey,
        ColumnMatch,
        Unique,
        NoConstraint,
    }
    impl ConstraintTypes {
        pub fn from(ctype: &str) -> ConstraintTypes {
            return match ctype {
                "matches" | "ma" | "m" => ConstraintTypes::ColumnMatch,
                "primary" | "pk" | "p" => ConstraintTypes::PrimaryKey,
                "foreign" | "fk" | "f" => ConstraintTypes::ForeignKey,
                "unique" | "uq" | "u" => ConstraintTypes::Unique,
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
        pub fn construct_primary_key(name: &str) -> Constraint {
            return Constraint {
                ctype: ConstraintTypes::PrimaryKey,
                column: String::from(name),
                ref_table: String::new(),
                ref_column: String::new(),
            };
        }
        pub fn ct(&self) -> ConstraintTypes {
            return self.ctype.clone();
        }

        pub fn col(&self) -> String {
            return self.column.clone();
        }

        pub fn col_as_ref(&self) -> &String {
            return &self.column;
        }

        pub fn refta(&self) -> &String {
            return &self.ref_table;
        }

        pub fn refcol(&self) -> &String {
            return &&self.ref_column;
        }

        pub fn ct_from(token: &str) -> Option<Constraint> {
            if token.is_empty() {
                return None;
            }

            let split: Vec<String> = token.trim().split(" ").map(|e| String::from(e)).collect();

            if split.len() < 2 {
                return None;
            }

            let column = split[0].clone();

            let ctype = ConstraintTypes::from(&split[1]);

            match ctype {
                ConstraintTypes::Unique | ConstraintTypes::PrimaryKey => {
                    return Some(Constraint {
                        ctype,
                        column,
                        ref_table: String::new(),
                        ref_column: String::new(),
                    });
                }
                ConstraintTypes::NoConstraint => {
                    //and then will ignore it
                    return None;
                }
                ConstraintTypes::ForeignKey | ConstraintTypes::ColumnMatch => {
                    if split.len() < 3 {
                        println!("bad reference");
                        return None;
                    }
                    let references: Vec<String> = split[2].split(".").map(|e| String::from(e)).collect();
                    let ref_table = references[0].clone();
                    let ref_column = references[1].clone();

                    return Some(Constraint {
                        ctype,
                        column,
                        ref_table,
                        ref_column,
                    });
                }
            }
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
            }
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
    pub struct Table {
        name: String,
        head: Vec<String>,
        columns: Vec<TableColumn>,
        records: Vec<Record>,
        rid_counter: u64,
        // depends on tables
        // tables which depend on it
        constraints: Vec<Constraint>,
        relatives: Vec<String>,
    }
    impl Table {
        pub fn create_table_column() {
            // ->Option<TableColumn>
            // name: String,
            // data_type: FieldTypes,
            // unique: bool,
            // is_primary_key: bool,
            // is_foreign_key: bool,
            // matches_other_column: bool,
        }
        pub fn new(name: &str, columns: Vec<TableColumn>, constraints: Vec<Constraint>) -> Table {
            let records: Vec<Record> = Vec::new();
            let relatives: Vec<String> = Vec::new();
            let mut head: Vec<String> = Vec::new();

            for c in &columns {
                head.push(c.name());
            }
            let table = Table {
                name: String::from(name),
                head,
                columns,
                records,
                rid_counter: 0,
                constraints,
                relatives,
            };
            return table;
        }
        pub fn get_column_index(&self, col: &String) -> Option<usize> {
            let mut index = 0;
            for h in &self.head {
                if h == col {
                    return Some(index);
                }
                index += 1;
            }
            return None;
        }
        pub fn get_column_name(&self, index: usize) -> String {
            return self.head[index].clone();
        }

        pub fn get_column_type_ref(&self, index: usize) -> &FieldTypes {
            return self.columns[index].data_type_ref();
        }

        pub fn get_column_type(&self, index: usize) -> FieldTypes {
            return self.columns[index].typef();
        }

        /**
         *
         */
        pub fn build_from_text(create_text: String, namespace: &str, db: &Database) -> Option<Table> {
            let split: Vec<String> = create_text.trim().split("|").map(|e| String::from(e)).collect();

            if split.len() < 2 {
                println!("not enuff");
                return None;
            }
            let name = split[0].trim().clone();
            let fields: Vec<String> = split[1].split(",").map(|e| String::from(e)).collect();

            let mut tablefields: Vec<TableColumn> = Vec::new();

            let idcol = TableColumn::new2(String::from("id"), FieldTypes::Integer(1));
            tablefields.push(idcol);
            let full_table_name = Database::compose_table_name(namespace, &name);

            for f in fields {
                let tc = TableColumn::from_text(&f);
                if tc.is_none() {
                    println!("one of the columns is not described properly");
                    return None;
                }
                let col = tc.unwrap();
                tablefields.push(col);
            }

            let mut cst: Vec<Constraint> = Vec::new();

            let idpk = Constraint::construct_primary_key("id");

            cst.push(idpk);

            let mut table = Table::new(full_table_name.as_str(), tablefields, cst);

            if split.len() > 2 {
                let constraints: Vec<&str> = split[2].trim().split(",").collect();

                for cons in constraints {
                    //new constraint
                    let nc = Constraint::ct_from(cons);
                    if nc.is_none() {
                        continue;
                    }
                    //ha ha beat it
                    let nc = nc.unwrap();
                    table.add_constraint(nc, db);
                }
            }

            // return None;

            return Some(table);
        }

        pub fn build_from_statement(create_text: String, namespace: &str, db: &Database) -> Option<Table> {
            // Table
            let replaced = create_text.trim().replace("#T", "").replace("T#", "");
            let replaced = replaced.replace("#t", "").replace("t#", "");

            // name|fields
            let split: Vec<String> = replaced.split("|").map(|e| String::from(e)).collect();
            let name = split[0].clone();
            let fields: Vec<String> = split[1].split(",").map(|e| String::from(e)).collect();

            let len = fields.len();

            let mut tablefields: Vec<TableColumn> = Vec::new();
            // push the id column first
            if let Some(id) = TableColumn::from("id", "int") {
                tablefields.push(id);
            } else {
                println!("could not insert auto id column");
                return None;
            }

            let mut cst: Vec<Constraint> = Vec::new();
            for ix in 0..len {
                let split: Vec<&str> = fields[ix].split("->").collect();
                if split.len() < 2 || split.len() > 3 {
                    continue;
                }
                let name = split[0];
                if name == "" {
                    println!("empty name was provided for column");
                    return None;
                }
                let ftype = split[1];
                if ftype == "" {
                    println!("columntype can not be described by empty string");
                    return None;
                }

                if split.len() == 3 {
                    //third one is constraints
                    if split[2].contains("=m>") && (split[2].contains("=fk>") || split[2].contains("=f>")) {
                        println!("incompatible constraints");
                        return None;
                    }
                    if split[2].contains("=u>") && split[2].contains("=p>") {
                        println!("incompatible constraints");
                        return None;
                    }
                    if split[2].contains("=p>") && (split[2].contains("=fk>") || split[2].contains("=f>")) {
                        println!("primary key can't be foreign");
                        return None;
                    }

                    let consplit: Vec<&str> = split[2].split("_").collect();
                    for c in consplit {
                        let css = Constraint::from_token(name, c.trim());
                        if css.is_none() {
                            println!("constraint not recognized");
                            return None;
                        }
                        let cs = css.unwrap();
                        match cs.ct() {
                            // for column match and foreign keys check existence of the table referenced
                            ConstraintTypes::ColumnMatch | ConstraintTypes::ForeignKey => {
                                let valid = db.check_column_referenced(&cs, String::from(ftype));
                                if !valid {
                                    return None;
                                }
                            }
                            _ => {}
                        }
                        if cs.ct() != ConstraintTypes::NoConstraint {
                            cst.push(cs);
                        }
                    }
                }
                let data_t = FieldTypes::from(ftype);

                if data_t.is_none() {
                    println!("not valid data type");
                    return None;
                }
                let data_type = data_t.unwrap();
                if let Some(tf) = TableColumn::new(String::from(name), data_type) {
                    tablefields.push(tf);
                } else {
                    return None;
                }
            }

            let full_table_name = Database::compose_table_name(namespace, &name);
            let table = Table::new(full_table_name.as_str(), tablefields, cst);
            return Some(table);
        }

        pub fn add_constraint(&mut self, necot: Constraint, db: &Database) {
            let index = self.get_column_index(&necot.col());
            if index.is_none() {
                println!("name not found in table head, column for specified construct to be constructed is not found");
                println!("contstraint statement will be ignored and not constructed");
                return;
            }
            let index = index.unwrap();

            match necot.ct() {
                // for column match and foreign keys check existence of the table referenced
                ConstraintTypes::ColumnMatch => {
                    let ftype = FieldTypes::to2(self.get_column_type(index));
                    let valid = db.check_column_referenced(&necot, ftype);
                    if !valid {
                        return;
                    }
                    self.constraints.push(necot);
                }
                ConstraintTypes::ForeignKey => {
                    let ftype = FieldTypes::to2(self.get_column_type(index));
                    let valid = db.check_column_referenced(&necot, ftype);
                    if !valid {
                        return;
                    }
                    self.constraints.push(necot);
                }
                ConstraintTypes::PrimaryKey => {
                    //overwrite :remove older primary keys
                    //and set this one instead
                    self.constraints.retain(|x| x.ct() != ConstraintTypes::PrimaryKey);
                    self.constraints.push(necot);
                }
                ConstraintTypes::Unique => {
                    self.constraints.push(necot);
                }
                _ => {}
            }
        }
        pub fn remove_constraint(&mut self, colname: &str, cont: ConstraintTypes) {
            //upsolute
            self.constraints.retain(|x| x.col_as_ref() != colname && x.ct() != cont);
        }

        pub fn add_relative(&mut self, s: String) {
            self.relatives.push(s);
        }
        // fn compose_new_record

        pub fn search_for_value(&self, name: String, value: String) -> bool {
            let colindex = self.get_column_index(&name);
            if colindex.is_none() {
                println!("field not found, probably not Ok");
                return false;
            }
            let colindex = colindex.unwrap();
            for r in &self.records {
                if let Some(field) = &r.get(colindex as usize) {
                    if field.to_string() == value {
                        println!("match constraint validated");
                        return true;
                    }
                }
            }
            return false;
        }

        pub fn tname(&self) -> String {
            return self.name.clone();
        }
        pub fn get_columns(&self) -> Vec<TableColumn> {
            return self.columns.clone();
        }
        pub fn get_columns_referenced(&self) -> &Vec<TableColumn> {
            return &self.columns;
        }
        pub fn get_constraints_referenced(&self) -> &Vec<Constraint> {
            return &self.constraints;
        }
        pub fn get_records_referenced(&self) -> &Vec<Record> {
            return &self.records;
        }
        pub fn get_recordid_counter(&self) -> u64 {
            return self.rid_counter;
        }
        pub fn increment_recordid(&mut self) {
            return self.rid_counter += 1;
        }
        pub fn info(&self) -> QueryResult {
            for f in &self.columns {
                println!("{}-{}", f.name(), FieldTypes::describe(&f.data_type));
            }
            for c in &self.constraints {
                println!("{}-{}", &c.col(), ConstraintTypes::describe(&c.ct()));
            }
            for r in &self.relatives {
                println!("relative:{}", &r);
            }
            return QueryResult::SUCCESS;
        }

        // pub fn insert(&mut self, s: Statement, db: &mut Database) -> QueryResult {
        //     return QueryResult::SUCCESS;
        // }
        pub fn insert_record(&mut self, r: Record) {
            self.records.push(r);
        }
        pub fn select(&mut self, s: Statement) -> QueryResult {
            let selecttext = s.verbs[0].clone();
            let fields: Vec<String> = selecttext.replace("$", "").split(",").map(|e| String::from(e)).collect();

            let mut crit = s.get_crit();
            println!("records:{}", &self.records.len());

            let mut indexes: Vec<usize> = Vec::new();
            for f in fields {
                if let Some(x) = self.get_column_index(&f) {
                    indexes.push(x);
                }
                if let Some(c) = Criteria::public_from(f) {
                    let critfield = c.get_pname();
                    crit.push(c);
                    if let Some(x) = self.get_column_index(&critfield) {
                        indexes.push(x);
                    }
                }
            }
            for r in &self.records {
                let mut applies = true;
                if crit.len() > 0 {
                    for c in &crit {
                        let pindex = self.get_column_index(&c.get_pname());
                        if pindex.is_none() {
                            continue;
                        }
                        let pindex = pindex.unwrap();
                        if let Some(v) = &r.get(pindex) {
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
                for i in &indexes {
                    if let Some(v) = &r.get(*i) {
                        match &v.value {
                            FieldTypes::Varchar(x) => println!("{}", x.get()),
                            FieldTypes::Fxchar(x) => println!("{}", x.get()),
                            FieldTypes::Number(x) => println!("{}", x),
                            FieldTypes::Integer(x) => println!("{}", x),
                            FieldTypes::SignedInteger(x) => println!("{}", x),
                            FieldTypes::Date(x) => println!("{}", x),
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
                    // for c in &crit {
                    //     let pname = c.get_pname();
                    //     if let Some(v) = &r.get(pname) {
                    //         if !c.apply(&v) {
                    //             applies = false;
                    //             break;
                    //         }
                    //     }
                    // }
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
                    // r.set(pname, &value);
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
            //ids array
            //
            for r in &mut self.records {
                let mut applies = true;
                if crit.len() > 0 {
                    // break;
                    // for c in &crit {
                    //     let pname = c.get_pname();
                    //     if let Some(v) = &r.get(pname) {
                    //         if !c.apply(&v) {
                    //             applies = false;
                    //             break;
                    //         }
                    //     }
                    // }
                }
                if !applies {
                    continue;
                }
            }
            // if id in ids array;
            // self.records.retain(|x| !x.d());

            return QueryResult::FAILURE;
        }
        pub fn serialize() {}
        pub fn deserialize() {}
    }
}
