pub mod statements {
    use crate::qrtlib::{Database, FieldTypes, TableField};
    #[derive(Clone)]
    pub enum DDLStatementTypes {
        CREATE_DATABASE,
        CREATE_NAMESPACE,
        CREATE_TABLE,
        ALTER_DATABASE,
        ALTER_NAMESPACE,
        ALTER_TABLE,
        INFO_DATABASE,
        INFO_NAMESPACE,
        INFO_TABLE,
        DROP_DATABASE,
        DROP_NAMESPACE,
        DROP_TABLE,
        NONVALID,
    }

    impl DDLStatementTypes {
        pub fn from(token: &str) -> DDLStatementTypes {
            return DDLStatementTypes::CREATE_DATABASE;
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
        criteria: Vec<String>,
        pub verbs: Vec<String>,
        text: String,
    }

    impl Statement {
        pub fn new(line: &str) -> Statement {
            let st_type = StatementCategory::UNRECOGNIZED;
            let text = String::from(line.trim());
            let nouns: Vec<String> = Vec::new();
            let verbs: Vec<String> = Vec::new();
            let criteria: Vec<String> = Vec::new();
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
        pub fn prepare(&mut self) -> PrepareResult {
            let tokens: Vec<&str> = self.text.trim().split(" ").collect();
            println!("{:?}", tokens);

            for token in tokens {
                if token.contains("@") {
                    // @noun
                    self.nouns = token.replace("@", "").split("::").map(|e| String::from(e)).collect();
                    continue;
                }
                if token.contains("[") {
                    // @noun
                    self.criteria = token.split(",").map(|e| String::from(e)).collect();
                    continue;
                }
                if token.len() == 0 {
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
