pub mod record {
    use crate::qrtlib::field_types::FieldTypes;

    use crate::table::table::Table;

    #[derive(Clone, PartialEq)]
    pub struct RecordValue {
        pub(super) value: FieldTypes,
    }
    impl RecordValue {
        pub fn new(value: FieldTypes) -> RecordValue {
            return RecordValue { value };
        }
        pub fn to_string(&self) -> String {
            return FieldTypes::tostr(&self.value);
        }
        pub fn set(&mut self, r: FieldTypes) {
            self.value = r;
        }
        pub fn get(&self) -> FieldTypes {
            return self.value.clone();
        }
        pub fn get_referenced(&self) -> &FieldTypes {
            return &self.value;
        }

        pub fn from(f: String) -> Option<RecordValue> {
            if let Some(ftype) = FieldTypes::from(&f) {
                return Some(RecordValue { value: ftype });
            }
            return None;
        }
    }
    pub struct Record {
        pub(crate) fields: Vec<RecordValue>,
    }

    impl Record {
        pub fn new(fields: Vec<RecordValue>) -> Record {
            return Record { fields };
        }
        pub fn empty() -> Record {
            let fields: Vec<RecordValue> = Vec::new();
            return Record { fields };
        }

        pub fn get(&self, index: usize) -> Option<RecordValue> {
            if index > self.fields.len() {
                return None;
            }
            return Some(self.fields[index].clone());
        }
        pub fn set(&mut self, index: usize, v: &String) {
            if index > self.fields.len() {
                return;
            }
            // self.fields[index].set(v);
        }
        pub fn serialize_record(record: Record) -> Vec<u8> {
            let data: Vec<u8> = Vec::new();
            return data;
        }
        pub fn deserialize_record(data: Vec<u8>, table: Table) -> Record {
            let fields: Vec<RecordValue> = Vec::new();
            return Record { fields };
        }
    }
}
