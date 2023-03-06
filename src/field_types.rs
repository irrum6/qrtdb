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

        pub fn create_with_value(fieldname: &str, value: &str) -> Option<FieldTypes> {
            //  n i s v f d
            return match fieldname {
                "number" | "num" => {
                    let x: f64 = value.parse().unwrap();
                    Some(FieldTypes::Number(x))
                }
                "integer" | "int" => {
                    let x: u64 = value.parse().unwrap();
                    Some(FieldTypes::Integer(x))
                }
                "sigint" | "sig" => {
                    let x: i64 = value.parse().unwrap();
                    Some(FieldTypes::SignedInteger(x))
                }
                "varchar" | "vchar" => {
                    let vchar = Varchar::new(24, String::from(value));
                    Some(FieldTypes::Varchar(vchar))
                }
                "fxchar" | "fixchar" | "fchar" => {
                    let fchar = Fixedchar::new(24, String::from(value));
                    Some(FieldTypes::Fxchar(fchar))
                }
                "date" => {
                    let x: u64 = value.parse().unwrap();
                    Some(FieldTypes::Date(x))
                }
                _ => None,
            };
        }

        /**
         * Create with value type assisted
         * instead of calling tostring in table and parsing here
         * we pass reference and construct new with new value
         */
        pub fn create_with_value_ta_(ft: &FieldTypes, value: &str) -> Option<FieldTypes> {
            // println!("{} {}",FieldTypes::describe(ft),value);
            return match ft {
                FieldTypes::Number(_) => {
                    let x: f64 = value.parse().unwrap();
                    Some(FieldTypes::Number(x))
                }
                FieldTypes::Integer(_) => {
                    let x: u64 = value.parse().unwrap();
                    Some(FieldTypes::Integer(x))
                }
                FieldTypes::SignedInteger(_) => {
                    let x: i64 = value.parse().unwrap();
                    Some(FieldTypes::SignedInteger(x))
                }
                FieldTypes::Varchar(_) => {
                    let vchar = Varchar::new(24, String::from(value));
                    Some(FieldTypes::Varchar(vchar))
                }
                FieldTypes::Fxchar(_) => {
                    let fchar = Fixedchar::new(24, String::from(value));
                    Some(FieldTypes::Fxchar(fchar))
                }
                FieldTypes::Date(_) => {
                    let x: u64 = value.parse().unwrap();
                    Some(FieldTypes::Date(x))
                }

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
        
        pub fn to2(f: FieldTypes) -> String {
            return match f {
                FieldTypes::Number(_) => String::from("num"),
                FieldTypes::Integer(_) => String::from("int"),
                FieldTypes::SignedInteger(_) => String::from("sig"),
                FieldTypes::Varchar(_) => String::from("vchar"),
                FieldTypes::Fxchar(_) => String::from("fchar"),
                FieldTypes::Date(_) => String::from("date"),
            };
        }
        

        pub fn tostr(f: &FieldTypes) -> String {
            return match f {
                FieldTypes::Number(v) => v.to_string(),
                FieldTypes::Integer(v) => v.to_string(),
                FieldTypes::SignedInteger(v) => v.to_string(),
                FieldTypes::Varchar(v) => v.get(),
                FieldTypes::Fxchar(v) => v.get(),
                FieldTypes::Date(v) => v.to_string(),
            };
        }
        /**
        .give string description of field types
        */
        pub fn describe(f: &FieldTypes) -> String {
            return match f {
                FieldTypes::Number(_) => String::from("number:64bit float"),
                FieldTypes::Integer(_) => String::from("integer:64 bit unsigned integer"),
                FieldTypes::SignedInteger(_) => String::from("sigint:64 bit unsigned integer"),
                FieldTypes::Varchar(_) => String::from("varchar:variable length string"),
                FieldTypes::Fxchar(_) => String::from("fxchar:fixed length string"),
                FieldTypes::Date(_) => String::from("date:64 bit unsigned integer"),
            };
        }

        pub fn serialize(t: FieldTypes) -> Vec<u8> {
            return match t {
                FieldTypes::Number(num) => num.to_be_bytes().to_vec(),
                FieldTypes::Integer(int) => int.to_be_bytes().to_vec(),
                FieldTypes::Varchar(var) => var.get().into_bytes(),
                FieldTypes::Fxchar(var) => var.get().into_bytes(),
                FieldTypes::Date(int) => int.to_be_bytes().to_vec(),
                FieldTypes::SignedInteger(int) => int.to_be_bytes().to_vec(),
            };
        }

        pub fn deserialize(f: FieldTypes, v: Vec<u8>) -> FieldTypes {
            let ftype: FieldTypes = match f {
                FieldTypes::Number(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Number(f64::from_be_bytes(x))
                }
                FieldTypes::Integer(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Integer(u64::from_be_bytes(x))
                }
                FieldTypes::SignedInteger(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::SignedInteger(i64::from_be_bytes(x))
                }
                FieldTypes::Varchar(vchar) => FieldTypes::Varchar(Varchar::new(vchar.len(), String::from_utf8(v).unwrap())),
                FieldTypes::Fxchar(vchar) => FieldTypes::Fxchar(Fixedchar::new(vchar.len(), String::from_utf8(v).unwrap())),
                FieldTypes::Date(_) => {
                    let mut x: [u8; 8] = [0; 8];
                    for i in 0..8 {
                        x[i] = v[i];
                    }
                    FieldTypes::Date(u64::from_be_bytes(x))
                }
            };
            return ftype;
            // return TableField::new("age", "integer");
        }
    }
}
