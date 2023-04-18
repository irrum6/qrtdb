// new statement
pub mod statements_new {
    use crate::{
        qrtlib::field_types::FieldTypes,
        qrtlib::table::{RecordValue,},
    };
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

    #[derive(Clone)]
    pub enum CriteriaTypes {
        Equal,
        NonEqual,
        Greater,
        Less,
        Empty,
    }

    #[derive(Clone)]
    pub struct Criteria {
        crit_type: CriteriaTypes,
        pname: String,
        value: String,
    }

    impl Criteria {
        pub fn apply(&self, rev: &RecordValue) -> bool {
            //to not impede
            return false;
        }

        pub fn get_name(&self) -> &str {
            return &self.pname;
        }

        pub fn new(crit_type: CriteriaTypes, pname: String, value: String) -> Criteria {
            return Criteria { crit_type, pname, value };
        }
    }

    pub enum PrepareResult {
        SUCCESS,
        Unrecognized,
    }

    pub enum QueryResult {
        SUCCESS,
        FAILURE,
    }

    pub struct Statement {
        category: StatementCategory,
        objects: Vec<String>,
        // also known as actors
        criteria: Vec<Criteria>,
        pub verbs: Vec<String>,
        text: String,
    }

    impl Statement {
        fn prepare() -> PrepareResult {
            PrepareResult::SUCCESS
        }
        fn execute() -> QueryResult {
            QueryResult::FAILURE
        }
    }
}
