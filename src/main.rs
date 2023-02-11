use qrtdb::qrtlib::help;
use qrtdb::{Database, MetaCommands};

mod file_test_old;
mod db4;
mod field_types;

use db4::db4::rundb4;

fn main() {
    file_test_old::run_file_test();

    rundb4();
    return;
    use std::io::stdin;
    let mut line = String::new();

    let mut db = Database::new();

    loop {
        println!("SQRite > ");
        stdin().read_line(&mut line).unwrap();
        // process line
        if line.contains(".") {
            let mc = MetaCommands::from(&line);
            match mc {
                MetaCommands::EXIT => return,
                MetaCommands::HELP => db.help(),
                MetaCommands::TABLES => db.lstables(),
                MetaCommands::UnrecognizedCommand => {
                    println!("Unrecognized meta command")
                }
            }
            line.truncate(0);
            continue;
        }
        db.process_statement(&line);
        line.truncate(0);
        // break;
    }
}
