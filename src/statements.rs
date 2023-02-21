pub mod statements {
    use crate::qrtlib::{Database, FieldTypes, TableField};
    #[derive(Clone)]
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
            let start = &token[..2];
            let end = &token[token.len() - 2..2];

            if start != end {
                return DDLStatementTypes::NONVALID;
            }

            return match start {
                "#d" => DDLStatementTypes::CreateDatabase,
                "$d" => DDLStatementTypes::InfoDatabase,
                "*d" => DDLStatementTypes::AlterDatabase,
                "!d" => DDLStatementTypes::DropDatabase,
                "#t" => DDLStatementTypes::CreateTable,
                "$t" => DDLStatementTypes::InfoTable,
                "*t" => DDLStatementTypes::AlterTable,
                "!t" => DDLStatementTypes::DropTable,
                "#n" => DDLStatementTypes::CreateNamespace,
                "$n" => DDLStatementTypes::InfoNamespace,
                "*n" => DDLStatementTypes::AlterNamespace,
                "!n" => DDLStatementTypes::DropNamespace,
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

    #[derive(Clone)]
    pub enum StatementCategory {
        DDLStatement(DDLStatementTypes),
        DMLStatement(DMLStatementTypes),
        UNRECOGNIZED,
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
        pub fn apply(&self, t: &TableField) -> bool {
            if t.name() != self.pname {
                return false;
            }
            // if &self.clause == WhereClauses::Empty {
            //     return true;
            // }

            return match t.typef() {
                FieldTypes::Number(v) => {
                    let x: f64 = self.value.parse().unwrap();
                    return WhereClauses::number_cmp(&self.clause, v, x);
                }
                FieldTypes::Integer(v) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, v, x);
                }
                FieldTypes::SignedInteger(v) => {
                    let x: i64 = self.value.parse().unwrap();
                    return WhereClauses::sigint_cmp(&self.clause, v, x);
                }
                FieldTypes::Varchar(v) => v.compare(self.value.clone(), self.clause.clone()),
                FieldTypes::Fxchar(v) => v.compare(self.value.clone(), self.clause.clone()),
                FieldTypes::Date(v) => {
                    let x: u64 = self.value.parse().unwrap();
                    return WhereClauses::int_cmp(&self.clause, v, x);
                }
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
            if token.contains("!=") || token.contains("<>") {
                pat = "!="
            }
            if token.contains(">") {
                pat = ">"
            }
            if token.contains("<") {
                pat = "<"
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
            println!("{:?}", tokens);

            for token in tokens {
                if token.len() == 0 {
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
                let dmltype = DMLStatementTypes::from(token);
                if dmltype == DMLStatementTypes::NONVALID {
                    self.st_type = StatementCategory::UNRECOGNIZED;
                } else {
                    self.st_type = StatementCategory::DMLStatement(dmltype);
                    self.verbs.push(String::from(token));
                }
            }

            if self.verbs.len() != 1 {
                println!("only one verb");
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
