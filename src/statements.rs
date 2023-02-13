pub mod statements {

    pub enum DDLStatementTypes {
        CREATE_DATABASE,
        CREATE_NAMESPACE,
        CREATE_TABLE,
        ALTER_DATABASE,
        ALTER_NAMESPACE,
        ALTER_TABLE,
        ALTER_DATABASE,
        ALTER_NAMESPACE,
        ALTER_TABLE,
        DROP_DATABASE,
        DROP_NAMESPACE,
        DROP_TABLE,
    }

    impl DDLStatements {}

    pub enum DMLStatementTypes {
        INSERT,
        SELECT,
        UPDATE,
        DELETE,
    }
    impl DMLStatements {}

    #[derive(Clone, Copy)]
    pub enum StatementCategory {
        DDLStatementTypes,
        DMLStatementTypes,
    }

    pub enum PrepareResult {
        SUCCESS,
        UnrecognizedStatement,
    }

    pub struct Statement{
        st_type:StatementCategory
    }

    impl Statement {
        pub fn new() -> Statement {
            let st_type = StatementType::SELECT;
            return Statement { st_type };
        }
        pub fn prepare(&mut self, line: &String) -> PrepareResult {
            if line.contains("insert") {
                self.st_type = StatementType::INSERT;
                return PrepareResult::SUCCESS;
            } else if line.contains("select") {
                self.st_type = StatementType::SELECT;
                return PrepareResult::SUCCESS;
            }
            return PrepareResult::UnrecognizedStatement;
        }
    }
}
