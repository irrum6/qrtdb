pub mod tcolumn {
    use crate::qrtlib::field_types::FieldTypes;
    #[derive(Clone)]
    pub struct TableColumn {
        name: String,
        data_type: FieldTypes,
    }
    impl TableColumn {
        pub fn new(name: String, data_type: FieldTypes) -> Option<TableColumn> {
            let taco = TableColumn { name, data_type };
            return Some(taco);
        }

        pub fn new2(name: String, data_type: FieldTypes) -> TableColumn {
            return TableColumn { name, data_type };
        }

        pub fn from_text(input: &str) -> Option<TableColumn> {
            if input.is_empty() {
                return None;
            }
            let split: Vec<&str> = input.split(" ").collect();

            if split.len() < 2 {
                println!("no type was provided for column");
                return None;
            }
            return TableColumn::from(split[0], split[1]);
        }

        pub fn from(name: &str, ftype: &str) -> Option<TableColumn> {
            if name == "" {
                return None;
            }
            if let Some(data_type) = FieldTypes::from(ftype) {
                return Some(TableColumn {
                    name: String::from(name),
                    data_type,
                });
            } else {
                return None;
            }
        }
        pub fn data_type_ref(&self) -> &FieldTypes {
            return &self.data_type;
        }

        pub fn name(&self) -> String {
            return self.name.clone();
        }
        pub fn equal(t: TableColumn, t2: TableColumn) -> bool {
            return t.typef() == t2.typef();
        }

        pub fn self_equal(&self, tc: TableColumn) -> bool {
            return self.data_type == tc.typef();
        }
        pub fn typef(&self) -> FieldTypes {
            return self.data_type.clone();
        }
    }
}
