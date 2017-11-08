// @todo - It would be nice to break each parser into it's own module
// e.g. parser::roll, parser::say, parser::core

use nom::{alphanumeric, digit, ErrorKind, IResult};
use nom::simple_errors::Err;
use std::str;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub name: MacroOp,
    pub steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub args: Vec<Arg>,
    pub op: MacroOp,
    pub result: StepResult,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroOp {
    /// Addition (+)
    Add,
    /// Division (/)
    Divide,
    /// Multiplication (*)
    Multiply,
    /// Macro Name
    Name(String),
    /// Prompt (!prompt)
    Prompt,
    /// Roll (!roll)
    Roll,
    /// Say (!say)
    Say,
    /// Subtraction (-)
    Subtract,
    /// Whisper (!whisper)
    Whisper,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Pass Result (>>)
    Pass,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arg {
    /// Number (Float, Integer)
    Number(u32),
    /// Unrecognized argument
    Unrecognized(String),
    /// Roll arguments
    Roll(RollArg),
    /// Say arguments
    Say(SayArg),
    /// Static variable ($)
    Variable(String),
}

// Arguments for the roll command, used by the parser
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollArg {
    Advantage,
    Comment(String),
    Disadvantage,
    D(u8), // e.g. d20
    E(i16),
    H(i16),
    K,
    L(i16),
    N(u8), // e.g. 1 (part of 1d20)
    RO(i16),
    RR(i16),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SayArg {
    Message(String),
    To(String),
}

/// Matches advantage roll argument
pub fn advantage_p(input: &[u8]) -> IResult<&[u8], Arg> {
    map!(input, alt_complete!(tag!("advantage") | tag!("adv")), |_| Arg::Roll(RollArg::Advantage))
}

/// Matches arguments of unknown commands
pub fn arguments_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        num |
        map!(string, | a | Arg::Unrecognized(a)) |
        map!(quoted, | a | Arg::Unrecognized(a)) |
        map!(single_quoted, | a | Arg::Unrecognized(a)) |
        map!(variable, | a | Arg::Variable(a))
    )
}

/// Matches !roll arguments
pub fn arguments_roll_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        advantage |
        disadvantage |
        roll_num |
        roll_die |
        roll_flag_e |
        roll_flag_h |
        roll_flag_k |
        roll_flag_l |
        roll_flag_ro |
        roll_flag_rr |
        map!(quoted,        | a | Arg::Roll(RollArg::Comment(a))) |
        map!(single_quoted, | a | Arg::Roll(RollArg::Comment(a))) |
        map!(variable,      | a | Arg::Variable(a))
    )
}

/// Matches !say arguments
pub fn arguments_say_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(string, | a | Arg::Say(SayArg::Message(a))) |
        map!(quoted, | a | Arg::Say(SayArg::Message(a))) |
        map!(single_quoted, | a | Arg::Say(SayArg::Message(a))) |
        map!(variable, | a | Arg::Variable(a))
    )
}

/// Matches !whisper arguments
pub fn arguments_whisper_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(variable, | a | Arg::Say(SayArg::To(a))) |
        map!(string, | a | Arg::Say(SayArg::Message(a))) |
        map!(quoted, | a | Arg::Say(SayArg::Message(a))) |
        map!(single_quoted, | a | Arg::Say(SayArg::Message(a)))
    )
}

/// Matches any command
pub fn command_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    add_return_error!(input, ErrorKind::Custom(2), alt!(
        map!(ws!(tag!("!roll")),     |_| MacroOp::Roll)      |
        map!(ws!(tag!("!r")),        |_| MacroOp::Roll)      |
        map!(ws!(tag!("!say")),      |_| MacroOp::Say)       |
        map!(ws!(tag!("!s")),        |_| MacroOp::Say)       |
        map!(ws!(tag!("!whisper")),  |_| MacroOp::Whisper)   |
        map!(ws!(tag!("!w")),        |_| MacroOp::Whisper)
    ))
}

/// Matches disadvantage roll argument
pub fn disadvantage_p(input: &[u8]) -> IResult<&[u8], Arg> {
    map!(input, alt_complete!(tag!("disadvantage") | tag!("dis")), |_| Arg::Roll(RollArg::Disadvantage))
}

/// Matches a macro name
pub fn name_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    add_return_error!(input, ErrorKind::Custom(1), ws!(
        do_parse!(
            tag!("#") >>
            name: map_res!(is_not!(" \t\r\n"), |r: &[u8]| String::from_utf8(r.to_vec())) >>
            (MacroOp::Name(name))
        )
    ))
}

/// Match numbers to argument strings
pub fn num_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        num: ws!(digit) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Number(s.parse::<u32>().unwrap()))
    )
}

/// Matches any type of operation
pub fn op_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    alt!(input,
        name |
        command |
        ws!(primitive)
    )
}

/// Parse the complete macro
pub fn parse_p(input: &[u8]) -> IResult<&[u8], Program> {
    do_parse!(input,
        prog_name: name >>
        steps: many0!(parse_step) >>
        (Program {
            name: prog_name,
            steps: steps,
        })
    )
}

