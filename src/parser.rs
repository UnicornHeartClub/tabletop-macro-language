// @todo - It would be nice to break each parser into it's own module
// e.g. parser::roll, parser::say, parser::core

// use errors::*;
use nom::{alphanumeric, digit, ErrorKind, IResult};
use roll::RollArg;
use std::str;

#[derive(Debug, PartialEq, Eq)]
struct Argument {
    arg: Arg,
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Program {
    name: MacroOp,
    steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq)]
struct Step {
    args: Vec<Argument>,
    op: MacroOp,
    result: StepResult,
}

#[derive(Debug, PartialEq, Eq)]
enum MacroOp {
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
enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Pass Result (>>)
    Pass,
}

#[derive(Debug, PartialEq, Eq)]
enum Arg {
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

#[derive(Debug, PartialEq, Eq)]
enum SayArg {
    Message,
    To,
}

named!(advantage <&[u8]>, alt_complete!(
    tag!("advantage") |
    tag!("adv")
));

named!(disadvantage <&[u8]>, alt_complete!(
    tag!("disadvantage") |
    tag!("dis")
));

/// Matches unknown arguments to commands
named!(arguments <&[u8], Argument>, alt_complete!(
    map!(num, | a | Argument { arg: Arg::Number, value: a }) |
    map!(string, | a | Argument { arg: Arg::Unrecognized, value: a }) |
    map!(quoted, | a | Argument { arg: Arg::Unrecognized, value: a }) |
    map!(single_quoted, | a | Argument { arg: Arg::Unrecognized, value: a }) |
    map!(variable, | a | Argument { arg: Arg::Variable, value: a })
));

// bug here - doesn't continue parsing on ...
/// Matches !roll arguments
named!(arguments_roll <&[u8], Argument>, alt_complete!(
    map!(advantage, | _ | Argument { arg: Arg::Roll(RollArg::Advantage), value: String::from("") }) |
    map!(disadvantage, | _ | Argument { arg: Arg::Roll(RollArg::Disadvantage), value: String::from("") }) |
    map!(roll_num, | a | Argument { arg: Arg::Roll(RollArg::N), value: a }) |
    map!(roll_die, | a | Argument { arg: Arg::Roll(RollArg::D), value: a }) |
    map!(roll_flag_e, | a | Argument { arg: Arg::Roll(RollArg::E), value: a }) |
    map!(roll_flag_h, | a | Argument { arg: Arg::Roll(RollArg::H), value: a }) |
    map!(roll_flag_k, | a | Argument { arg: Arg::Roll(RollArg::K), value: a }) |
    map!(roll_flag_l, | a | Argument { arg: Arg::Roll(RollArg::L), value: a }) |
    map!(roll_flag_ro, | a | Argument { arg: Arg::Roll(RollArg::RO), value: a }) |
    map!(roll_flag_rr, | a | Argument { arg: Arg::Roll(RollArg::RR), value: a }) |
    map!(quoted, | a | Argument { arg: Arg::Roll(RollArg::Comment), value: a }) |
    map!(single_quoted, | a | Argument { arg: Arg::Roll(RollArg::Comment), value: a }) |
    map!(variable, | a | Argument { arg: Arg::Variable, value: a })
));

named!(roll_flag_e <&[u8], String>, do_parse!(
    tag!("e") >>
    num: digit >>
    (String::from_utf8(num.to_vec()).unwrap())
));

named!(roll_flag_h <&[u8], String>, do_parse!(
    tag!("h") >>
    num: digit >>
    (String::from_utf8(num.to_vec()).unwrap())
));

named!(roll_flag_k <&[u8], String>, do_parse!(
    tag!("k") >>
    (String::from(""))
));

named!(roll_flag_l <&[u8], String>, do_parse!(
    tag!("l") >>
    num: digit >>
    (String::from_utf8(num.to_vec()).unwrap())
));

named!(roll_flag_ro <&[u8], String>, do_parse!(
    tag!("ro") >>
    num: digit >>
    (String::from_utf8(num.to_vec()).unwrap())
));

named!(roll_flag_rr <&[u8], String>, do_parse!(
    tag!("rr") >>
    num: digit >>
    (String::from_utf8(num.to_vec()).unwrap())
));

/// Matches !say arguments
named!(arguments_say <&[u8], Argument>, alt_complete!(
    map!(string, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
    map!(quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
    map!(single_quoted, | a | Argument { arg: Arg::Say(SayArg::Message), value: a }) |
    map!(variable, | a | Argument { arg: Arg::Variable, value: a })
));

/// Matches a command
named!(command <&[u8], MacroOp>, alt!(
    map!(ws!(tag!("!roll")),     |_| MacroOp::Roll)      |
    map!(ws!(tag!("!r")),        |_| MacroOp::Roll)      |
    map!(ws!(tag!("!say")),      |_| MacroOp::Say)       |
    map!(ws!(tag!("!s")),        |_| MacroOp::Say)       |
    map!(ws!(tag!("!whisper")),  |_| MacroOp::Whisper)   |
    map!(ws!(tag!("!w")),        |_| MacroOp::Whisper)
));

named!(roll_num <&[u8], String>, do_parse!(
    num: ws!(digit) >>
    (String::from_utf8(num.to_vec()).unwrap())
));

named!(roll_die <&[u8], String>, do_parse!(
    num: ws!(preceded!(tag!("d"), digit)) >>
    (String::from_utf8(num.to_vec()).unwrap())
));

/// Matches a macro name
named!(name <&[u8], MacroOp>, ws!(do_parse!(
    tag!("#") >>
    name: map_res!(is_not!(" \t\r\n"), |r: &[u8]| String::from_utf8(r.to_vec())) >>
    (MacroOp::Name(name))
)));

/// Match numbers to argument strings
named!(num <&[u8], String>, do_parse!(
    number: ws!(digit) >>
    (String::from_utf8(number.to_vec()).unwrap())
));

/// Matches any type of operation
named!(op <&[u8], MacroOp>, alt!(
    name |
    command |
    ws!(primitive)
));

/// Matches primitive operations
named!(primitive <&[u8], MacroOp>, alt!(
    map!(tag!("*"), |_| MacroOp::Multiply)  |
    map!(tag!("+"), |_| MacroOp::Add)       |
    map!(tag!("-"), |_| MacroOp::Subtract)  |
    map!(tag!("/"), |_| MacroOp::Divide)
));

/// Parse the complete macro
named!(parse <&[u8], Program>, do_parse!(
    prog_name: name >>
    steps: many0!(parse_step) >>
    (Program {
        name: prog_name,
        steps: steps,
    })
));

/// Parse a step of the program
named!(parse_step <&[u8], Step>, do_parse!(
    op_type: op >>
    args: many0!(switch!(value!(&op_type),
        &MacroOp::Roll => call!(arguments_roll) |
        &MacroOp::Say => call!(arguments_say) |
        _ => call!(arguments)
    )) >>
    result: step_result >>
    (Step {
        op: op_type,
        result,
        args,
    })
));

/// Matches arguments in quotes ("")
named!(quoted <&[u8], String>, do_parse!(
    word: ws!(delimited!(tag!("\""),take_until!("\""), tag!("\""))) >>
    (String::from_utf8(word.to_vec()).unwrap())
));

/// Matches arguments in quotes ('')
named!(single_quoted <&[u8], String>, do_parse!(
    word: ws!(delimited!(tag!("'"),take_until!("'"), tag!("'"))) >>
    (String::from_utf8(word.to_vec()).unwrap())
));

/// Matches a passed or ignored result
named!(step_result <&[u8], StepResult>, alt_complete!(
    map!(ws!(tag!(">>")), |_| StepResult::Pass) |
    value!(StepResult::Ignore)
));

/// Match alphanumeric values to strings
named!(string <&[u8], String>, do_parse!(
    word: ws!(alphanumeric) >>
    (String::from_utf8(word.to_vec()).unwrap())
));

/// Matches variables
named!(variable <&[u8], String>, do_parse!(
    var: ws!(preceded!(tag!("$"), alphanumeric)) >>
    (String::from_utf8(var.to_vec()).unwrap())
));

// ---- Tests ----
// @todo - I would like to move these into the tests/ folder

#[test]
fn test_simple_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name")),
        steps: vec![Step {
            args: vec![
                Argument {
                    arg: Arg::Roll(RollArg::N),
                    value: "1".to_string()
                },
                Argument {
                    arg: Arg::Roll(RollArg::D),
                    value: "20".to_string()
                }
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }],
    };
    let (_, result) = parse(b"#simple-macro-name !roll 1d20").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name-2")),
        steps: vec![Step {
            args: vec![Argument {
                arg: Arg::Say(SayArg::Message),
                value: "Hello, world!".to_string()
            }],
            op: MacroOp::Say,
            result: StepResult::Ignore,
        }],
    };
    let (_, result) = parse(b"#simple-macro-name-2 !say \"Hello, world!\"").unwrap();
    assert_eq!(result, program);
}

#[test]
fn test_complex_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name")),
        steps: vec![
            Step {
                args: vec![
                    Argument {
                        arg: Arg::Roll(RollArg::N),
                        value: "1".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::D),
                        value: "20".to_string()
                    }
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![Argument {
                    arg: Arg::Say(SayArg::Message),
                    value: "Smite!".to_string()
                }],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse(b"#complex-macro-name !roll 1d20 !say \"Smite!\"").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name-2")),
        steps: vec![
            Step {
                args: vec![
                    Argument {
                        arg: Arg::Roll(RollArg::N),
                        value: "3".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::D),
                        value: "8".to_string()
                    }
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![Argument {
                    arg: Arg::Number,
                    value: "3".to_string()
                }],
                op: MacroOp::Add,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![Argument {
                    arg: Arg::Say(SayArg::Message),
                    value: "Smite!".to_string()
                }],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Argument {
                        arg: Arg::Roll(RollArg::N),
                        value: "2".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::D),
                        value: "20".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::K),
                        value: "".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::H),
                        value: "1".to_string()
                    }
                ],
                op: MacroOp::Roll,
                result: StepResult::Pass,
            },
            Step {
                args: vec![
                    Argument {
                        arg: Arg::Say(SayArg::Message),
                        value: "I rolled a ".to_string()
                    },
                    Argument {
                        arg: Arg::Variable,
                        value: "1".to_string()
                    }
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse(b"#complex-macro-name-2 !roll 3d8+3 !say \"Smite!\" !roll 2d20kh1 >> !say \"I rolled a \" $1").unwrap();
    assert_eq!(result, program);
}

#[test]
fn test_name_parser() {
    let (_, result) = name(b"#macro_name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro_name")));

    let (_, result) = name(b"#macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro-name")));

    let (_, result) = name(b"#123macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("123macro-name")));

    let (_, result) = name(b"#Z123macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("Z123macro-name")));

    let (_, result) = name(b"#macro-name cannot have spaces").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro-name")));

    let bad_result = name(b"macro_name");
    match bad_result {
        IResult::Error(e) => assert_eq!(e, ErrorKind::Tag),
        _ => assert_eq!(1, 0),
    }
}

#[test]
fn test_command_parser_roll() {
    let (_, result) = command(b"!roll 1d20").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"!r 1d20").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"!roll advantage").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"!roll adv").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"!roll disadvantage").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"!roll dis").unwrap();
    assert_eq!(result, MacroOp::Roll);
}

