pub mod db;

pub use db::db::{Database, MetaCommands};

pub mod qrtlib{
    pub fn help(){
        println!("Help!");
    }
}