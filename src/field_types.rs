pub mod field_types {
    #[derive(Clone)]
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
        pub fn len(&self)->u64{
            return self.length;
        }
    }
    #[derive(Clone)]
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
        pub fn len(&self)->u64{
            return self.length;
        }
    }

    #[derive(Clone)]
    pub enum FieldTypes {
        Number(f64),
        Integer(u64),
        SignedInteger(i64),
        Varchar(Varchar),
        Fxchar(Fixedchar),
        Date(u64),
    }

    impl FieldTypes {
        pub fn from(fieldname: &str) -> FieldTypes {
            return match fieldname {
                "number" | "num" => FieldTypes::Number(0.0),
                "integer" | "int" => FieldTypes::Integer(0),
                "sigint" | "sig" => FieldTypes::Integer(0),
                "varchar" | "vchar" => FieldTypes::Varchar(Varchar::new(24, String::new())),
                "fxchar" | "fchar" => FieldTypes::Fxchar(Fixedchar::new(24, String::new())),
                "date" => FieldTypes::Date(0),
                _ => FieldTypes::Integer(0),
            };
        }
    }
}
