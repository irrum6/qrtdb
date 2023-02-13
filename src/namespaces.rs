pub mod namespaces {
    pub struct Namespace {
        name: String,
    }
    impl Namespace {
        pub fn new(name: &str, dbname: String) -> Namespace {
            return Namespace {
                name: String::from(name),
                // dbname,
            };
        }
        pub fn name(&self) -> String {
            return self.name.clone();
        }
        pub fn dbname(&self) -> String {
            return self.name.clone();
        }
    }
}
