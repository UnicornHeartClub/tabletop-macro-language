// use errors::*;
use nom::{alphanumeric, anychar, eol, ErrorKind, IResult};

#[derive(Debug, PartialEq, Eq)]
struct Program {
    name: MacroOp,
    steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq)]
struct Step {
    result: StepResult,
    op: MacroOp,
    args: Vec<String>
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
enum RollOp {
    Advantage,
    Comment,
    Disadvantage,
    NdD,
}

#[derive(Debug, PartialEq, Eq)]
enum SayOp {
    Message,
    To,
}

named!(arguments <&[u8], String>, alt!(
    string |
    quoted |
    single_quoted
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

/// Matches a macro name
named!(name <&[u8], MacroOp>, ws!(do_parse!(
    tag!("#") >>
    name: map_res!(is_not!(" \t\r\n"), |r: &[u8]| String::from_utf8(r.to_vec())) >>
    (MacroOp::Name(name))
)));

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

named!(step_result <&[u8], StepResult>, alt_complete!(
    map!(ws!(tag!(">>")), |_| StepResult::Pass) |
    value!(StepResult::Ignore)
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
    args: ws!(many0!(arguments)) >>
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


/// Match alphanumeric values to strings
named!(string <&[u8], String>, do_parse!(
    word: ws!(alphanumeric) >>
    (String::from_utf8(word.to_vec()).unwrap())
));

#[test]
fn test_simple_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name")),
        steps: vec![Step {
            op: MacroOp::Roll,
            result: StepResult::Ignore,
            args: vec![ "1d20".to_string() ],
        }],
    };
    let (_, result) = parse(b"#simple-macro-name !roll 1d20").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name-2")),
        steps: vec![Step {
            op: MacroOp::Say,
            result: StepResult::Ignore,
            args: vec![ "Hello, world!".to_string() ],
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
                args: vec![ "1d20".to_string() ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![ "Smite!".to_string() ],
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
                args: vec![ "3d8".to_string() ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![ "3".to_string() ],
                op: MacroOp::Add,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![ "Smite!".to_string() ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![ "1d20".to_string() ],
                op: MacroOp::Roll,
                result: StepResult::Pass,
            },
            Step {
                args: vec![ "I rolled a ".to_string() ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse(b"#complex-macro-name-2 !roll 3d8+3 !say \"Smite!\" !roll 1d20 >> !say \"I rolled a \" $1").unwrap();
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
    assert_eq!(result, String::from("hello"));
    let (_, result) = arguments(b"   Hello  ").unwrap();
    assert_eq!(result, String::from("Hello"));
    let (_, result) = arguments(b"'   Single String Args'").unwrap();
    assert_eq!(result, String::from("Single String Args"));
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
