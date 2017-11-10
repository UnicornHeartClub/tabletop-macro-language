extern crate ttml;
extern crate nom;

use nom::{IResult, ErrorKind};
use ttml::parser::*;

#[test]
fn test_simple_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name")),
        steps: vec![Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20)))
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
            value: None,
        }],
    };
    let (_, result) = parse_p(b"#simple-macro-name !roll 1d20").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name-2")),
        steps: vec![Step {
            args: vec![
                Arg::Say(SayArg::Message("Hello, world!".to_string())),
            ],
            op: MacroOp::Say,
            result: StepResult::Ignore,
            value: None,
        }],
    };
    let (_, result) = parse_p(b"#simple-macro-name-2 !say \"Hello, world!\"").unwrap();
    assert_eq!(result, program);
}

#[test]
fn test_complex_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Pass,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(8))),
                    Arg::Roll(RollArg::Comment(ArgValue::Text("A cool roll comment".to_string()))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message("Smite!".to_string())),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
                value: None,
            },
        ],
    };
    let (_, result) = parse_p(b"#complex-macro-name !r 1d20 >> !roll $1d8 \"A cool roll comment\" !say \"Smite!\"").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name-2")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(3))),
                    Arg::Roll(RollArg::D(ArgValue::Number(8))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Number(3),
                ],
                op: MacroOp::Add,
                result: StepResult::Ignore,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message("Smite!".to_string())),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(2))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                    Arg::Roll(RollArg::H(ArgValue::Number(1))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Pass,
                value: None,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message("I rolled a ".to_string())),
                    Arg::Variable("1".to_string()),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
                value: None,
            },
        ],
    };
    let (_, result) = parse_p(b"#complex-macro-name-2 !roll 3d8+3 !say \"Smite!\" !roll 2d20kh1 >> !say \"I rolled a \" $1").unwrap();
    assert_eq!(result, program);
}

#[test]
fn test_name_parser() {
    let (_, result) = name_p(b"#macro_name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro_name")));

    let (_, result) = name_p(b"#macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro-name")));

    let (_, result) = name_p(b"#123macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("123macro-name")));

    let (_, result) = name_p(b"#Z123macro-name").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("Z123macro-name")));

    let (_, result) = name_p(b"#macro-name cannot have spaces").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("macro-name")));

    let bad_result = name_p(b"macro_name");
    assert_eq!(bad_result, IResult::Error(ErrorKind::Custom(1)))
}

#[test]
fn test_command_parser_roll() {
    let (_, result) = command_p(b"!roll 1d20").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command_p(b"!r 1d20").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command_p(b"!roll advantage").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command_p(b"!roll adv").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command_p(b"!roll disadvantage").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = command_p(b"!roll dis").unwrap();
    assert_eq!(result, MacroOp::Roll);
}

#[test]
fn test_op_parser() {
    let (_, result) = op_p(b"    #test-macro   ").unwrap();
    assert_eq!(result, MacroOp::Name(String::from("test-macro")));
    let (_, result) = op_p(b"    !roll 1d20 ").unwrap();
    assert_eq!(result, MacroOp::Roll);
    let (_, result) = op_p(b"   !say \"Hello!\" ").unwrap();
    assert_eq!(result, MacroOp::Say);
    let (_, result) = op_p(b"   !whisper").unwrap();
    assert_eq!(result, MacroOp::Whisper);
}

#[test]
fn test_arguments_parser() {
    let (_, result) = arguments_p(b"\"hello\"").unwrap();
    assert_eq!(result, Arg::Unrecognized(String::from("hello")));
    let (_, result) = arguments_p(b"   Hello  ").unwrap();
    assert_eq!(result, Arg::Unrecognized(String::from("Hello")));
    let (_, result) = arguments_p(b"'   Single String Args'").unwrap();
    assert_eq!(result, Arg::Unrecognized(String::from("Single String Args")));
}

#[test]
fn test_quoted_parser() {
    let (_, result) = quoted_p(b"\"hello\"").unwrap();
    assert_eq!(result, String::from("hello"));
    let (_, result) = quoted_p(b"\"   Hello  \"").unwrap();
    assert_eq!(result, String::from("Hello  "));
}

