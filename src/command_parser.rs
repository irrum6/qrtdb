pub mod command_parser {
    pub enum MetaCommands {
        EXIT,
        TABLES,
        HELP,
        UnrecognizedCommand,
    }
    impl MetaCommands {
        pub fn from(s: &str) -> MetaCommands {
            let st:Vec<&str> = s.trim().split(" ").collect();
            return match st[0] {
                ".exit" | ".EXIT" | ".X" | ".x" => MetaCommands::EXIT,
                ".help" | ".HELP" | ".H" | ".h" => MetaCommands::HELP,
                ".tables" | ".TABLES" | ".T" | ".t" => MetaCommands::TABLES,
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