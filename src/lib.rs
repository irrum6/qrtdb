mod database;
mod db4;
mod field_types;
mod table;

mod command_parser;
mod statements;

mod parser;

pub mod qrtlib {
    // use crate::parser;

    pub use super::command_parser::command_parser::MetaCommands;
    pub use super::database::database::Database;
    pub use super::db4::db4::rundb4;
    pub use super::db4::db4::Database4;
    pub use super::field_types::field_types;
    pub use super::statements::statements;
    pub use super::table::table;

    pub use super::parser::read2;
    pub use super::parser::whole_statement2;

    pub fn help() {
        println!("Help!");
    }
}
