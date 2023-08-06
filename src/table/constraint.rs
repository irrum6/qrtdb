pub mod constraint {
    #[derive(Clone, PartialEq)]
    pub enum ConstraintTypes {
        PrimaryKey,
        ForeignKey,
        ColumnMatch,
        Unique,
        NoConstraint,
    }
    impl ConstraintTypes {
        pub fn from(ctype: &str) -> ConstraintTypes {
            return match ctype {
                "matches" | "ma" | "m" => ConstraintTypes::ColumnMatch,
                "primary" | "pk" | "p" => ConstraintTypes::PrimaryKey,
                "foreign" | "fk" | "f" => ConstraintTypes::ForeignKey,
                "unique" | "uq" | "u" => ConstraintTypes::Unique,
                "==>" => ConstraintTypes::NoConstraint,
                _ => ConstraintTypes::NoConstraint,
            };
        }
        pub fn to(ct: &ConstraintTypes) -> String {
            return match ct {
                ConstraintTypes::ColumnMatch => String::from("=m>"),
                ConstraintTypes::PrimaryKey => String::from("=p>"),
                ConstraintTypes::ForeignKey => String::from("=f>"),
                ConstraintTypes::Unique => String::from("=u>"),
                ConstraintTypes::NoConstraint => String::from("==>"),
            };
        }
        pub fn describe(ct: &ConstraintTypes) -> String {
            return match ct {
                ConstraintTypes::ColumnMatch => String::from("Column Match"),
                ConstraintTypes::PrimaryKey => String::from("Primary Key"),
                ConstraintTypes::ForeignKey => String::from("Foreign Key"),
                ConstraintTypes::Unique => String::from("Unique"),
                ConstraintTypes::NoConstraint => String::from("NoConstraint"),
            };
        }
    }
    #[derive(Clone)]
    pub struct Constraint {
        ctype: ConstraintTypes,
        column: String,
        pub ref_table: String,
        pub ref_column: String,
    }
    impl Constraint {
        pub fn new(ct: &str, col: &str, reft: &str, refcol: &str) -> Constraint {
            let ctype = ConstraintTypes::from(ct);
            let column = String::from(col);
            let ref_table = String::from(reft);
            let ref_column = String::from(refcol);
            return Constraint {
                ctype,
                column,
                ref_table,
                ref_column,
            };
        }
        pub fn construct_primary_key(name: &str) -> Constraint {
            return Constraint {
                ctype: ConstraintTypes::PrimaryKey,
                column: String::from(name),
                ref_table: String::new(),
                ref_column: String::new(),
            };
        }
        pub fn ct(&self) -> ConstraintTypes {
            return self.ctype.clone();
        }

        pub fn col(&self) -> String {
            return self.column.clone();
        }

        pub fn col_as_ref(&self) -> &String {
            return &self.column;
        }

        pub fn refta(&self) -> &String {
            return &self.ref_table;
        }

        pub fn refcol(&self) -> &String {
            return &&self.ref_column;
        }

        pub fn ct_from(token: &str) -> Option<Constraint> {
            if token.is_empty() {
                return None;
            }

            let split: Vec<String> = token.trim().split(" ").map(|e| String::from(e)).collect();

            if split.len() < 2 {
                return None;
            }

            let column = split[0].clone();

            let ctype = ConstraintTypes::from(&split[1]);

            match ctype {
                ConstraintTypes::Unique | ConstraintTypes::PrimaryKey => {
                    return Some(Constraint {
                        ctype,
                        column,
                        ref_table: String::new(),
                        ref_column: String::new(),
                    });
                }
                ConstraintTypes::NoConstraint => {
                    //and then will ignore it
                    return None;
                }
                ConstraintTypes::ForeignKey | ConstraintTypes::ColumnMatch => {
                    if split.len() < 3 {
                        println!("bad reference");
                        return None;
                    }
                    let references: Vec<String> = split[2].split(".").map(|e| String::from(e)).collect();
                    let ref_table = references[0].clone();
                    let ref_column = references[1].clone();

                    return Some(Constraint {
                        ctype,
                        column,
                        ref_table,
                        ref_column,
                    });
                }
            }
        }

        pub fn from_token(col: &str, token: &str) -> Option<Constraint> {
            let mut pat = "";

            if token.contains("=u>") {
                pat = "=u>";
            }

            if token.contains("=p>") {
                pat = "=p>";
            }

            if token.contains("=p>") || token.contains("=u>") {
                let ctype = ConstraintTypes::from(pat);
                return Some(Constraint {
                    ctype,
                    column: String::from(col),
                    ref_table: String::new(),
                    ref_column: String::new(),
                });
            }
            if token.contains("=m>") {
                pat = "=m>";
            }

            if token.contains("=f>") {
                pat = "=f>";
            }

            if token.contains("=fk>") {
                pat = "=fk>";
            }
            if pat.len() > 0 {
                let column = String::from(col);
                let split: Vec<String> = token.trim().replace(pat, "").split(".").map(|e| String::from(e)).collect();
                if split.len() != 2 {
                    println!("bad reference");
                    return None;
                }
                let ref_table = split[0].clone();
                let ref_column = split[1].clone();
                let ctype = ConstraintTypes::from(pat);

                return Some(Constraint {
                    ctype,
                    column,
                    ref_table,
                    ref_column,
                });
            }
            return None;
        }
    }
}