#[test]
fn test_single_quoted_parser() {
    let (_, result) = single_quoted_p(b"'test 123'").unwrap();
    assert_eq!(result, String::from("test 123"));
    let (_, result) = single_quoted_p(b"'   Single String Args'").unwrap();
    assert_eq!(result, String::from("Single String Args"));
}

#[test]
fn test_step_result_parser() {
    let (_, result) = step_result_p(b">>").unwrap();
    assert_eq!(result, StepResult::Pass);

    let (_, result) = step_result_p(b" ").unwrap();
    assert_eq!(result, StepResult::Ignore);
}

#[test]
fn test_arguments_roll_parser() {
    // Pass it through once should yield us the N and remove a "d"
    let (rest, result) = arguments_roll_p(b"1d20").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::N(ArgValue::Number(1))));
    // Running through a second time will yield us the D
    let (_, result) = arguments_roll_p(rest).unwrap();
    assert_eq!(result, Arg::Roll(RollArg::D(ArgValue::Number(20))));

    // Advantage
    let (_, result) = arguments_roll_p(b"adv").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Advantage));
    let (_, result) = arguments_roll_p(b"advantage").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Advantage));

    // Disadvantage
    let (_, result) = arguments_roll_p(b"dis").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Disadvantage));
    let (_, result) = arguments_roll_p(b"disadvantage").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Disadvantage));

    // Comment
    let (_, result) = arguments_roll_p(b"\"I am a comment\"").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::Text("I am a comment".to_string()))));

    // Variables

    // N
    let (_, result) = arguments_roll_p(b"$1d20").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))));
    // D
    let (rest, _) = arguments_roll_p(b"1d$1").unwrap();
    let (_, result) = arguments_roll_p(rest).unwrap();
    assert_eq!(result, Arg::Roll(RollArg::D(ArgValue::VariableReserved(1))));
    // E
    let (_, result) = roll_flag_e_p(b"e$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::E(ArgValue::VariableReserved(1))));
    // H
    let (_, result) = roll_flag_h_p(b"kh$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::H(ArgValue::VariableReserved(1))));
    // L
    let (_, result) = roll_flag_l_p(b"kl$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::L(ArgValue::VariableReserved(1))));
    // RO
    let (_, result) = roll_flag_ro_p(b"ro$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::RO(ArgValue::VariableReserved(1))));
    // RR
    let (_, result) = roll_flag_rr_p(b"rr$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::RR(ArgValue::VariableReserved(1))));
}

#[test]
fn test_arguments_whisper_parser() {
    let (_, result) = arguments_whisper_p(b"\"I am a message\"").unwrap();
    assert_eq!(result, Arg::Say(SayArg::Message("I am a message".to_string())));

    let (_, result) = arguments_whisper_p(b"$me").unwrap();
    assert_eq!(result, Arg::Say(SayArg::To("me".to_string())));
}

#[test]
fn test_error_handling() {
    let result = name_p(b"invalid input").unwrap_err();
    assert_eq!(error_to_string(result), "Missing or invalid macro name".to_string());

    let result = command_p(b"invalid input").unwrap_err();
    assert_eq!(error_to_string(result), "Invalid or unrecognized command".to_string());
}

#[test]
fn test_token_parser() {
    let (_, result) = token_p(b"@foo").unwrap();
    assert_eq!(result, "foo".to_string());

    let (_, result) = token_p(b"@foo123bar").unwrap();
    assert_eq!(result, "foo123bar".to_string());
}

#[test]
fn test_variable_parser() {
    let (_, result) = variable_p(b"$foo").unwrap();
    assert_eq!(result, "foo".to_string());

    let (_, result) = variable_p(b"$foo123bar").unwrap();
    assert_eq!(result, "foo123bar".to_string());
}

#[test]
fn test_variable_reserved_parser() {
    let (_, result) = variable_reserved_p(b"$1").unwrap();
    assert_eq!(result, 1);

    let (_, result) = variable_reserved_p(b"$12").unwrap();
    assert_eq!(result, 12);
}
