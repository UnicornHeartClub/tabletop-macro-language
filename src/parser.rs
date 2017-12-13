// @todo - It would be nice to break each parser into it's own module
// e.g. parser::roll, parser::say, parser::core

use nom::{alphanumeric, digit, ErrorKind, IResult};
use nom::simple_errors::Err;
use std::str;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assign {
    pub left: ArgValue,
    pub right: Vec<ArgValue>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conditional {
    pub left: ArgValue,
    pub comparison: ComparisonArg,
    pub right: ArgValue,
    pub success: Option<Step>,
    pub failure: Option<Step>,
}

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
    pub attribute: Option<String>,
    pub macro_name: Option<String>,
}

// Top-level arguments
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arg {
    Assign(Assign),
    Conditional(Conditional),
    Roll(RollArg),
    Say(SayArg),
    Token(TokenArg),
    Unrecognized(String),
    Variable(String),
}

// Command-level arguments
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgValue {
    Number(i32),
    Primitive(Primitive),
    Text(String),
    Token(TokenArg),
    Variable(String),
    VariableReserved(i16),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Primitive {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonArg {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroOp {
    Primitive,
    /// Lamda (assignment or conditional argument)
    Lambda,
    /// Macro Name
    Name(String),
    /// Prompt (!prompt)
    Prompt,
    /// Roll (!roll)
    Roll,
    /// Say (!say)
    Say,
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
    D(ArgValue), // e.g. d20
    Disadvantage,
    E(ArgValue),
    GT(ArgValue),
    GTE(ArgValue),
    H(ArgValue),
    L(ArgValue),
    LT(ArgValue),
    LTE(ArgValue),
    Max(ArgValue),
    Min(ArgValue),
    ModifierNeg(ArgValue),
    ModifierPos(ArgValue),
    N(ArgValue), // e.g. 1 (part of 1d20)
    RO(ArgValue),
    RR(ArgValue),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SayArg {
    Message(String),
    To(TokenArg),
    From(TokenArg),
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
    Number(i32),
    Text(String),
}

/// Matches advantage roll argument
pub fn advantage_p(input: &[u8]) -> IResult<&[u8], Arg> {
    map!(input, alt_complete!(tag!("advantage") | tag!("adv")), |_| Arg::Roll(RollArg::Advantage))
}

/// Matches left = right scenarios
pub fn assignment_p(input: &[u8]) -> IResult<&[u8], Assign> {
    do_parse!(input,
        // we can only assign to tokens and variables
        left: ws!(alt_complete!(
            map!(variable_p, | a | ArgValue::Variable(a)) |
            map!(token_p, | a | ArgValue::Token(a))
        )) >>
        ws!(tag!("=")) >>
        // but we can assign almost anything else to them (except inline arguments)
        right: many0!(ws!(alt_complete!(
            primitive_p |
            map!(num_p, | a | ArgValue::Number(a)) |
            map!(string_p, | a | ArgValue::Text(a)) |
            map!(quoted_p, | a | ArgValue::Text(a)) |
            map!(single_quoted_p, | a | ArgValue::Text(a)) |
            map!(variable_reserved_p, | a | ArgValue::VariableReserved(a)) |
            map!(variable_p, | a | ArgValue::Variable(a))
        ))) >>
        (Assign {
            left,
            right,
        })
    )
}

/// Matches arguments of unknown commands
pub fn arguments_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(conditional_p, | a | Arg::Conditional(a)) |
        map!(assignment_p, | a | Arg::Assign(a)) |
        map!(variable_p, | a | Arg::Variable(a)) |
        map!(token_p, | a | Arg::Token(a)) |
        map!(string_p, | a | Arg::Unrecognized(a)) |
        map!(quoted_p, | a | Arg::Unrecognized(a)) |
        map!(single_quoted_p, | a | Arg::Unrecognized(a))
    )
}

/// Matches !roll arguments
pub fn arguments_roll_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        advantage_p |
        disadvantage_p |
        roll_num_p |
        roll_die_p |
        roll_flag_e_p |
        roll_flag_gt_p |
        roll_flag_gte_p |
        roll_flag_h_p |
        roll_flag_l_p |
        roll_flag_lt_p |
        roll_flag_lte_p |
        roll_flag_max_p |
        roll_flag_min_p |
        roll_flag_ro_p |
        roll_flag_rr_p |
        roll_modifier_pos_p |
        roll_modifier_neg_p |
        map!(quoted_p,        | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))) |
        map!(single_quoted_p, | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))) |
        map!(variable_p,      | a | Arg::Variable(a))
    )
}

