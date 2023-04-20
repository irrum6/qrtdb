// new statement
pub mod statements_new {
    #[derive(Clone, PartialEq)]
    pub enum DDLTypes {
        AddDatabase,
        ChangeDatabase,
        RemoveDatabase,
        AddNamespace,
        AddTable,
        ChangeTable,
        RemoveTable,
        NotADDL,
    }

    //dml (data manipulation language) actions are performed on records
    #[derive(Clone, PartialEq)]
    pub enum DMLTypes {
        ADD,
        READ,
        CHANGE,
        REMOVE,
        NotADML,
    }

    #[derive(Clone, PartialEq)]
    pub enum StatementCategory {
        DDL(DDLTypes),
        DML(DMLTypes),
        NotAValidCategory,
    }

    impl StatementCategory {
        //if not a dml or ddl
        //then not a category
    }

    pub struct Statement {
        category: StatementCategory,
        objects: Vec<String>,
        // also known as actors
        criteria: Vec<Criteria>,
        pub verbs: Vec<String>,
        text: String,
    }
}
