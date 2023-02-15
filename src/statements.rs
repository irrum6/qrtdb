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

    impl DDLStatementTypes {}
    #[derive(Clone)]
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

    pub enum QueryResult{
        SUCCESS,
        FAILURE
    }

    pub struct Statement {
        st_type: StatementCategory,
        text: String,
    }

    impl Statement {
        pub fn new(line: &str) -> Statement {
            let st_type = StatementCategory::UNRECOGNIZED;
            let text = String::from(line.trim());
            return Statement { st_type, text };
        }
        pub fn prepare(&mut self) -> PrepareResult {
            let tokens: Vec<&str> = self.text.split(" ").collect();

            let mut nouns: Vec<&str> = Vec::new();
            let mut verbs: Vec<&str> = Vec::new();
            //where
            let mut criterias: Vec<&str> = Vec::new();
            for token in tokens {
                if token.contains("@") {
                    // @noun
                    nouns = token.split("::").collect();
                    continue;
                }
                if token.contains("[") {
                    // @noun
                    criterias = token.split(",").collect();
                    continue;
                }

            }

            // drop(nouns);

            println!("{:?}", nouns);

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
            if self.text.contains("insert") {
                self.st_type = StatementCategory::DMLStatement(DMLStatementTypes::INSERT);
                return PrepareResult::SUCCESS;
            } else if self.text.contains("select") {
                self.st_type = StatementCategory::DMLStatement(DMLStatementTypes::SELECT);
                return PrepareResult::SUCCESS;
            }
            return PrepareResult::UnrecognizedStatement;
        }
        pub fn execute(&mut self, db: Database) {}
    }
}
