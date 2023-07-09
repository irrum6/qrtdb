pub mod identity {

    use crate::{
        database,
        qrtlib::context::{self, Context, ContextTypes, QueryContext, SessionContext},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Name {
        database: String,
        namespace: String,
        tablename: String,
    }
    impl Name {
        fn derive_full_name(s: String, qc: &impl Context) -> Option<Name> {
            let names: Vec<String> = s.split("::").map(|e| e.to_string()).collect();
            if names.len() == 0 {
                return None;
            }

            if names.len() == 1 {
                let namespace = qc.get_working_namespace();
                let database = qc.get_working_database();
                return Some(Name {
                    database,
                    namespace,
                    tablename: names[0].clone(),
                });
            }
            if names.len() == 2 {
                let database = qc.get_working_database();
                return Some(Name {
                    database,
                    namespace: names[0].clone(),
                    tablename: names[1].clone(),
                });
            }
            return Some(Name {
                database: names[0].clone(),
                namespace: names[1].clone(),
                tablename: names[2].clone(),
            });
        }
        pub fn new(s: String, qc: &impl Context) -> Option<Name> {
            if s.is_empty() {
                println!("empty string");
                return None;
            }
            //resolve ?
            if s.starts_with("?") {
                let alias = s.replace("?", "");
                let full_name = qc.get_alias_value(alias);
                if full_name.1 == false {
                    println!("could not resolve alias");
                    return None;
                }
                return  Some(full_name.0);
                // return Name::derive_full_name(full_name.0, qc);
            }
            return Name::derive_full_name(s, qc);
        }

        pub fn new2(s: String) -> Name {
            let names: Vec<String> = s.split("::").map(|e| e.to_string()).collect();
            return Name {
                database: names[0].clone(),
                namespace: names[1].clone(),
                tablename: names[2].clone(),
            };
        }
        pub fn empty() -> Name {
            return Name {
                database: String::new(),
                namespace: String::new(),
                tablename: String::new(),
            };
        }

        pub fn get_tablename(&self) -> String {
            return self.tablename.clone();
        }
        pub fn get_namespace(&self) -> String {
            return self.namespace.clone();
        }
        pub fn get_database(&self) -> String {
            return self.database.clone();
        }
        pub fn set_database(&self) -> String {
            return self.database.clone();
        }
    }
}
