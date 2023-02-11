pub mod field_types {
    pub struct Number {
        value: f64,
    }

    impl Number {
        fn new(v: f64) -> Number {
            return Number { value: v };
        }
    }

    pub struct Integer {
        value: u64,
    }
    impl Integer {
        fn new(v: u64) -> Integer {
            return Integer { value: v };
        }
    }

    pub struct Varchar {
        length: u64,
        value: String,
    }

    impl Varchar {
        fn new(length: u64, v: &str) -> Varchar {
            let value = String::from(v);
            if value.len() > length as usize {}
            return Varchar { length, value };
        }
    }

    pub struct Fixedchar {
        length: u64,
        value: String,
    }
    impl Fixedchar {
        fn new(length: u64, v: &str) -> Fixedchar {
            let value = String::from(v);
            if value.len() > length as usize {}
            return Fixedchar { length, value };
        }
    }

    pub struct Date {
        value: u64,
    }

    impl Date {
        fn new(v: u64) -> Date {
            return Date { value: v };
        }
    }

    pub enum FieldTypes {
        Number(Number),
        Integer(Integer),
        Varchar(Varchar),
        Fxchar(Fixedchar),
        Date(Date),
    }

    impl FieldTypes {
        pub fn from(fieldname: &str) -> FieldTypes {
            return match fieldname {
                "number" | "num" => FieldTypes::Number(Number::new(0.0)),
                "integer" | "int" => FieldTypes::Integer(Integer::new(0)),
                "varchar" | "vchar" => FieldTypes::Varchar(Varchar::new(24, "")),
                "fxchar" | "fchar" => FieldTypes::Fxchar(Fixedchar::new(24, "")),
                "date" => FieldTypes::Date(Date::new(0)),
                _ => FieldTypes::Integer(Integer::new(0)),
            };
        }
    }
}
