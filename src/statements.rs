pub mod statements {
    use crate::{qrtlib::field_types::FieldTypes, qrtlib::table::RecordValue};
    #[derive(Debug, Clone, PartialEq)]
    pub enum DDLTypes {
        AddDatabase,
        AddNamespace,
        AddTable,
        ChangeDatabase,
        ChangeTable,
        RemoveDatabase,
        RemoveNamespace,
        RemoveTable,
        NotADDL,
    }

    impl DDLTypes {
        pub fn from(token: &str) -> DDLTypes {
            if token.len() == 0 {
                println!("empty token");
                return DDLTypes::NotADDL;
            }
            // let s = String::from(token);
            let start = &token[..2];
            let end = &token[token.len() - 2..token.len()];

            // ignore before better parser
            if String::from(start) != end.chars().rev().collect::<String>() {
                return DDLTypes::NotADDL;
            }
            let mut startend: String = start.to_owned();
            startend.push_str(end);

            return match startend.as_str() {
                "#dd#" => DDLTypes::AddDatabase,
                "*dd*" => DDLTypes::ChangeDatabase,
                "!dd!" => DDLTypes::RemoveDatabase,
                "#tt#" => DDLTypes::AddTable,
                "*tt*" => DDLTypes::ChangeTable,
                "!tt!" => DDLTypes::RemoveTable,
                "#nn#" => DDLTypes::AddNamespace,
                "!nn!" => DDLTypes::RemoveNamespace,
                _ => DDLTypes::NotADDL,
            };
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DMLTypes {
        ADD,
        READ,
        CHANGE,
        REMOVE,
        NotADML,
    }
    impl DMLTypes {
        pub fn from(token: &str) -> DMLTypes {
            if token.len() == 0 {
                println!("empty token");
                return DMLTypes::NotADML;
            }
            let chars: Vec<char> = token.chars().collect();
            let start = chars[0];
            let end = chars[chars.len() - 1];

            if start != end {
                return DMLTypes::NotADML;
            }

            return match start {
                '#' => DMLTypes::ADD,
                '$' => DMLTypes::READ,
                '*' => DMLTypes::CHANGE,
                '!' => DMLTypes::REMOVE,
                _ => DMLTypes::NotADML,
            };
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum StatementCategory {
        DDL(DDLTypes),
        DML(DMLTypes),
        UNRECOGNIZED,
    }
    impl StatementCategory {
        pub fn from(token: &str) -> StatementCategory {
            let dml = DMLTypes::from(token);
            let ddl = DDLTypes::from(token);

            if ddl != DDLTypes::NotADDL {
                return StatementCategory::DDL(ddl);
            } else if dml != DMLTypes::NotADML {
                return StatementCategory::DML(dml);
            } else {
                return StatementCategory::UNRECOGNIZED;
            }
        }
    }
    // CriteriaTypes
    #[derive(Debug, Clone)]
    pub enum WhereClauses {
        Equal,
        NonEqual,
        Greater,
        Less,
        Empty,
    }
    impl WhereClauses {
        pub fn from(cho: &str) -> WhereClauses {
            return match cho {
                "=" => WhereClauses::Equal,
                "!=" | "<>" => WhereClauses::NonEqual,
                ">" => WhereClauses::Greater,
                "<" => WhereClauses::Less,
                _ => WhereClauses::Empty,
                //todo greater or equal
            };
        }
        pub fn number_cmp(w: &WhereClauses, v: f64, x: f64) -> bool {
            return match w {
                WhereClauses::Equal => x == v,
                WhereClauses::NonEqual => x != v,
                WhereClauses::Greater => v > x,
                WhereClauses::Less => v < x,
                WhereClauses::Empty => true,
            };
        }
        pub fn int_cmp(w: &WhereClauses, v: u64, x: u64) -> bool {
            return match w {
                WhereClauses::Equal => x == v,
                WhereClauses::NonEqual => x != v,
                WhereClauses::Greater => v > x,
                WhereClauses::Less => v < x,
                WhereClauses::Empty => true,
            };
        }
        pub fn sigint_cmp(w: &WhereClauses, v: i64, x: i64) -> bool {
            return match w {
                WhereClauses::Equal => x == v,
                WhereClauses::NonEqual => x != v,
                WhereClauses::Greater => v > x,
                WhereClauses::Less => v < x,
                WhereClauses::Empty => true,
            };
        }
    }
    #[derive(Clone, Debug)]
    pub struct Criteria {
        clause: WhereClauses,
        pname: String,
        value: String,
    }

    impl Criteria {
        pub fn apply(&self, rev: &RecordValue) -> bool {
            // if t.name() != self.pname {
            //     return false;
            // }
            // if &self.clause == WhereClauses::Empty {
            //     return true;
            // }

            return match rev.get_referenced() {
                FieldTypes::Number(v) => {
                    let x: f64 = self.value.parse().unwrap();
                    return WhereClauses::number_cmp(&self.clause, *v, x);
                }
                FieldTypes::Integer(v) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, *v, x);
                }
                FieldTypes::SignedInteger(v) => {
                    let x: i64 = self.value.parse().unwrap();
                    return WhereClauses::sigint_cmp(&self.clause, *v, x);
                }
                FieldTypes::Varchar(v) => v.compare(self.value.clone(), self.clause.clone()),
                FieldTypes::Fxchar(v) => v.compare(self.value.clone(), self.clause.clone()),
                FieldTypes::Date(v) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, *v, x);
                }
            };
        }

        pub fn get_pname(&self) -> String {
            return self.pname.clone();
        }
        pub fn new(clause: WhereClauses, pname: String, value: String) -> Criteria {
            return Criteria { clause, pname, value };
        }
        pub fn public_from(token: String)->Option<Criteria>{
            return  Criteria::from(token);
        }
        fn from(token: String) -> Option<Criteria> {
            let mut pat = "";
            if token.contains("=") {
                pat = "=";
            }
            if token.contains(">") {
                pat = ">";
            }
            if token.contains("<") {
                pat = "<";
            }
            if token.contains("!=") {
                pat = "!=";
            }
            if token.contains("<>") {
                pat = "<>";
            }
            if pat.len() > 0 {
                let x: Vec<String> = token.trim().split(pat).map(|e| String::from(e)).collect();
                if x.len() != 2 {
                    return None;
                }
                let pname = x[0].clone();
                let value = x[1].clone();
                let critter = Criteria::new(WhereClauses::from(pat), pname, value);
                return Some(critter);
            }
            return None;
        }
        pub fn en_masse(fromwhere: String) -> Vec<Criteria> {
            let mut crits: Vec<Criteria> = Vec::new();
            let replaced = fromwhere.trim().replace("[", "").replace("]", "");
            let split: Vec<String> = replaced.split(",").map(|e| String::from(e)).collect();
            for sp in split {
                if let Some(critter) = Criteria::from(sp) {
                    crits.push(critter);
                }
            }
            return crits;
        }
    }

    pub enum PrepareResult {
        SUCCESS,
        UnrecognizedStatement,
    }

    pub enum QueryResult {
        SUCCESS,
        FAILURE,
    }

    #[derive(Debug, Clone)]
    pub struct Statement {
        category: StatementCategory,
        objectnames: Vec<String>,
        // also known as actors
        criteria: Vec<Criteria>,
        pub verbs: Vec<String>,
        text: String,
    }

    impl Statement {
        pub fn new(line: &str) -> Statement {
            let category = StatementCategory::UNRECOGNIZED;
            let text = String::from(line.trim());
            let objectnames: Vec<String> = Vec::new();
            let verbs: Vec<String> = Vec::new();
            let criteria: Vec<Criteria> = Vec::new();
            return Statement {
                category,
                text,
                objectnames,
                verbs,
                criteria,
            };
        }
        pub fn empty() -> Statement {
            return Statement::new("");
        }
        pub fn sttype(&self) -> StatementCategory {
            return self.category.clone();
        }
        pub fn get_objectnames(&self) -> Vec<String> {
            return self.objectnames.clone();
        }
        pub fn get(&self, s: &str) {
            //return property
        }

        pub fn get_verbs_ref(&self)->&str{
            return &self.verbs[0];
        }

        pub fn get_crit(&self) -> Vec<Criteria> {
            return self.criteria.clone();
        }
        pub fn prepare(&mut self) -> PrepareResult {
            let tokens: Vec<&str> = self.text.trim().split(" ").collect();

            for token in tokens {
                if token.len() == 0 {
                    continue;
                }
                let stcat = StatementCategory::from(token);
                if stcat != StatementCategory::UNRECOGNIZED {
                    self.verbs.push(String::from(token));
                    self.category = stcat;
                    continue;
                }

                if token.contains("@") {
                    // @noun
                    self.objectnames = token.replace("@", "").split("::").map(|e| String::from(e)).collect();
                    continue;
                }
                if token.contains("[") {
                    self.criteria = Criteria::en_masse(String::from(token));
                    continue;
                }
            }

            if self.verbs.len() != 1 {
                println!("there should be only one verb");
                if self.verbs.len() > 1 {
                    println!("more than one verb was provided");
                }
                if self.verbs.len() == 0 {
                    println!("no verb was provided");
                }
                return PrepareResult::UnrecognizedStatement;
            }

            return match self.category {
                StatementCategory::UNRECOGNIZED => PrepareResult::UnrecognizedStatement,
                _ => PrepareResult::SUCCESS,
            };
        }
        //new fn

        pub fn add_verb(&mut self, s: String) {
            self.verbs.push(s);
        }

        pub fn add_noun(&mut self, s: String) {
            self.objectnames.push(s);
        }

        pub fn add_crit(&mut self, c: Criteria) {
            self.criteria.push(c);
        }

        pub fn set_category(&mut self, s: StatementCategory) {
            self.category = s;
        }

        pub fn prepare2(&mut self) -> PrepareResult {
            if self.verbs.len() != 1 {
                println!("there should be only one verb");
                if self.verbs.len() > 1 {
                    println!("more than one verb was provided");
                }
                if self.verbs.len() == 0 {
                    println!("no verb was provided");
                }
                return PrepareResult::UnrecognizedStatement;
            }

            return match self.category {
                StatementCategory::UNRECOGNIZED => PrepareResult::UnrecognizedStatement,
                _ => PrepareResult::SUCCESS,
            };
        }
    }
}
