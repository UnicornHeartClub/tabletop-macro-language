// @todo - It would be nice to break each parser into it's own module
// e.g. parser::roll, parser::say, parser::core

// use errors::*;
use nom::{alphanumeric, digit, ErrorKind, IResult};
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub arg: Arg,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub name: MacroOp,
    pub steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    pub args: Vec<Argument>,
    pub op: MacroOp,
    pub result: StepResult,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Pass Result (>>)
    Pass,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Arg {
    /// Number (Float, Integer)
    Number,
    /// Unrecognized argument
    Unrecognized,
    /// Roll arguments
    Roll(RollArg),
    /// Say arguments
    Say(SayArg),
    /// Static variable ($)
    Variable,
}

// Arguments for the roll command, used by the parser
#[derive(Debug, PartialEq, Eq)]
pub enum RollArg {
    Advantage,
    Comment,
    Disadvantage,
    D, // e.g. d20
    E,
    H,
    K,
    L,
    N, // e.g. 1 (part of 1d20)
    RO,
    RR,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SayArg {
    Message,
    To,
}

/// Matches advantage roll argument
pub fn advantage_p(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt_complete!(input, tag!("advantage") | tag!("adv"))
}

/// Matches arguments of unknown commands
pub fn arguments_p(input: &[u8]) -> IResult<&[u8], Argument> {
    alt_complete!(input,
        map!(num, | a | Argument { arg: Arg::Number, value: a }) |
        map!(string, | a | Argument { arg: Arg::Unrecognized, value: a }) |
        map!(quoted, | a | Argument { arg: Arg::Unrecognized, value: a }) |
        map!(single_quoted, | a | Argument { arg: Arg::Unrecognized, value: a }) |
        map!(variable, | a | Argument { arg: Arg::Variable, value: a })
    )
}

/// Matches !roll arguments
pub fn arguments_roll_p(input: &[u8]) -> IResult<&[u8], Argument> {
    alt_complete!(input,
        map!(advantage,     | _ | Argument { arg: Arg::Roll(RollArg::Advantage), value: String::from("") }) |
        map!(disadvantage,  | _ | Argument { arg: Arg::Roll(RollArg::Disadvantage), value: String::from("") }) |
        map!(roll_num,      | a | Argument { arg: Arg::Roll(RollArg::N), value: a }) |
        map!(roll_die,      | a | Argument { arg: Arg::Roll(RollArg::D), value: a }) |
        map!(roll_flag_e,   | a | Argument { arg: Arg::Roll(RollArg::E), value: a }) |
        map!(roll_flag_h,   | a | Argument { arg: Arg::Roll(RollArg::H), value: a }) |
        map!(roll_flag_k,   | a | Argument { arg: Arg::Roll(RollArg::K), value: a }) |
        map!(roll_flag_l,   | a | Argument { arg: Arg::Roll(RollArg::L), value: a }) |
        map!(roll_flag_ro,  | a | Argument { arg: Arg::Roll(RollArg::RO), value: a }) |
        map!(roll_flag_rr,  | a | Argument { arg: Arg::Roll(RollArg::RR), value: a }) |
        map!(quoted,        | a | Argument { arg: Arg::Roll(RollArg::Comment), value: a }) |
        map!(single_quoted, | a | Argument { arg: Arg::Roll(RollArg::Comment), value: a }) |
        map!(variable,      | a | Argument { arg: Arg::Variable, value: a })
    )
}

/// Matches !say arguments
pub fn arguments_say_p(input: &[u8]) -> IResult<&[u8], Argument> {
    alt_complete!(input,
        map!(string, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
        map!(quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
        map!(single_quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
        map!(variable, | a | Argument { arg: Arg::Variable, value: a })
    )
}

/// Matches !whisper arguments
pub fn arguments_whisper_p(input: &[u8]) -> IResult<&[u8], Argument> {
    alt_complete!(input,
        map!(variable, | a | Argument { arg: Arg::Say(SayArg::To), value: a }) |
        map!(string, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
        map!(quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
        map!(single_quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a })
    )
}

/// Matches any command
pub fn command_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    alt!(input,
        map!(ws!(tag!("!roll")),     |_| MacroOp::Roll)      |
        map!(ws!(tag!("!r")),        |_| MacroOp::Roll)      |
        map!(ws!(tag!("!say")),      |_| MacroOp::Say)       |
        map!(ws!(tag!("!s")),        |_| MacroOp::Say)       |
        map!(ws!(tag!("!whisper")),  |_| MacroOp::Whisper)   |
        map!(ws!(tag!("!w")),        |_| MacroOp::Whisper)
    )
}

/// Matches disadvantage roll argument
pub fn disadvantage_p(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt_complete!(input, tag!("disadvantage") | tag!("dis"))
}

/// Matches a macro name
pub fn name_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    ws!(
        input,
        do_parse!(
            tag!("#") >>
            name: map_res!(is_not!(" \t\r\n"), |r: &[u8]| String::from_utf8(r.to_vec())) >>
            (MacroOp::Name(name))
        )
    )
}

/// Match numbers to argument strings
pub fn num_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        number: ws!(digit) >>
        (String::from_utf8(number.to_vec()).unwrap())
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
pub fn roll_flag_e_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("e") >>
        num: digit >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// Matches roll flag "h"
pub fn roll_flag_h_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("h") >>
        num: digit >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// Matches roll flag "k"
pub fn roll_flag_k_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("k") >>
        (String::from(""))
    )
}

/// matches roll flag "l"
pub fn roll_flag_l_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("l") >>
        num: digit >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// matches roll flag "ro"
pub fn roll_flag_ro_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("ro") >>
        num: digit >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// matches roll flag "rr"
pub fn roll_flag_rr_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        tag!("rr") >>
        num: digit >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// Matches "N" in NdD
pub fn roll_num_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        num: ws!(digit) >>
        (String::from_utf8(num.to_vec()).unwrap())
    )
}

