pub mod context {
    use std::collections::HashMap;

    use crate::qrtlib::identity::Name;

    pub trait Context {
        fn get_variable_value(&self, variable_name: String) -> (String, bool);
        fn set_variable_value(&mut self, variable_name: String, value: String) -> (String, bool);
        fn get_alias_value(&self, alias: String) -> (Name, bool);
        fn set_alias_value(&mut self, variable_name: String, value: Name) -> (Name, bool);
        fn get_working_database(&self) -> String;
        fn get_working_namespace(&self) -> String;
        fn get_working_table(&self) -> String;
    }

    #[derive()]
    pub struct GlobalContext {
        variables: HashMap<String, String>,
        aliases: HashMap<String, Name>,
    }
    impl GlobalContext {
        pub fn new() -> GlobalContext {
            let variables: HashMap<String, String> = HashMap::new();
            let aliases: HashMap<String, Name> = HashMap::new();
            return GlobalContext { variables, aliases };
        }
    }

    impl Context for GlobalContext {
        fn get_variable_value(&self, variable_name: String) -> (String, bool) {
            let x = self.variables.get(&variable_name);

            if x.is_none() {
                return (String::new(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_alias_value(&self, alias: String) -> (Name, bool) {
            let x = self.aliases.get(&alias);

            if x.is_none() {
                return (Name::empty(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_working_database(&self) -> String {
            return String::new();
        }
        fn get_working_namespace(&self) -> String {
            return String::new();
        }
        fn get_working_table(&self) -> String {
            return String::new();
        }

        fn set_variable_value(&mut self, variable_name: String, value: String) -> (String, bool) {
            todo!()
        }

        fn set_alias_value(&mut self, variable_name: String, value: Name) -> (Name, bool) {
            todo!()
        }
    }
    pub struct UserContext {
        variables: HashMap<String, String>,
        aliases: HashMap<String, Name>,
        working_database: String,
        working_namespace: String,
        working_table: String,
    }
    impl UserContext {
        pub fn new() -> UserContext {
            let variables: HashMap<String, String> = HashMap::new();
            let aliases: HashMap<String, Name> = HashMap::new();
            let working_database = String::new();
            let working_namespace = String::new();
            let working_table = String::new();

            return UserContext {
                variables,
                aliases,
                working_database,
                working_namespace,
                working_table,
            };
        }
    }

    impl Context for UserContext {
        fn get_variable_value(&self, variable_name: String) -> (String, bool) {
            let x = self.variables.get(&variable_name);

            if x.is_none() {
                return (String::new(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_alias_value(&self, alias: String) -> (Name, bool) {
            let x = self.aliases.get(&alias);

            if x.is_none() {
                return (Name::empty(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_working_database(&self) -> String {
            return self.working_database.clone();
        }
        fn get_working_namespace(&self) -> String {
            return self.working_namespace.clone();
        }
        fn get_working_table(&self) -> String {
            return self.working_table.clone();
        }

        fn set_variable_value(&mut self, variable_name: String, value: String) -> (String, bool) {
            todo!()
        }

        fn set_alias_value(&mut self, variable_name: String, value: Name) -> (Name, bool) {
            todo!()
        }
    }

    pub struct SessionContext {
        variables: HashMap<String, String>,
        aliases: HashMap<String, Name>,
        working_database: String,
        working_namespace: String,
        working_table: String,
    }
    impl SessionContext {
        pub fn new() -> SessionContext {
            let variables: HashMap<String, String> = HashMap::new();
            let aliases: HashMap<String, Name> = HashMap::new();
            let working_database = String::new();
            let working_namespace = String::new();
            let working_table = String::new();

            return SessionContext {
                variables,
                aliases,
                working_database,
                working_namespace,
                working_table,
            };
        }
    }
    impl Context for SessionContext {
        fn get_variable_value(&self, variable_name: String) -> (String, bool) {
            let x = self.variables.get(&variable_name);

            if x.is_none() {
                return (String::new(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_alias_value(&self, alias: String) -> (Name, bool) {
            let x = self.aliases.get(&alias);

            if x.is_none() {
                return (Name::empty(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_working_database(&self) -> String {
            return self.working_database.clone();
        }
        fn get_working_namespace(&self) -> String {
            return self.working_namespace.clone();
        }
        fn get_working_table(&self) -> String {
            return self.working_table.clone();
        }

        fn set_variable_value(&mut self, variable_name: String, value: String) -> (String, bool) {
            todo!()
        }

        fn set_alias_value(&mut self, variable_name: String, value: Name) -> (Name, bool) {
            todo!()
        }
    }

    pub struct QueryContext {
        variables: HashMap<String, String>,
        aliases: HashMap<String, Name>,
        working_database: String,
        working_namespace: String,
        working_table: String,
    }
    impl QueryContext {
        pub fn new() -> QueryContext {
            let variables: HashMap<String, String> = HashMap::new();
            let aliases: HashMap<String, Name> = HashMap::new();
            let working_database = String::new();
            let working_namespace = String::new();
            let working_table = String::new();

            return QueryContext {
                variables,
                aliases,
                working_database,
                working_namespace,
                working_table,
            };
        }
    }
    impl Context for QueryContext {
        fn get_variable_value(&self, variable_name: String) -> (String, bool) {
            let x = self.variables.get(&variable_name);

            if x.is_none() {
                return (String::new(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_alias_value(&self, alias: String) -> (Name, bool) {
            let x = self.aliases.get(&alias);

            if x.is_none() {
                return (Name::empty(), false);
            }
            let y = x.unwrap().to_owned();
            return (y, true);
        }
        fn get_working_database(&self) -> String {
            return self.working_database.clone();
        }
        fn get_working_namespace(&self) -> String {
            return self.working_namespace.clone();
        }
        fn get_working_table(&self) -> String {
            return self.working_table.clone();
        }

        fn set_variable_value(&mut self, variable_name: String, value: String) -> (String, bool) {
            self.variables.insert(variable_name, value.clone());
            return (value, true);
        }

        fn set_alias_value(&mut self, variable_name: String, value: Name) -> (Name, bool) {
            self.aliases.insert(variable_name, value.clone());
            return (value, true);
        }
    }

    pub enum ContextTypes {
        GlobalContext,
        UserContext,
        SessionContext,
        QueryContext,
    }
}
