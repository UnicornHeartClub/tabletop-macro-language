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
    let (_, result) = parse_p(b"#simple-macro-name !roll 1d20").unwrap();
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
                    Argument {
                        arg: Arg::Roll(RollArg::N),
                        value: "1".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::D),
                        value: "20".to_string()
                    },
                    Argument {
                        arg: Arg::Roll(RollArg::Comment),
                        value: "A cool roll comment".to_string()
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
    let (_, result) = parse_p(b"#complex-macro-name !roll 1d20 \"A cool roll comment\" !say \"Smite!\"").unwrap();
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
    match bad_result {
        IResult::Error(e) => assert_eq!(e, ErrorKind::Tag),
        _ => assert_eq!(1, 0),
    }
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
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("hello") });
    let (_, result) = arguments_p(b"   Hello  ").unwrap();
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("Hello") });
    let (_, result) = arguments_p(b"'   Single String Args'").unwrap();
    assert_eq!(result, Argument { arg: Arg::Unrecognized, value: String::from("Single String Args") });
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
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::N),
        value: "1".to_string(),
    });
    // Running through a second time will yield us the D
    let (_, result) = arguments_roll_p(rest).unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::D),
        value: "20".to_string(),
    });

    // Advantage
    let (_, result) = arguments_roll_p(b"adv").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Advantage),
        value: "".to_string(),
    });
    let (_, result) = arguments_roll_p(b"advantage").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Advantage),
        value: "".to_string(),
    });

    // Disadvantage
    let (_, result) = arguments_roll_p(b"dis").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Disadvantage),
        value: "".to_string(),
    });
    let (_, result) = arguments_roll_p(b"disadvantage").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Disadvantage),
        value: "".to_string(),
    });

    // Comment
    let (_, result) = arguments_roll_p(b"\"I am a comment\"").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Roll(RollArg::Comment),
        value: "I am a comment".to_string(),
    });
}

#[test]
fn test_arguments_whisper_parser() {
    let (_, result) = arguments_whisper_p(b"\"I am a message\"").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Say(SayArg::Message),
        value: "I am a message".to_string(),
    });

    let (_, result) = arguments_whisper_p(b"$me").unwrap();
    assert_eq!(result, Argument {
        arg: Arg::Say(SayArg::To),
        value: "me".to_string(),
    });
}