/// Parse a step of the program
pub fn parse_step_p(input: &[u8]) -> IResult<&[u8], Step> {
    do_parse!(input,
        op_type: op >>
        args: many0!(switch!(value!(&op_type),
            &MacroOp::Roll => call!(arguments_roll) |
            &MacroOp::Say => call!(arguments_say) |
            &MacroOp::Whisper => call!(arguments_whisper) |
            _ => call!(arguments)
        )) >>
        result: step_result >>
        (Step {
            op: op_type,
            result,
            args,
        })
    )
}

/// Matches primitive operations
pub fn primitive_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    alt!(input,
        map!(tag!("*"), |_| MacroOp::Multiply)  |
        map!(tag!("+"), |_| MacroOp::Add)       |
        map!(tag!("-"), |_| MacroOp::Subtract)  |
        map!(tag!("/"), |_| MacroOp::Divide)
    )
}

/// Matches arguments in quotes ("")
pub fn quoted_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        word: ws!(delimited!(tag!("\""),take_until!("\""), tag!("\""))) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches roll flag "e"
pub fn roll_flag_e_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("e") >>
        num: digit >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::E((s.parse::<i16>().unwrap()))))
    )
}

/// Matches roll flag "h"
pub fn roll_flag_h_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("h") >>
        num: digit >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::H((s.parse::<i16>().unwrap()))))
    )
}

/// Matches roll flag "k"
pub fn roll_flag_k_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("k") >>
        (Arg::Roll(RollArg::K))
    )
}

/// matches roll flag "l"
pub fn roll_flag_l_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("l") >>
        num: digit >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::L((s.parse::<i16>().unwrap()))))
    )
}

/// matches roll flag "ro"
pub fn roll_flag_ro_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("ro") >>
        num: digit >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::RO((s.parse::<i16>().unwrap()))))
    )
}

/// matches roll flag "rr"
pub fn roll_flag_rr_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("rr") >>
        num: digit >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::RR((s.parse::<i16>().unwrap()))))
    )
}

/// Matches "N" in NdD
pub fn roll_num_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        num: ws!(digit) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::N(s.parse::<u8>().unwrap())))
    )
}

/// Matches "D" in NdD
pub fn roll_die_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        num: ws!(preceded!(tag!("d"), digit)) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (Arg::Roll(RollArg::D(s.parse::<u8>().unwrap())))
    )
}

/// Matches arguments in quotes ('')
pub fn single_quoted_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        word: ws!(delimited!(tag!("'"),take_until!("'"), tag!("'"))) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches a passed or ignored result
pub fn step_result_p(input: &[u8]) -> IResult<&[u8], StepResult> {
    alt_complete!(input,
        map!(ws!(tag!(">>")), |_| StepResult::Pass) |
        value!(StepResult::Ignore)
    )
}

/// Match alphanumeric values to strings
pub fn string_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        word: ws!(alphanumeric) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches variables
pub fn variable_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        var: ws!(preceded!(tag!("$"), alphanumeric)) >>
        (String::from_utf8(var.to_vec()).unwrap())
    )
}

/// Maps error codes to readable strings
pub fn error_to_string(e: Err) -> String {
    let err = match e {
        ErrorKind::Custom(1)    => "Missing or invalid macro name",
        ErrorKind::Custom(2)    => "Invalid or unrecognized command",
        _                       => "Unknown problem encountered while parsing",
    };
    err.to_string()
}

// Define our macros
named!(advantage <&[u8], Arg>, call!(advantage_p));
named!(arguments <&[u8], Arg>, call!(arguments_p));
named!(arguments_roll <&[u8], Arg>, call!(arguments_roll_p));
named!(arguments_say <&[u8], Arg>, call!(arguments_say_p));
named!(arguments_whisper <&[u8], Arg>, call!(arguments_whisper_p));
named!(command <&[u8], MacroOp>, call!(command_p));
named!(disadvantage <&[u8], Arg>, call!(disadvantage_p));
named!(name <&[u8], MacroOp>, call!(name_p));
named!(num <&[u8], Arg>, call!(num_p));
named!(op <&[u8], MacroOp>, call!(op_p));
named!(parse <&[u8], Program>, call!(parse_p));
named!(parse_step <&[u8], Step>, call!(parse_step_p));
named!(primitive <&[u8], MacroOp>, call!(primitive_p));
named!(quoted <&[u8], String>, call!(quoted_p));
named!(roll_flag_e <&[u8], Arg>, call!(roll_flag_e_p));
named!(roll_flag_h <&[u8], Arg>, call!(roll_flag_h_p));
named!(roll_flag_k <&[u8], Arg>, call!(roll_flag_k_p));
named!(roll_flag_l <&[u8], Arg>, call!(roll_flag_l_p));
named!(roll_flag_ro <&[u8], Arg>, call!(roll_flag_ro_p));
named!(roll_flag_rr <&[u8], Arg>, call!(roll_flag_rr_p));
named!(roll_num <&[u8], Arg>, call!(roll_num_p));
named!(roll_die <&[u8], Arg>, call!(roll_die_p));
named!(single_quoted <&[u8], String>, call!(single_quoted_p));
named!(step_result <&[u8], StepResult>, call!(step_result_p));
named!(string <&[u8], String>, call!(string_p));
named!(variable <&[u8], String>, call!(variable_p));
