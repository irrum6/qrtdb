// pub mod db;

// pub use db::db::{Database, MetaCommands};

mod database;
mod db4;
mod field_types;
mod table;

mod command_parser;
mod statements;

pub mod qrtlib {
    pub use super::command_parser::command_parser::MetaCommands;
    pub use super::database::database::Database;
    pub use super::db4::db4::rundb4;
    pub use super::field_types::field_types::{FieldTypes, Fixedchar, Varchar};
    pub use super::statements::statements::{
        Criteria, DDLStatementTypes, DMLStatementTypes, PrepareResult, QueryResult, WhereClauses,
    };
    pub use super::table::table::{Table, TableColumn,Constraint,ConstraintTypes};
    pub fn help() {
        println!("Help!");
    }
}
