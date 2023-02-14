// pub mod db;

// pub use db::db::{Database, MetaCommands};

mod db4;
mod field_types;
mod table;
mod namespaces;
mod database;

mod command_parser;

pub mod qrtlib {
    pub use super::db4::db4::rundb4;
    pub use super::field_types::field_types::FieldTypes;
    pub use super::table::table::{Table, TableField};
    pub use super::database::database::Database;
    pub use super::namespaces::namespaces::Namespace;
    pub use super::command_parser::command_parser::MetaCommands;
    pub fn help() {
        println!("Help!");
    }
}
