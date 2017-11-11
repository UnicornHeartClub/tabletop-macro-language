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
    pub value: Option<StepValue>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenArg {
    pub name: String,
    pub attribute: Option<String>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assign {
    pub left: ArgValue,
    pub right: ArgValue,
}

// Top-level arguments
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arg {
    Assign(Assign),
    Roll(RollArg),
    Say(SayArg),
    Token(TokenArg),
    Unrecognized(String),
    Variable(String),
}

// Command-level arguments
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgValue {
    Number(i16),
    Text(String),
    Token(TokenArg),
    Variable(String),
    VariableReserved(i16),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroOp {
    /// Addition (+)
    Add,
    /// Assign (=)
    Assign,
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
pub enum ProgramResult {
    Roll,
}

// Arguments for the roll command, used by the parser
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollArg {
    Advantage,
    Comment(ArgValue),
    Disadvantage,
    D(ArgValue), // e.g. d20
    E(ArgValue),
    H(ArgValue),
    L(ArgValue),
    ModifierPos(ArgValue),
    ModifierNeg(ArgValue),
    N(ArgValue), // e.g. 1 (part of 1d20)
    RO(ArgValue),
    RR(ArgValue),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SayArg {
    Message(String),
    To(String),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Save Result (>>)
    Save,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepValue {
    Number(i16),
    Text(String),
}

/// Matches advantage roll argument
pub fn advantage_p(input: &[u8]) -> IResult<&[u8], Arg> {
    map!(input, alt_complete!(tag!("advantage") | tag!("adv")), |_| Arg::Roll(RollArg::Advantage))
}

/// Matches left = right scenarios
pub fn assignment_p(input: &[u8]) -> IResult<&[u8], Assign> {
    do_parse!(input,
        left: ws!(alt_complete!(
            map!(variable, | a | ArgValue::Variable(a)) |
            map!(token, | a | ArgValue::Token(a))
        )) >>
        ws!(tag!("=")) >>
        right: ws!(alt_complete!(
            map!(num, | a | ArgValue::Number(a)) |
            map!(string, | a | ArgValue::Text(a)) |
            map!(quoted, | a | ArgValue::Text(a)) |
            map!(single_quoted, | a | ArgValue::Text(a))
        )) >>
        (Assign {
            left,
            right,
        })
    )
}

/// Matches arguments of unknown commands
pub fn arguments_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(assignment, | a | Arg::Assign(a)) |
        map!(variable, | a | Arg::Variable(a)) |
        map!(token, | a | Arg::Token(a)) |
        map!(string, | a | Arg::Unrecognized(a)) |
        map!(quoted, | a | Arg::Unrecognized(a)) |
        map!(single_quoted, | a | Arg::Unrecognized(a))
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
        roll_flag_l |
        roll_flag_ro |
        roll_flag_rr |
        roll_modifier_pos |
        roll_modifier_neg |
        map!(quoted,        | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))) |
        map!(single_quoted, | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))) |
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
    add_return_error!(input, ErrorKind::Custom(2), ws!(alt!(
        map!(alt!(tag!("!roll") | tag!("!r")),      |_| MacroOp::Roll)      |
        map!(alt!(tag!("!say") | tag!("!s")),       |_| MacroOp::Say)       |
        map!(alt!(tag!("!whisper") | tag!("!w")),   |_| MacroOp::Whisper)
    )))
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
pub fn num_p(input: &[u8]) -> IResult<&[u8], i16> {
    do_parse!(input,
        num: ws!(digit) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (s.parse::<i16>().unwrap())
    )
}

/// Matches any type of operation
pub fn op_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    alt_complete!(input,
        name |
        command |
        ws!(primitive) |
        value!(MacroOp::Assign)
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
            value: None,
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

/// Matches digits for "D" and parses to i16
pub fn roll_digit_p(input: &[u8]) -> IResult<&[u8], i16> {
    do_parse!(input,
        var: digit >>
        num: value!(String::from_utf8(var.to_vec()).unwrap()) >>
        (num.parse::<i16>().unwrap())
    )
}

/// Matches roll flag "e"
pub fn roll_flag_e_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("e") >>
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::E(var)))
    )
}

/// Matches roll flag "h"
pub fn roll_flag_h_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("kh") >>
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::H(var)))
    )
}

/// matches roll flag "l"
pub fn roll_flag_l_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("kl") >>
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::L(var)))
    )
}

/// matches roll flag "ro"
pub fn roll_flag_ro_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("ro") >>
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::RO(var)))
    )
}

/// matches roll flag "rr"
pub fn roll_flag_rr_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("rr") >>
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::RR(var)))
    )
}

