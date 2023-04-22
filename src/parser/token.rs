pub mod token {
    use nom::{
        branch::alt,
        bytes,
        bytes::complete::{is_not, tag, tag_no_case, take_till, take_until, take_while, take_while1},
        character::complete::{self, char as ncchar, line_ending, multispace0, newline},
        character::{is_alphabetic, is_newline, is_space},
        combinator::{self, all_consuming, map, map_parser, opt, recognize},
        error::context,
        multi::separated_list1,
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };
    use std::{
        fs::File,
        io::{Error as IOError, Read},
    };

    use crate::qrtlib::{self};

    // use crate::qrtlib::stmnt2;

    use crate::qrtlib::statements::{DDLTypes, DMLTypes, PrepareResult, Statement, StatementCategory};

    use crate::qrtlib::Database4;

    #[derive(Debug, PartialEq, Eq)]
    pub enum PrimaryExpression<'a> {
        // @
        ObjectId(&'a str),
        // [] Where
        Criteria(&'a str),
        // {}
        Limit(&'a str),
        // \//
        Comment(&'a str),
        // #
        Add(&'a str),
        // $
        Read(&'a str),
        // *
        Change(&'a str),
        // \!!
        Remove,
        // #d d#
        AddDatabase(&'a str),
        // #n n#
        AddNamespace(&'a str),
        // #tt#
        AddTable(&'a str),
        // #table table#
        AddTable2(&'a str),
        // *tt*
        ChangeTable(&'a str),
        // *table table*
        ChangeTable2(&'a str),
        // \!tt!
        RemoveTable(&'a str),
        // \!table!
        RemoveTable2(&'a str),
        Text(&'a str),
    }

    // recognizes id
    fn parse_id(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                // eat space
                multispace0,
                tag("@"),
                combinator::map(take_until(" "), |inner: &str| PrimaryExpression::ObjectId(inner)),
            )),
            |x| x.2,
        )(input)
    }

    // add table directive
    fn add_database(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag_no_case("#d"),
                combinator::map(take_until("d#"), |inner: &str| PrimaryExpression::AddDatabase(inner)),
                tag_no_case("d#"),
            )),
            |x| x.1,
        )(input)
    }
    // add table directive
    fn add_namespace(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag_no_case("#n"),
                combinator::map(take_until("n#"), |inner: &str| PrimaryExpression::AddNamespace(inner)),
                tag_no_case("n#"),
            )),
            |x| x.1,
        )(input)
    }

    // add table directive
    fn add_table_dir(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag_no_case("#t"),
                combinator::map(take_until("t#"), |inner: &str| PrimaryExpression::AddTable(inner)),
                tag_no_case("t#"),
            )),
            |x| x.1,
        )(input)
    }
    fn change_table(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag_no_case("*t"),
                combinator::map(take_until("*#"), |inner: &str| PrimaryExpression::ChangeTable(inner)),
                tag_no_case("t*"),
            )),
            |x| x.1,
        )(input)
    }
    // add record directive
    fn add_record_dir(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                multispace0,
                tag("# "),
                combinator::map(take_until(" #"), |inner: &str| PrimaryExpression::Add(inner)),
                tag(" #"),
            )),
            |x| x.2,
        )(input)
    }

    // read record directive
    fn read_record_dir(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                multispace0,
                tag("$ "),
                // multispace1,
                combinator::map(take_until(" $"), |inner: &str| PrimaryExpression::Read(inner)),
                // multispace1,
                tag(" $"),
            )),
            //position of tuple where PrimaryExpression lives
            |x| x.2,
        )(input)
    }
    // change record directive
    fn change_record(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag("* "),
                combinator::map(take_until(" *"), |inner: &str| PrimaryExpression::Change(inner)),
                tag(" *"),
            )),
            |x| x.1,
        )(input)
    }

    // remove record directive
    // fn remove_record(input: &str) -> IResult<&str, PrimaryExpression> {
    //     tuple((tag("!!"), PrimaryExpression::Remove))(input)
    // }

    fn is_dml(input: &str) -> IResult<&str, PrimaryExpression> {
        //if is either read or adding reacords
        //alt needs one argument wrap
        alt((read_record_dir, add_record_dir, change_record))(input)
    }

    fn is_ddl(input: &str) -> IResult<&str, PrimaryExpression> {
        alt((add_database, add_namespace, add_table_dir, change_table))(input)
    }
    fn is_noun(input: &str) -> IResult<&str, PrimaryExpression> {
        alt((is_ddl, is_dml))(input)
    }

    // query criteria[]
    fn criteria(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag("["),
                combinator::map(take_until("]"), |inner: &str| PrimaryExpression::Criteria(inner)),
                tag("]"),
            )),
            |x| x.1,
        )(input)
    }

    //query limit {3}
    fn limit(input: &str) -> IResult<&str, PrimaryExpression> {
        combinator::map(
            tuple((
                tag("{"),
                combinator::map(take_until("]"), |inner: &str| PrimaryExpression::Limit(inner)),
                tag("}"),
            )),
            |x| x.1,
        )(input)
    }

    //is one of the primary expressions
    fn prexp(input: &str) -> IResult<&str, PrimaryExpression> {
        //alt takes noun
        alt((is_noun, parse_id, criteria, limit))(input)
    }

    // fn whole_statement(input: &str) -> IResult<&str, PrimaryExpression> {
    //     //whoah i get it now
    //     //but it doesn't work
    //     terminated(
    //         // tuple((alt((is_dml, is_ddl)), parse_id)),
    //         //spaces
    //         delimited(multispace0, alt((is_dml, is_ddl)), multispace0),
    //         ncchar(';'),
    //         // tag(";")
    //     )(input)
    // }

    fn ok_content(){

    }
    fn process_content(input: &str) -> Statement {
        let mut emsta = Statement::empty();
        let mut proc_input = input;

        loop {
            // break;
            let mut exit = false;
            match prexp(&proc_input) {
                Ok((rem, token)) => {
                    println!("parsed {:?}", token);

                    match token {
                        PrimaryExpression::AddNamespace(s) => {
                            emsta.set_category(StatementCategory::DDL(DDLTypes::AddNamespace));
                            emsta.add_verb(s.to_string());
                        }
                        PrimaryExpression::AddTable(s) => {
                            emsta.set_category(StatementCategory::DDL(DDLTypes::AddTable));
                            emsta.add_verb(s.to_string());
                        }
                        PrimaryExpression::Add(s) => {
                            println!("additio");
                            emsta.set_category(StatementCategory::DML(DMLTypes::ADD));
                            emsta.add_verb(s.to_string());
                        }

                        PrimaryExpression::Read(s) => {
                            println!("readitio");
                            emsta.set_category(StatementCategory::DML(DMLTypes::READ));
                            emsta.add_verb(s.to_string());
                        }

                        PrimaryExpression::ObjectId(s) => {
                            println!("object");
                            //need to split here
                            let strabons = s.split("::");
                            for s in strabons {
                                emsta.add_noun(s.to_string());
                            }
                        }
                        _ => {
                            println!("Other expression");
                        }
                    }
                    println!("------");
                    println!("remaining {}", rem);
                    if rem.is_empty() {
                        break;
                    }
                    if rem == " " {
                        println!("last space");
                        break;
                    }
                    proc_input = rem;
                    continue;
                }
                Err(nom::Err::Error(ne)) => {
                    println!("Nom error");
                    println!("{:?}", ne);
                    break;
                }
                Err(e) => {
                    println!("Other error");
                    break;
                }
            }
        }
        return emsta;
    }

    fn read_till_semicolon(input: &str) -> IResult<&str, Statement> {
        combinator::map(
            tuple((
                combinator::map(take_until(";"), |inner: &str| process_content(inner)),
                tag(";"),
            )),
            |x| x.0,
        )(input)
    }
    //Vec<stmnt2::Statement>

    pub fn whole_statement2(input: &str) -> IResult<&str, Statement> {
        // combinator::map(
        //     tuple((
        //         multispace0,
        //         combinator::map(take_until("]"), |inner: &str| PrimaryExpression::Text(inner)),
        //     )),
        //     |x| x.1,
        // )(input)
        preceded(multispace0, read_till_semicolon)(input)
    }

    fn process3(input: &str, db: &Database4) -> Vec<Statement> {
        let mut proc_input = input;

        let mut statos: Vec<Statement> = Vec::new();
        loop {
            // break;
            let mut exit = false;
            match whole_statement2(&proc_input) {
                Ok((rem, token)) => {
                    println!("parsed {:?}", token);
                    println!("------");
                    println!("remaining {}", rem);
                    if rem.is_empty() {
                        break;
                    }
                    proc_input = rem;
                    statos.push(token);
                    continue;
                }
                Err(nom::Err::Error(ne)) => {
                    println!("Nom error");
                    println!("{:?}", ne);
                    break;
                }
                Err(e) => {
                    println!("Other error");
                    break;
                }
            }
        }
        return statos;
    }

    pub fn read2(s: &String, db: &mut Database4) -> Result<String, IOError> {
        let x: Vec<&str> = s.trim().split(" ").collect();

        if x.len() > 1 && x[1] != "" {
            let mut line = String::new();
            File::open(x[1])?.read_to_string(&mut line)?;
            // self.process_statement(&line);
            // print!("{}", &line);
            // process(&line);
            // process2(&line);
            let states = process3(&line, db);

            for mut st in states {
                let result = st.prepare2();
                match result {
                    PrepareResult::UnrecognizedStatement => {
                        println!("read2::process_statement > Some of the statements failed, aborting");
                        break;
                    }
                    PrepareResult::SUCCESS => {
                        // execute staments
                        db.execute(st);
                    }
                };
            }
        }

        return Ok(String::from("success"));
    }

}
