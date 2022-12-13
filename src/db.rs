pub mod db {

    struct Student {
        name: String,
        full_name: String,
        age: u8,
        id: String,
    }
    impl Student {
        pub fn new(name: String, full_name: String, age: u8, id: String) -> Student {
            return Student {
                name,
                full_name,
                age,
                id,
            };
        }
    }
    trait Table {
        fn select(&self);
        fn insert(&mut self);
        fn update(&mut self);
        fn delete(&mut self);
        fn head(&self);
        fn set_query_params(&mut self, s: String);
        fn name(&self) -> String;
    }

    struct StudentRecords {
        rows: Vec<Student>,
        current_params: String,
    }
    impl StudentRecords {
        pub fn new() -> StudentRecords {
            let rows: Vec<Student> = Vec::new();
            let current_params = String::new();
            return StudentRecords {
                rows,
                current_params,
            };
        }
    }
    impl Table for StudentRecords {
        fn select(&self) {
            for record in &self.rows {
                let x = format!("{},{}", record.name, record.age);
                println!("{}", x);
            }
        }

        fn insert(&mut self) {
            // name: String, full_name: String, age: u64, id: String
            let split: Vec<&str> = self.current_params.split(";").collect();

            let name = String::from(split[0]);
            let full_name = String::from(split[1]);
            let age: u8 = String::from(split[2]).parse().expect("number");
            let id = String::from(split[3]);

            let new_stu = Student::new(name, full_name, age, id);
            self.rows.push(new_stu);
            // todo!()
        }

        fn update(&mut self) {
            todo!()
        }

        fn delete(&mut self) {
            todo!()
        }
        fn head(&self) {
            // print table head aka table info
            println!("name, full name, age , id");
        }
        fn set_query_params(&mut self, s: String) {
            self.current_params = s.clone();
        }

        fn name(&self) -> String {
            return String::from("StudentRecords");
        }
    }

    pub enum MetaCommands {
        EXIT,
        TABLES,
        HELP,
        UnrecognizedCommand,
    }

    impl MetaCommands {
        pub fn from(s: &str) -> MetaCommands {
            let st = s.trim();
            return match st {
                ".exit" | ".EXIT" => MetaCommands::EXIT,
                ".help" | ".HELP" => MetaCommands::HELP,
                ".tables" | ".TABLES" | ".T" => MetaCommands::TABLES,
                _ => MetaCommands::UnrecognizedCommand,
            };
        }
    }

    pub enum PrepareResult {
        SUCCESS,
        UnrecognizedStatement,
    }

    #[derive(Clone, Copy)]
    pub enum StatementType {
        INSERT,
        SELECT,
    }

    pub struct Statement {
        st_type: StatementType,
    }

    impl Statement {
        pub fn new() -> Statement {
            let st_type = StatementType::SELECT;
            return Statement { st_type };
        }
        // pub fn get_type(&self) -> StatementType {
        //     return self.st_type;
        // }
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

        fn execute(&mut self) {
            let strong = match self.st_type {
                StatementType::INSERT => "We want to INSERT something in uranus",
                StatementType::SELECT => "We want to SELECT from ur mom",
            };
            println!("{}", strong);
        }
    }

    fn print_help() {
        print!("help");
    }

    pub struct Database {
        // https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5
        tables: Vec<Box<dyn Table>>,
        statement: Statement,
    }
    impl Database {
        pub fn new() -> Database {
            let mut tables: Vec<Box<dyn Table>> = Vec::new();
            let table = StudentRecords::new();
            tables.push(Box::new(table));
            let statement = Statement::new();
            return Database { tables, statement };
        }
        fn prepare_statement(&mut self, line: &String) -> PrepareResult {
            return self.statement.prepare(line);
        }
        fn execute_statement(&mut self, line: &String) {
            let split:Vec<&str> = line.split(" ").collect();
            match self.statement.st_type {
                StatementType::INSERT => {
                    let strong = String::from(split[1]);
                    self.tables[0].set_query_params(strong);
                    self.tables[0].insert();
                }
                StatementType::SELECT =>{
                    self.tables[0].select();
                },
            }
        }

        pub fn lstables(&self) {
            for table in &self.tables {
                println!("{}", table.name());
            }
        }
        pub fn help(&self) {
            print_help();
        }
        
        pub fn process_statement(&mut self, line: &String) {
            //split parameters
            let pr = self.prepare_statement(line);
            match pr {
                PrepareResult::SUCCESS => self.execute_statement(line),
                PrepareResult::UnrecognizedStatement => println!("UnrecognizedStatement"),
            };
        }
    }
}