#[test]
fn test_op_parser() {
    let (_, result) = op(b"    #test-macro   ").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("test-macro")));
    let (_, result) = command(b"    !roll 1d20 ").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command(b"   !say \"Hello!\" ").unwrap();
    assert_eq!(result, MacroOp::Say);
    let (_, result) = command(b"   !whisper").unwrap();
    assert_eq!(result, MacroOp::Whisper);
}

#[test]
fn test_arguments_parser() {
    let (_, result) = arguments(b"\"hello\"").unwrap();
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("hello") });
    let (_, result) = arguments(b"   Hello  ").unwrap();
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("Hello") });
    let (_, result) = arguments(b"'   Single String Args'").unwrap();
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("Single String Args") });
}

#[test]
fn test_quoted_parser() {
    let (_, result) = quoted(b"\"hello\"").unwrap();
    assert_eq!(result, String::from("hello"));
    let (_, result) = quoted(b"\"   Hello  \"").unwrap();
    assert_eq!(result, String::from("Hello  "));
}

#[test]
fn test_single_quoted_parser() {
    let (_, result) = single_quoted(b"'test 123'").unwrap();
    assert_eq!(result, String::from("test 123"));
    let (_, result) = single_quoted(b"'   Single String Args'").unwrap();
    assert_eq!(result, String::from("Single String Args"));
}

#[test]
fn test_step_result_parser() {
    let (_, result) = step_result(b">>").unwrap();
    assert_eq!(result, StepResult::Pass);

    let (_, result) = step_result(b" ").unwrap();
    assert_eq!(result, StepResult::Ignore);
}

#[test]
fn test_arguments_roll_parser() {
    // Pass it through once should yield us the N and remove a "d"
    let (rest, result) = arguments_roll(b"1d20").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::N),
        value: "1".to_string(),
    });
    // Running through a second time will yield us the D
    let (_, result) = arguments_roll(rest).unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::D),
        value: "20".to_string(),
    });

    // Advantage
    let (_, result) = arguments_roll(b"adv").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Advantage),
        value: "".to_string(),
    });
    let (_, result) = arguments_roll(b"advantage").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Advantage),
        value: "".to_string(),
    });

    // Disadvantage
    let (_, result) = arguments_roll(b"dis").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Disadvantage),
        value: "".to_string(),
    });
    let (_, result) = arguments_roll(b"disadvantage").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Disadvantage),
        value: "".to_string(),
    });
}
