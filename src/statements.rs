pub mod statements {
    use crate::qrtlib::{FieldTypes, TableField};
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
    }

    impl DDLStatementTypes {}
    #[derive(Clone)]
    pub enum DMLStatementTypes {
        INSERT,
        SELECT,
        UPDATE,
        DELETE,
    }
    impl DMLStatementTypes {}

    #[derive(Clone)]
    pub enum StatementCategory {
        DDLStatement(DDLStatementTypes),
        DMLStatement(DMLStatementTypes),
        UNRECOGNIZED,
    }

    pub enum PrepareResult {
        SUCCESS,
        UnrecognizedStatement,
    }

    pub struct Statement {
        st_type: StatementCategory,
        text: String,
    }

    impl Statement {
        pub fn new(line: &String) -> Statement {
            let st_type = StatementCategory::UNRECOGNIZED;
            let text = line.clone();
            return Statement { st_type, text };
        }
        pub fn prepare(&mut self) -> PrepareResult {
            //splite by white space
            //check for constraints
            //once valid
            //return 
            if self.text.contains("insert") {
                self.st_type = StatementCategory::DMLStatement(DMLStatementTypes::INSERT);
                return PrepareResult::SUCCESS;
            } else if self.text.contains("select") {
                self.st_type = StatementCategory::DMLStatement(DMLStatementTypes::SELECT);
                return PrepareResult::SUCCESS;
            }
            return PrepareResult::UnrecognizedStatement;
        }
    }
}
