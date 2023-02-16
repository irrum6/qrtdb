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
        NONVALID,
    }
    impl DMLStatementTypes {
        pub fn from(token: &str) -> DMLStatementTypes {
            let chars: Vec<char> = token.chars().collect();
            let start = chars[0];
            let end = chars[chars.len() - 1];

            if start != end {
                return DMLStatementTypes::NONVALID;
            }
            if start == '#' {
                return DMLStatementTypes::INSERT;
            }
            if start == '$' {
                return DMLStatementTypes::SELECT;
            }
            if start == '*' {
                return DMLStatementTypes::UPDATE;
            }
            if start == '!' {
                return DMLStatementTypes::DELETE;
            }
            return DMLStatementTypes::NONVALID;
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
        text: String,
    }

    impl Statement {
        pub fn new(line: &str) -> Statement {
            let st_type = StatementCategory::UNRECOGNIZED;
            let text = String::from(line.trim());
            let nouns: Vec<String> = Vec::new();
            let criteria: Vec<String> = Vec::new();
            return Statement {
                st_type,
                text,
                nouns,
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
            let tokens: Vec<&str> = self.text.split(" ").collect();
            for token in tokens {
                if token.contains("@") {
                    // @noun
                    self.nouns = token.split("::").map(|e| String::from(e)).collect();
                    continue;
                }
                if token.contains("[") {
                    // @noun
                    self.criteria = token.split(",").map(|e| String::from(e)).collect();
                    continue;
                }
                let dmltype = DMLStatementTypes::from(token);
                if dmltype == DMLStatementTypes::NONVALID {
                    self.st_type = StatementCategory::UNRECOGNIZED;
                } else {
                    self.st_type = StatementCategory::DMLStatement(dmltype);
                }
            }

            //splite by white space
            //check for constraints
            //once valid
            //return
            // let action $name$ select
            // let id @
            // let where [name=gela,name] it can work with single [ or ] as well
            // let values #(gela,19)# insert
            // if multiple tokens
            // let fields $name$

            return match self.st_type {
                StatementCategory::UNRECOGNIZED => PrepareResult::UnrecognizedStatement,
                _ => PrepareResult::SUCCESS,
            };
        }
    }
}
