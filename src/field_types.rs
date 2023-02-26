pub mod field_types {
    use crate::qrtlib::WhereClauses;

    #[derive(Clone, PartialEq)]
    pub struct Varchar {
        length: u64,
        value: String,
    }

    impl Varchar {
        pub fn new(length: u64, v: String) -> Varchar {
            if v.len() > length as usize {}
            return Varchar { length, value: v };
        }
        pub fn get(&self) -> String {
            return self.value.clone();
        }
        pub fn len(&self) -> u64 {
            return self.length;
        }
        pub fn compare(&self, other: String, w: WhereClauses) -> bool {
            return match w {
                WhereClauses::Equal => self.value == other,
                WhereClauses::NonEqual => self.value != other,
                WhereClauses::Greater => self.value > other,
                WhereClauses::Less => self.value < other,
                WhereClauses::Empty => true,
            };
        }
    }
    #[derive(Clone, PartialEq)]
    pub struct Fixedchar {
        length: u64,
        value: String,
    }
    impl Fixedchar {
        pub fn new(length: u64, v: String) -> Fixedchar {
            if v.len() > length as usize {}
            return Fixedchar { length, value: v };
        }
        pub fn get(&self) -> String {
            return self.value.clone();
        }
        pub fn len(&self) -> u64 {
            return self.length;
        }
        pub fn compare(&self, other: String, w: WhereClauses) -> bool {
            // let chars: Vec<char> = self.value.chars().collect();
            // let otherchars: Vec<char> = self.value.chars().collect();
            return match w {
                WhereClauses::Equal => self.value == other,
                WhereClauses::NonEqual => self.value != other,
                WhereClauses::Greater => self.value > other,
                WhereClauses::Less => self.value < other,
                WhereClauses::Empty => true,
            };
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum FieldTypes {
        Number(f64),
        Integer(u64),
        SignedInteger(i64),
        Varchar(Varchar),
        Fxchar(Fixedchar),
        Date(u64),
    }

    impl FieldTypes {
        pub fn from(fieldname: &str) -> Option<FieldTypes> {
            return match fieldname {
                "number" | "num" => Some(FieldTypes::Number(0.0)),
                "integer" | "int" => Some(FieldTypes::Integer(0)),
                "sigint" | "sig" => Some(FieldTypes::SignedInteger(0)),
                "varchar" | "vchar" => {
                    let vchar = Varchar::new(24, String::new());
                    Some(FieldTypes::Varchar(vchar))
                }
                "fxchar" | "fixchar" | "fchar" => {
                    let fchar = Fixedchar::new(24, String::new());
                    Some(FieldTypes::Fxchar(fchar))
                }
                "date" => Some(FieldTypes::Date(0)),
                _ => None,
            };
        }
        
        pub fn to(f: FieldTypes) -> String {
            return match f {
                FieldTypes::Number(_) => String::from("number"),
                FieldTypes::Integer(_) => String::from("integer"),
                FieldTypes::SignedInteger(_) => String::from("sigint"),
                FieldTypes::Varchar(_) => String::from("varchar"),
                FieldTypes::Fxchar(_) => String::from("fxchar"),
                FieldTypes::Date(_) => String::from("date"),
            };
        }
        /**
        .give string description of field types
        */
        pub fn describe(f: FieldTypes) -> String {
            return match f {
                FieldTypes::Number(_) => String::from("number:64bit float"),
                FieldTypes::Integer(_) => String::from("integer:64 bit unsigned integer"),
                FieldTypes::SignedInteger(_) => String::from("sigint:64 bit unsigned integer"),
                FieldTypes::Varchar(_) => String::from("varchar:variable length string"),
                FieldTypes::Fxchar(_) => String::from("fxchar:fixed length string"),
                FieldTypes::Date(_) => String::from("date:64 bit unsigned integer"),
            };
        }
    }
}
