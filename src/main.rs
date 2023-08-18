// use qrtdb::qrtlib::help;
// use qrtdb::{Database, MetaCommands};

use qrtdb::qrtlib::rundb4;
mod file_test_old;

use qrtdb::qrtlib::field_types::DateFieldCustom;

mod query;
fn main() {
    DateFieldCustom::test_sede();
    rundb4();
    return;
}