/// Matches !say arguments
pub fn arguments_say_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(string_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(quoted_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(single_quoted_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(token_p, | a | Arg::Say(SayArg::From(a))) |
        map!(variable_p, | a | Arg::Variable(a))
    )
}

/// Matches !whisper arguments
pub fn arguments_whisper_p(input: &[u8]) -> IResult<&[u8], Arg> {
    alt_complete!(input,
        map!(string_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(quoted_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(single_quoted_p, | a | Arg::Say(SayArg::Message(a))) |
        map!(token_p, | a | Arg::Say(SayArg::To(a))) |
        map!(variable_p, | a | Arg::Variable(a))
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

/// Matches conditional statements (e.g. "1 > 2 ? success : failure")
pub fn conditional_p(input: &[u8]) -> IResult<&[u8], Conditional> {
    add_return_error!(input, ErrorKind::Custom(3), do_parse!(
        // we can only assign to tokens and variables
        left: ws!(alt_complete!(
            map!(variable_reserved_p, | a | ArgValue::VariableReserved(a)) |
            map!(variable_p, | a | ArgValue::Variable(a)) |
            map!(token_p, | a | ArgValue::Token(a)) |
            map!(num_p, | a | ArgValue::Number(a))
        )) >>
        comparison: ws!(alt_complete!(
            map!(tag!("=="), |_| ComparisonArg::EqualTo) |
            map!(tag!(">="), |_| ComparisonArg::GreaterThanOrEqual) |
            map!(tag!("<="), |_| ComparisonArg::LessThanOrEqual) |
            map!(tag!(">"), |_| ComparisonArg::GreaterThan) |
            map!(tag!("<"), |_| ComparisonArg::LessThan)
        )) >>
        // but we can assign almost anything else to them (except inline arguments)
        right: ws!(alt_complete!(
            map!(num_p, | a | ArgValue::Number(a)) |
            map!(variable_reserved_p, | a | ArgValue::VariableReserved(a)) |
            map!(variable_p, | a | ArgValue::Variable(a))
        )) >>
        ws!(tag!("?")) >>
        success: ws!(alt_complete!(
            map!(tag!("|"), |_| None) |
            opt!(parse_step_p)
        )) >>
        ws!(tag!(":")) >>
        failure: ws!(alt_complete!(
            map!(tag!("|"), |_| None) |
            opt!(parse_step_p)
        )) >>
        (Conditional {
            left,
            comparison,
            right,
            success: success,
            failure: failure,
        })
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
pub fn num_p(input: &[u8]) -> IResult<&[u8], i32> {
    do_parse!(input,
        num: ws!(digit) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        (s.parse::<i32>().unwrap())
    )
}

/// Matches any type of operation
pub fn op_p(input: &[u8]) -> IResult<&[u8], MacroOp> {
    alt_complete!(input,
        name_p |
        command_p |
        value!(MacroOp::Lambda)
    )
}

/// Parse the complete macro
pub fn parse_p(input: &[u8]) -> IResult<&[u8], Program> {
    do_parse!(input,
        prog_name: name_p >>
        steps: many0!(parse_step_p) >>
        (Program {
            name: prog_name,
            steps: steps,
        })
    )
}

/// Parse a step of the program
pub fn parse_step_p(input: &[u8]) -> IResult<&[u8], Step> {
    do_parse!(input,
        op_type: op_p >>
        args: many0!(switch!(value!(&op_type),
            &MacroOp::Roll => call!(arguments_roll_p) |
            &MacroOp::Say => call!(arguments_say_p) |
            &MacroOp::Whisper => call!(arguments_whisper_p) |
            _ => call!(arguments_p)
        )) >>
        result: step_result_p >>
        (Step {
            op: op_type,
            result,
            args,
            value: None,
        })
    )
}

/// Matches primitive operations (starts with a number)
pub fn primitive_p(input: &[u8]) -> IResult<&[u8], ArgValue> {
    ws!(input, alt_complete!(
        map!(tag!("+"), |_| ArgValue::Primitive(Primitive::Add)) |
        map!(tag!("-"), |_| ArgValue::Primitive(Primitive::Subtract)) |
        map!(tag!("/"), |_| ArgValue::Primitive(Primitive::Divide)) |
        map!(tag!("*"), |_| ArgValue::Primitive(Primitive::Multiply))
    ))
}

/// Matches arguments in quotes ("")
pub fn quoted_p(input: &[u8]) -> IResult<&[u8], String> {
    do_parse!(input,
        word: ws!(delimited!(tag!("\""),take_until!("\""), tag!("\""))) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches digits for "D" and parses to i32
pub fn roll_digit_p(input: &[u8]) -> IResult<&[u8], i32> {
    do_parse!(input,
        var: digit >>
        num: value!(String::from_utf8(var.to_vec()).unwrap()) >>
        (num.parse::<i32>().unwrap())
    )
}

/// Matches roll flag "e"
pub fn roll_flag_e_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("e") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::E(var)))
    )
}

/// Matches roll flag "gt"
pub fn roll_flag_gt_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("gt") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::GT(var)))
    )
}

/// Matches roll flag "gte"
pub fn roll_flag_gte_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("gte") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::GTE(var)))
    )
}

