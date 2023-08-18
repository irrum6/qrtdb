pub mod char{
    use crate::qrtlib::statements::WhereClauses;
    
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

    
}