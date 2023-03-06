pub mod statements {
    use crate::{
        qrtlib::FieldTypes,
        table::table::{RecordValue, RecordValueTypes},
    };
    #[derive(Clone, PartialEq)]
    pub enum DDLStatementTypes {
        CreateDatabase,
        CreateNamespace,
        CreateTable,
        AlterDatabase,
        AlterNamespace,
        AlterTable,
        InfoDatabase,
        InfoNamespace,
        InfoTable,
        DropDatabase,
        DropNamespace,
        DropTable,
        NONVALID,
    }

    impl DDLStatementTypes {
        pub fn from(token: &str) -> DDLStatementTypes {
            if token.len() == 0 {
                println!("empty token");
                return DDLStatementTypes::NONVALID;
            }
            // let s = String::from(token);
            println!("{}", token);
            let start = &token[..2];
            let end = &token[token.len() - 2..token.len()];

            println!("{}", start);
            println!("{}", end);

            // ignore before better parser
            if String::from(start) != end.chars().rev().collect::<String>() {
                return DDLStatementTypes::NONVALID;
            }
            let mut startend: String = start.to_owned();
            startend.push_str(end);

            return match startend.as_str() {
                "#dd#" => DDLStatementTypes::CreateDatabase,
                "$dd$" => DDLStatementTypes::InfoDatabase,
                "*dd*" => DDLStatementTypes::AlterDatabase,
                "!dd!" => DDLStatementTypes::DropDatabase,
                "#tt#" => DDLStatementTypes::CreateTable,
                "$tt$" => DDLStatementTypes::InfoTable,
                "*tt*" => DDLStatementTypes::AlterTable,
                "!tt!" => DDLStatementTypes::DropTable,
                "#nn#" => DDLStatementTypes::CreateNamespace,
                "$nn$" => DDLStatementTypes::InfoNamespace,
                "*nn*" => DDLStatementTypes::AlterNamespace,
                "!nn!" => DDLStatementTypes::DropNamespace,
                _ => DDLStatementTypes::NONVALID,
            };
        }
    }
    #[derive(Clone, PartialEq)]
    pub enum DMLStatementTypes {
        INSERT,
        SELECT,
        UPDATE,
        DELETE,
        ORDER,
        NONVALID,
    }
    impl DMLStatementTypes {
        pub fn from(token: &str) -> DMLStatementTypes {
            if token.len() == 0 {
                println!("empty token");
                return DMLStatementTypes::NONVALID;
            }
            let chars: Vec<char> = token.chars().collect();
            let start = chars[0];
            let end = chars[chars.len() - 1];

            if start != end {
                return DMLStatementTypes::NONVALID;
            }

            return match start {
                '#' => DMLStatementTypes::INSERT,
                '$' => DMLStatementTypes::SELECT,
                '*' => DMLStatementTypes::UPDATE,
                '!' => DMLStatementTypes::DELETE,
                '?' => DMLStatementTypes::ORDER,
                _ => DMLStatementTypes::NONVALID,
            };
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum StatementCategory {
        DDLStatement(DDLStatementTypes),
        DMLStatement(DMLStatementTypes),
        UNRECOGNIZED,
    }
    impl StatementCategory {
        pub fn from(token: &str) -> StatementCategory {
            let dml = DMLStatementTypes::from(token);
            let ddl = DDLStatementTypes::from(token);

            if ddl != DDLStatementTypes::NONVALID {
                return StatementCategory::DDLStatement(ddl);
            } else if dml != DMLStatementTypes::NONVALID {
                return StatementCategory::DMLStatement(dml);
            } else {
                return StatementCategory::UNRECOGNIZED;
            }
        }
    }
    #[derive(Clone)]
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
    #[derive(Clone)]
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
                RecordValueTypes::Value(FieldTypes::Number(v)) => {
                    let x: f64 = self.value.parse().unwrap();
                    return WhereClauses::number_cmp(&self.clause, *v, x);
                }
                RecordValueTypes::Value(FieldTypes::Integer(v)) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, *v, x);
                }
                RecordValueTypes::Value(FieldTypes::SignedInteger(v)) => {
                    let x: i64 = self.value.parse().unwrap();
                    return WhereClauses::sigint_cmp(&self.clause, *v, x);
                }
                RecordValueTypes::Value(FieldTypes::Varchar(v)) => v.compare(self.value.clone(), self.clause.clone()),
                RecordValueTypes::Value(FieldTypes::Fxchar(v)) => v.compare(self.value.clone(), self.clause.clone()),
                RecordValueTypes::Value(FieldTypes::Date(v)) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, *v, x);
                }
                RecordValueTypes::NULL => false,
            };
        }

        pub fn get_pname(&self) -> String {
            return self.pname.clone();
        }
        pub fn new(clause: WhereClauses, pname: String, value: String) -> Criteria {
            return Criteria { clause, pname, value };
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

    pub struct Statement {
        st_type: StatementCategory,
        nouns: Vec<String>,
        // also known as actors
        criteria: Vec<Criteria>,
        pub verbs: Vec<String>,
        text: String,
    }

    impl Statement {
        pub fn new(line: &str) -> Statement {
            let st_type = StatementCategory::UNRECOGNIZED;
            let text = String::from(line.trim());
            let nouns: Vec<String> = Vec::new();
            let verbs: Vec<String> = Vec::new();
            let criteria: Vec<Criteria> = Vec::new();
            return Statement {
                st_type,
                text,
                nouns,
                verbs,
                criteria,
            };
        }
        pub fn sttype(&self) -> StatementCategory {
            return self.st_type.clone();
        }
        pub fn get_nouns(&self) -> Vec<String> {
            return self.nouns.clone();
        }
        pub fn get(&self, s: &str) {
            //return property
        }
        pub fn get_crit(&self) -> Vec<Criteria> {
            return self.criteria.clone();
        }
        pub fn prepare(&mut self) -> PrepareResult {
            let tokens: Vec<&str> = self.text.trim().split(" ").collect();
            // println!("{:?}", tokens);

            for token in tokens {
                if token.len() == 0 {
                    continue;
                }
                let stcat = StatementCategory::from(token);
                if stcat != StatementCategory::UNRECOGNIZED {
                    self.verbs.push(String::from(token));
                    self.st_type = stcat;
                    continue;
                }

                if token.contains("@") {
                    // @noun
                    self.nouns = token.replace("@", "").split("::").map(|e| String::from(e)).collect();
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

            return match self.st_type {
                StatementCategory::UNRECOGNIZED => PrepareResult::UnrecognizedStatement,
                _ => PrepareResult::SUCCESS,
            };
        }
    }
}
