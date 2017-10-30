use chrono::DateTime;
use chrono::prelude::{Local, Utc};
use errors::*;
use nom::{alphanumeric, IResult};
use output::Output;
use std::str;

#[derive(Debug, PartialEq, Eq)]
struct Program {
    steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq)]
struct Step {
    op: MacroOp,
    args: Vec<String>
}

#[derive(Debug, PartialEq, Eq)]
enum MacroOp {
    Add,
    Divide,
    Multiply,
    Name(String),
    Roll,
    Say,
    Subtract,
    Whisper,
}

#[derive(Debug, PartialEq, Eq)]
enum RollOp {
    Advantage,
    Disadvantage,
    NdD,
}

#[derive(Debug, PartialEq, Eq)]
enum SayOp {
    Message,
    To,
}

named!(command <&[u8], MacroOp>, alt!(
    map!(tag!("!roll"),     |_| MacroOp::Roll)      |
    map!(tag!("!r"),        |_| MacroOp::Roll)      |
    map!(tag!("!say"),      |_| MacroOp::Say)       |
    map!(tag!("!s"),        |_| MacroOp::Say)       |
    map!(tag!("!whisper"),  |_| MacroOp::Whisper)   |
    map!(tag!("!w"),        |_| MacroOp::Whisper)
));

named!(name <&[u8], MacroOp>, do_parse!(
    tag!("#") >>
    name: map_res!(is_not!(" \t\r\n"), |r: &[u8]| String::from_utf8(r.to_vec())) >>
    (MacroOp::Name(name))
));

named!(primitive <&[u8], MacroOp>, alt!(
    map!(tag!("*"), |_| MacroOp::Multiply)  |
    map!(tag!("+"), |_| MacroOp::Add)       |
    map!(tag!("-"), |_| MacroOp::Subtract)  |
    map!(tag!("/"), |_| MacroOp::Divide)
));


#[test]
fn test_name() {
    let (_, result) = name(b"#macro_name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro_name")));

    let bad_result = name(b"macro_name");
}
