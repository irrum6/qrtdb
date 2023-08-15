// use crate::field_types::field_types::FieldTypes;
pub mod db4 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{self, Read},
    };

    use qrtlib::identity::Name;

    use crate::{
        qrtlib::context::{Context, ContextTypes, GlobalContext, QueryContext, SessionContext, UserContext},
        qrtlib::statements::{DDLTypes, DMLTypes, PrepareResult, QueryResult, Statement, StatementCategory},
        qrtlib::{self, read2, whole_statement2, Database, MetaCommands},
    };

    pub struct Database4 {
        databases: Vec<Database>,
        database_indexes: HashMap<String, u64>,
        dbindex: u64,
        working_database_index: u64,
        current_context: ContextTypes,
        ctxqry: QueryContext,
        ctxssn: SessionContext,
        ctxglob: GlobalContext,
        ctxuser: UserContext,
        version: String,
    }

    impl Database4 {
        pub fn new() -> Database4 {
            let databases: Vec<Database> = Vec::new();

            let database_indexes: HashMap<String, u64> = HashMap::new();
            let dbindex = 0;
            let working_database_index = 0;
            let current_context = ContextTypes::QueryContext;
            let ctxqry = QueryContext::new();
            let ctxssn = SessionContext::new();
            let ctxglob = GlobalContext::new();
            let ctxuser = UserContext::new();
            let version = String::from("0.11.3");
            return Database4 {
                databases,
                database_indexes,
                dbindex,
                working_database_index,
                current_context,
                ctxqry,
                ctxssn,
                ctxglob,
                ctxuser,
                version,
            };
        }

        pub fn version(&self) -> &str {
            return &self.version;
        }
        fn set_context(&mut self, ctx: ContextTypes) {
            self.current_context = ctx;
        }

        fn assign_variable(&mut self, n: String, v: String) {
            // &self.current_context.get_variable_value();
            match &self.current_context {
                ContextTypes::GlobalContext => {
                    self.ctxglob.set_variable_value(n, v);
                }
                ContextTypes::UserContext => {
                    self.ctxuser.set_variable_value(n, v);
                }
                ContextTypes::SessionContext => {
                    self.ctxssn.set_variable_value(n, v);
                }
                ContextTypes::QueryContext => {
                    self.ctxqry.set_variable_value(n, v);
                }
            }
        }

        fn create_alias(&mut self, alias: String, name: Name) {
            // &self.current_context.get_variable_value();
            match &self.current_context {
                ContextTypes::GlobalContext => {
                    self.ctxglob.set_alias_value(alias, name);
                }
                ContextTypes::UserContext => {
                    self.ctxuser.set_alias_value(alias, name);
                }
                ContextTypes::SessionContext => {
                    self.ctxssn.set_alias_value(alias, name);
                }
                ContextTypes::QueryContext => {
                    self.ctxqry.set_alias_value(alias, name);
                }
            }
        }

        fn create_database(&mut self, name: &str) -> QueryResult {
            let database = Database::new(name);
            self.databases.push(database);
            self.database_indexes.insert(String::from(name), self.dbindex);
            self.dbindex += 1;
            return QueryResult::SUCCESS;
        }

        pub fn set_working_database(&mut self, name: String) {
            let dab_index = self.database_indexes.get(&name).unwrap();
            self.working_database_index = *dab_index;
        }

        pub fn add_namespace(&mut self, s: Statement) -> QueryResult {
            let objectnames = s.get_objectnames();

            if objectnames.len() < 1 {
                println!("no id");
                return QueryResult::FAILURE;
            }
            let namespace = s.verbs[0].replace("#n", "").replace("n#", "");
            self.databases[0].add_namespace(namespace.as_str());
            return QueryResult::SUCCESS;
        }

        fn create_table(&mut self, s: Statement) -> QueryResult {
            let objectnames = s.get_objectnames();
            let mut dbname = String::new();
            let mut namespace = String::new();
            let mut dab_index: u64 = 0;
            if objectnames.len() > 1 {
                dbname = objectnames[0].clone();
                namespace = objectnames[1].clone();
            } else if objectnames.len() > 0 {
                dbname = objectnames[0].clone();
                if let Some(db_index) = self.database_indexes.get(&dbname) {
                    dab_index = *db_index;
                    namespace = self.databases[dab_index as usize].get_namespace();
                }
            } else {
                dab_index = self.working_database_index;
                namespace = self.databases[dab_index as usize].get_namespace();
            }

            return self.databases[dab_index as usize].create_table(s, namespace.as_str());
        }

        fn table_info(&self, s: Statement) -> QueryResult {
            let objectnames = s.get_objectnames();
            let mut dbname = String::new();
            let mut namespace = String::new();
            let mut tablename = String::new();
            let mut tablename_full = String::new();
            let mut dab_index: u64 = 0;
            if objectnames.len() > 2 {
                dbname = objectnames[0].clone();
                namespace = objectnames[1].clone();
                tablename = objectnames[2].clone();

                if let Some(db_index) = self.database_indexes.get(&dbname) {
                    tablename_full = Database::compose_table_name(&namespace, &tablename);
                    dab_index = *db_index;
                } else {
                    println!("no database were found with such name");
                    return QueryResult::FAILURE;
                };
                // drop(objectnames);
            } else if objectnames.len() > 1 {
                namespace = objectnames[0].clone();
                tablename = objectnames[1].clone();
                dab_index = self.working_database_index;

                tablename_full = Database::compose_table_name(&namespace, &tablename);
            } else {
                tablename = objectnames[0].clone();
                dab_index = self.working_database_index;
                namespace = self.databases[dab_index as usize].get_namespace();

                tablename_full = Database::compose_table_name(&namespace, &tablename);
            }

            return self.databases[dab_index as usize].table_info(tablename_full);
        }

        fn alter_table(&mut self) {}

        fn drop_table(&mut self, name: String) {
            let index = 0;
            // for table in &tablelist||{
            // cant capture
            // table.name;
            // }
        }

        fn insert_into_table(&mut self, dbindex: u64, tablename: String, s: Statement) -> QueryResult {
            return self.databases[dbindex as usize].insert(tablename, s);
        }

        fn select_from_table(&mut self, dbindex: u64, tablename: String, s: Statement) -> QueryResult {
            println!("read table");
            return self.databases[dbindex as usize].select(tablename, s, &self.ctxqry);
        }

        fn update_rows_in_table(&mut self, dbindex: u64, tablename: String, s: Statement) -> QueryResult {
            return self.databases[dbindex as usize].update(tablename, s);
        }

        fn delete_rows_in_table(&mut self, dbindex: u64, tablename: String, s: Statement) -> QueryResult {
            return self.databases[dbindex as usize].delete(tablename, s);
        }
        fn execute_dml_statement(&mut self, s: Statement) -> Option<QueryResult> {
            let objectnames = s.get_objectnames();
            let mut dbname = String::new();
            let mut namespace = String::new();
            let mut tablename = String::new();
            let mut tablename_full = String::new();
            let mut dab_index: u64 = 0;
            if objectnames.len() > 2 {
                dbname = objectnames[0].clone();
                namespace = objectnames[1].clone();
                tablename = objectnames[2].clone();

                if let Some(db_index) = self.database_indexes.get(&dbname) {
                    tablename_full = Database::compose_table_name(&namespace, &tablename);
                    dab_index = *db_index;
                } else {
                    println!("no database were found with such name");
                    return Some(QueryResult::FAILURE);
                };
                // drop(objectnames);
            } else if objectnames.len() > 1 {
                namespace = objectnames[0].clone();
                tablename = objectnames[1].clone();
                dab_index = self.working_database_index;

                tablename_full = Database::compose_table_name(&namespace, &tablename);
            } else {
                //if is one.
                tablename = objectnames[0].clone();
                dab_index = self.working_database_index;
                namespace = self.databases[dab_index as usize].get_namespace();

                tablename_full = Database::compose_table_name(&namespace, &tablename);

                //rough patch
                if tablename.starts_with("?") {
                    let alias = tablename.replace("?", "");
                    let full_name = self.ctxqry.get_alias_value(alias);

                    if full_name.1 == false {
                        println!("could not resolve alias");
                        return Some(QueryResult::FAILURE);
                    }
                    let name = full_name.0;

                    dbname = name.get_database();
                    namespace = name.get_namespace();
                    tablename = name.get_tablename();

                    if let Some(db_index) = self.database_indexes.get(&dbname) {
                        tablename_full = Database::compose_table_name(&namespace, &tablename);
                        dab_index = *db_index;
                    } else {
                        println!("no database were found with such name");
                        return Some(QueryResult::FAILURE);
                    };
                }
            }
            match s.sttype() {
                StatementCategory::DML(DMLTypes::ADD) => {
                    return Some(self.insert_into_table(dab_index, tablename_full, s));
                }
                StatementCategory::DML(DMLTypes::READ) => {
                    return Some(self.select_from_table(dab_index, tablename_full, s));
                }
                StatementCategory::DML(DMLTypes::CHANGE) => {
                    return Some(self.update_rows_in_table(dab_index, tablename_full, s));
                }
                StatementCategory::DML(DMLTypes::REMOVE) => {
                    return Some(self.delete_rows_in_table(dab_index, tablename_full, s));
                }
                _ => {
                    return None;
                }
            }
        }

        fn execute_ddl_statement(&mut self, s: Statement) -> Option<QueryResult> {
            match s.sttype() {
                StatementCategory::DDL(DDLTypes::AddTable) => {
                    return Some(self.create_table(s));
                }
                StatementCategory::DDL(DDLTypes::AddDatabase) => {
                    // self.create_table(s);
                    return Some(self.create_database(s.get_objectnames()[0].as_str()));
                }
                StatementCategory::DDL(DDLTypes::AddNamespace) => {
                    // self.create_table(s);
                    return Some(self.add_namespace(s));
                }

                _ => {}
            }

            return Some(QueryResult::FAILURE);
        }

        pub fn execute(&mut self, s: Statement) -> QueryResult {
            // identify table
            let objectnames = s.get_objectnames();
            let mut objname_excludes = false;

            if StatementCategory::is_variable(&s.sttype()) || StatementCategory::is_alias(&s.sttype()) {
                objname_excludes = true;
            }
            if objectnames.len() == 0 && !objname_excludes {
                //here
                println!("no ids were provided");
                return QueryResult::FAILURE;
            }

            match s.sttype() {
                StatementCategory::DML(_) => {
                    if let Some(r) = self.execute_dml_statement(s) {
                        return r;
                    }
                }
                StatementCategory::DDL(_) => {
                    if let Some(qres) = self.execute_ddl_statement(s) {
                        return qres;
                    }
                }
                StatementCategory::VariableAssignment(va) => self.assign_variable(va.nameget(), va.valueget()),
                StatementCategory::AliasDeclaration(ad) => {
                    self.create_alias(ad.nameget(), ad.valueget());
                    //needs working
                    // println!("alias detected {} {}", ad.nameget(), ad.valueget().get_namespace());
                }
                _ => {}
            }
            return QueryResult::FAILURE;
        }

        pub fn process_statement(&mut self, line: &String) {
            let statements: Vec<&str> = line.trim().split(";").collect();
            // return;
            for stmt in statements {
                if stmt.len() == 0 {
                    continue;
                }
                let mut st = Statement::new(stmt);
                let result = st.prepare();
                match result {
                    PrepareResult::UnrecognizedStatement => {
                        println!("process_statement:Some of the statements failed, aborting");
                        break;
                    }
                    PrepareResult::SUCCESS => {
                        // execute staments
                        st.straighten_criteria(&self.ctxqry);
                        self.execute(st);
                    }
                };
            }
        }
        pub fn process_statement2(&mut self, line: &String) {
            match whole_statement2(&line) {
                Ok((rem, mut stmt)) => {
                    stmt.straighten_criteria(&self.ctxqry);
                    self.execute(stmt);
                }
                Err(nom::Err::Error(ne)) => {
                    println!("Nom error");
                    println!("{:?}", ne);
                }
                Err(e) => {
                    println!("Other error");
                }
            }
        }
        pub fn help() {
            println!("type .rex with filename followed to execute query");
            println!("type .x to quit");
            println!("type .v to display database version");
        }
        pub fn ls(&mut self, s: &String) {
            //over space
            let x: Vec<&str> = s.trim().split(" ").collect();

            if x.len() > 1 && x[1] != "" {
                let ids: Vec<String> = x[1].replace("@", "").split("::").map(|e| String::from(e)).collect();

                if let Some(dbindex) = self.database_indexes.get(&ids[0]) {
                    let dabi = *dbindex;
                    if ids.len() > 1 {
                        self.databases[dabi as usize].ls_tables(&ids[1]);
                    } else {
                        self.databases[dabi as usize].ls_tables("");
                    }
                } else {
                    println!("no database were found with such name");
                    return;
                }
                return;
            }
            //list databases if string empty
            for d in &self.databases {
                println!("database:{}", d.dbname());
            }
            return;
        }

        // pub fn read_and_execute(&mut self, s: &String) -> Result<String, io::Error> {
        //     let x: Vec<&str> = s.trim().split(" ").collect();

        //     if x.len() > 1 && x[1] != "" {
        //         let mut line = String::new();
        //         File::open(x[1])?.read_to_string(&mut line)?;
        //         self.process_statement(&line);
        //     }

        //     return Ok(String::from("success"));
        // }

        pub fn metacommand_processor(&mut self, s: &String) -> bool {
            let mc = MetaCommands::from(&s);
            match mc {
                MetaCommands::EXIT => return true,
                MetaCommands::HELP => Database4::help(),
                MetaCommands::TABLES => self.ls(&s),
                MetaCommands::ReadAndExecute => {
                    //
                    let _res = read2(s, self);
                }
                MetaCommands::UnrecognizedCommand => {
                    println!("Unrecognized meta command")
                }
                MetaCommands::NewParser => {
                    let _res = read2(s, self);
                }
                MetaCommands::VERSION => {
                    println!("Version:{}", self.version());
                }
            }
            return false;
        }
        pub fn init_some(&mut self) {
            self.create_database("sys");

            self.databases[0].add_namespace("sys");
            self.databases[0].add_namespace("info");
            // self.databases[0].insert_info_table();
        }
    }

    // main here
    pub fn rundb4() {
        use std::io::stdin;
        let mut db4 = Database4::new();
        db4.init_some();

        let mut line = String::new();

        println!("Hettooluykaa v{}", db4.version());
        loop {
            println!("Hettooluykaa > ");
            println!("type .help for help");
            stdin().read_line(&mut line).unwrap();
            // process line
            if line.contains(".") {
                if db4.metacommand_processor(&line) {
                    return;
                }
                line.truncate(0);
                continue;
            }
            // db4.process_statement(&line);
            db4.process_statement2(&line);
            line.truncate(0);
        }
    }
}
