pub mod command_parser{
    pub enum MetaCommands {
        EXIT,
        TABLES,
        HELP,
        UnrecognizedCommand,
    }
    impl MetaCommands {
        pub fn from(s: &str) -> MetaCommands {
            let st = s.trim();
            return match st {
                ".exit" | ".EXIT" | ".X" => MetaCommands::EXIT,
                ".help" | ".HELP" | ".H" => MetaCommands::HELP,
                ".tables" | ".TABLES" | ".T" => MetaCommands::TABLES,
                //ls databases
                //in base
                //ls namespacs
                //in base ls tables
                //print namespace:tablename
                _ => MetaCommands::UnrecognizedCommand,
            };
        }
    }
}