/// Matches + modifiers
pub fn roll_modifier_neg_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("-"), alt!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n)) |
            map!(token, |n| ArgValue::Token(n))
        ))) >>
        (Arg::Roll(RollArg::ModifierNeg(var)))
    )
}

/// Matches - modifiers
pub fn roll_modifier_pos_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("+"), alt!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n)) |
            map!(token, |n| ArgValue::Token(n))
        ))) >>
        (Arg::Roll(RollArg::ModifierPos(var)))
    )
}

/// Matches "N" in NdD
pub fn roll_num_p(input: &[u8]) -> IResult<&[u8], Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: ws!(alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        )) >>
        (Arg::Roll(RollArg::N(var)))
    )
}

/// Matches "D" in NdD
pub fn roll_die_p(input: &[u8]) -> IResult<&[u8], Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: ws!(preceded!(tag!("d"), alt_complete!(
            map!(variable_reserved, |n| ArgValue::VariableReserved(n)) |
            map!(variable, |n| ArgValue::Variable(n)) |
            map!(roll_digit, |n| ArgValue::Number(n))
        ))) >>
        (Arg::Roll(RollArg::D(var)))
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
        map!(ws!(tag!(">>")), |_| StepResult::Save) |
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

/// Matches tokens
pub fn token_p(input: &[u8]) -> IResult<&[u8], TokenArg> {
    // @todo match that we cannot start with a digit
    do_parse!(input,
        name_raw: ws!(preceded!(tag!("@"), alphanumeric)) >>
        name: value!(String::from_utf8(name_raw.to_vec()).unwrap()) >>
        attribute: switch!(opt!(complete!(preceded!(tag!("."), alphanumeric))),
            Some(a) => value!(Some(String::from_utf8(a.to_vec()).unwrap())) |
            _ => value!(None)
        ) >>
        (TokenArg { name, attribute })
    )
}

/// Matches variables
pub fn variable_p(input: &[u8]) -> IResult<&[u8], String> {
    // @todo match that we cannot start with a digit
    do_parse!(input,
        var: ws!(preceded!(tag!("$"), alphanumeric)) >>
        (String::from_utf8(var.to_vec()).unwrap())
    )
}

/// Matches reserved variables (digits only)
pub fn variable_reserved_p(input: &[u8]) -> IResult<&[u8], i16> {
    do_parse!(input,
        var: ws!(preceded!(tag!("$"), digit)) >>
        num: value!(String::from_utf8(var.to_vec()).unwrap()) >>
        (num.parse::<i16>().unwrap())
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
named!(assignment <&[u8], Assign>, call!(assignment_p));
named!(arguments <&[u8], Arg>, call!(arguments_p));
named!(arguments_roll <&[u8], Arg>, call!(arguments_roll_p));
named!(arguments_say <&[u8], Arg>, call!(arguments_say_p));
named!(arguments_whisper <&[u8], Arg>, call!(arguments_whisper_p));
named!(command <&[u8], MacroOp>, call!(command_p));
named!(disadvantage <&[u8], Arg>, call!(disadvantage_p));
named!(name <&[u8], MacroOp>, call!(name_p));
named!(num <&[u8], i16>, call!(num_p));
named!(op <&[u8], MacroOp>, call!(op_p));
named!(parse <&[u8], Program>, call!(parse_p));
named!(parse_step <&[u8], Step>, call!(parse_step_p));
named!(primitive <&[u8], MacroOp>, call!(primitive_p));
named!(quoted <&[u8], String>, call!(quoted_p));
named!(roll_digit <&[u8], i16>, call!(roll_digit_p));
named!(roll_flag_e <&[u8], Arg>, call!(roll_flag_e_p));
named!(roll_flag_h <&[u8], Arg>, call!(roll_flag_h_p));
named!(roll_flag_l <&[u8], Arg>, call!(roll_flag_l_p));
named!(roll_flag_ro <&[u8], Arg>, call!(roll_flag_ro_p));
named!(roll_flag_rr <&[u8], Arg>, call!(roll_flag_rr_p));
named!(roll_modifier_neg <&[u8], Arg>, call!(roll_modifier_neg_p));
named!(roll_modifier_pos <&[u8], Arg>, call!(roll_modifier_pos_p));
named!(roll_num <&[u8], Arg>, call!(roll_num_p));
named!(roll_die <&[u8], Arg>, call!(roll_die_p));
named!(single_quoted <&[u8], String>, call!(single_quoted_p));
named!(step_result <&[u8], StepResult>, call!(step_result_p));
named!(string <&[u8], String>, call!(string_p));
named!(token <&[u8], TokenArg>, call!(token_p));
named!(variable <&[u8], String>, call!(variable_p));
named!(variable_reserved <&[u8], i16>, call!(variable_reserved_p));