/// Matches "D" in NdD
pub fn roll_die_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        num: ws!(preceded!(tag!("d"), digit)) >>
        (String::from_utf8(num.to_vec()).unwrap())
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

// Define our macros
named!(advantage <&[u8]>, call!(advantage_p));
named!(arguments <&[u8], Argument>, call!(arguments_p));
named!(arguments_roll <&[u8], Argument>, call!(arguments_roll_p));
named!(arguments_say <&[u8], Argument>, call!(arguments_say_p));
named!(arguments_whisper <&[u8], Argument>, call!(arguments_whisper_p));
named!(command <&[u8], MacroOp>, call!(command_p));
named!(disadvantage <&[u8]>, call!(disadvantage_p));
named!(name <&[u8], MacroOp>, call!(name_p));
named!(num <&[u8], String>, call!(num_p));
named!(op <&[u8], MacroOp>, call!(op_p));
named!(parse <&[u8], Program>, call!(parse_p));
named!(parse_step <&[u8], Step>, call!(parse_step_p));
named!(primitive <&[u8], MacroOp>, call!(primitive_p));
named!(quoted <&[u8], String>, call!(quoted_p));
named!(roll_flag_e <&[u8], String>, call!(roll_flag_e_p));
named!(roll_flag_h <&[u8], String>, call!(roll_flag_h_p));
named!(roll_flag_k <&[u8], String>, call!(roll_flag_k_p));
named!(roll_flag_l <&[u8], String>, call!(roll_flag_l_p));
named!(roll_flag_ro <&[u8], String>, call!(roll_flag_ro_p));
named!(roll_flag_rr <&[u8], String>, call!(roll_flag_rr_p));
named!(roll_num <&[u8], String>, call!(roll_num_p));
named!(roll_die <&[u8], String>, call!(roll_die_p));
named!(single_quoted <&[u8], String>, call!(single_quoted_p));
named!(step_result <&[u8], StepResult>, call!(step_result_p));
named!(string <&[u8], String>, call!(string_p));
named!(variable <&[u8], String>, call!(variable_p));
