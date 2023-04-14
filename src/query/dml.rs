mod dml{
    use qrtdb::qrtlib::table::Record;
    struct QueryResult {
        succcess: bool,
        data: Vec<Record>,
        rows_processed: usize,
    }
    // trait query{
    //     fn execute(&self)->QueryResult;
    // }

    struct InsertQuery {
        id: String,
        parameters: Vec<String>,
    }

    // impl query for InsertQuery{
    //     fn execute(&self)->QueryResult {

    //     }
    // }
    struct SelectQuery {
        id: String,
        parameters: Vec<String>,
        criteria: Vec<String>,
        limit: usize,
    }

    trait recognize_self {
        fn recognize_start(&self, s: &str) -> bool;
        fn recognize_end(&self, s: &str) -> bool;
    }

    impl recognize_self for InsertQuery {
        fn recognize_start(&self, s: &str) -> bool {
            return s.trim()=="#";
        }
        fn recognize_end(&self, s: &str) -> bool {
            return s.trim()=="#";
        }
    }

    impl recognize_self for SelectQuery {
        fn recognize_start(&self, s: &str) -> bool {
            return s.trim()=="$";
        }
        fn recognize_end(&self, s: &str) -> bool {
            return s.trim()=="$";
        }
    }
}