/// Matches roll flag "lt"
pub fn roll_flag_lt_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("lt") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::LT(var)))
    )
}

/// Matches roll flag "lte"
pub fn roll_flag_lte_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("lte") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::LTE(var)))
    )
}

/// Matches roll flag "h"
pub fn roll_flag_h_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("kh") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::H(var)))
    )
}

/// Matches roll flag "l"
pub fn roll_flag_l_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("kl") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::L(var)))
    )
}

/// Matches roll flag "max"
pub fn roll_flag_max_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("max") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::Max(var)))
    )
}

/// Matches roll flag "min"
pub fn roll_flag_min_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("min") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::Min(var)))
    )
}

/// Matches roll flag "ro"
pub fn roll_flag_ro_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("ro") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::RO(var)))
    )
}

/// Matches roll flag "rr"
pub fn roll_flag_rr_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        tag!("rr") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::RR(var)))
    )
}

/// Matches valid roll flag inputs
pub fn roll_flag_var_p(input: &[u8]) -> IResult<&[u8], ArgValue> {
    ws!(input, alt_complete!(
        map!(variable_reserved_p, |n| ArgValue::VariableReserved(n)) |
        map!(variable_p, |n| ArgValue::Variable(n)) |
        map!(roll_digit_p, |n| ArgValue::Number(n))
    )) 
}


/// Matches + modifiers
pub fn roll_modifier_neg_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("-"), roll_modifier_var_p)) >>
        (Arg::Roll(RollArg::ModifierNeg(var)))
    )
}

/// Matches - modifiers
pub fn roll_modifier_pos_p(input: &[u8]) -> IResult<&[u8], Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("+"), roll_modifier_var_p)) >>
        (Arg::Roll(RollArg::ModifierPos(var)))
    )
}

/// Matches valid modifier inputs
pub fn roll_modifier_var_p(input: &[u8]) -> IResult<&[u8], ArgValue> {
    alt!(input,
        map!(variable_reserved_p, |n| ArgValue::VariableReserved(n)) |
        map!(variable_p, |n| ArgValue::Variable(n)) |
        map!(roll_digit_p, |n| ArgValue::Number(n)) |
        map!(token_p, |n| ArgValue::Token(n))
    )
}

/// Matches "N" in NdD
pub fn roll_num_p(input: &[u8]) -> IResult<&[u8], Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::N(var)))
    )
}

/// Matches "D" in NdD
pub fn roll_die_p(input: &[u8]) -> IResult<&[u8], Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: ws!(preceded!(tag!("d"), roll_flag_var_p)) >>
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
        map!(ws!(tag!("|")), |_| StepResult::Ignore) |
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
        macro_name: switch!(opt!(complete!(preceded!(tag!("->"), alphanumeric))),
            Some(a) => value!(Some(String::from_utf8(a.to_vec()).unwrap())) |
            _ => value!(None)
        ) >>
        (TokenArg { name, attribute, macro_name })
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
        ErrorKind::Custom(3)    => "Problem parsing conditional statement",
        _                       => "Unknown problem encountered while parsing",
    };
    err.to_string()
}
