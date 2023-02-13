pub mod database {
    pub struct Database {
        name: String,
    }

    impl Database {
        pub fn new(name: &str) -> Database {
            return Database {
                name: String::from(name),
            };
        }
        pub fn dbname(&self) -> String {
            return self.name.clone();
        }
    }
}
