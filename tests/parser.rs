// @todo
// These tests are written like junk
// I want to refactor this to use a more elegant Rust testing framework where I can write scenarios
// such as "it_parses_a_complete_say_command", "it_parses_say_command_arguments", etc

extern crate ttml;
extern crate nom;

use nom::{IResult, ErrorKind};
use std::collections::HashMap;
use ttml::arg::*;
use ttml::parser::*;
use ttml::step::*;

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
        }],
    };
    let (_, result) = parse_p(b"#simple-macro-name !roll 1d20").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("simple-macro-name-2")),
        steps: vec![
            Step {
                args: vec![],
                op: MacroOp::Exit,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message(TextInterpolated {
                        parts: vec! [
                            ArgValue::Text("Hello, world!".to_string()),
                        ],
                    })),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            }
        ],
    };
    let (_, result) = parse_p(b"#simple-macro-name-2 !exit !say \"Hello, world!\"").unwrap();
    assert_eq!(result, program);
}

#[test]
fn test_complex_parser() {
    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Assign(Assign {
                        left: ArgValue::Variable("foo".to_string()),
                        right: vec![ ArgValue::Number(1) ],
                    }),
                ],
                op: MacroOp::Lambda,
                result: StepResult::Save,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Save,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Variable("foo".to_string()))),
                    Arg::Roll(RollArg::D(ArgValue::VariableReserved(1))),
                    Arg::Roll(RollArg::Comment(ArgValue::TextInterpolated(TextInterpolated {
                        parts: vec![ ArgValue::Text("A cool roll comment".to_string()) ],
                    }))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message(TextInterpolated {
                        parts: vec! [
                            ArgValue::Text("Smite!".to_string()),
                        ],
                    })),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#complex-macro-name $foo = 1 >> !r 1d20 >> !roll ${foo}d$1 \"A cool roll comment\" !say \"Smite!\"").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("complex-macro-name-2")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(3))),
                    Arg::Roll(RollArg::D(ArgValue::Number(8))),
                    Arg::Roll(RollArg::Min(ArgValue::Number(8))),
                    Arg::Roll(RollArg::Max(ArgValue::Number(16))),
                    Arg::Roll(RollArg::ModifierPos(ArgValue::Number(3))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message(TextInterpolated {
                        parts: vec! [
                            ArgValue::Text("Smite!".to_string()),
                        ],
                    })),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(2))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                    Arg::Roll(RollArg::ModifierNeg(ArgValue::Number(5))),
                    Arg::Roll(RollArg::H(ArgValue::Number(1))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Save,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message(TextInterpolated {
                        parts: vec! [
                            ArgValue::Text("I rolled a ".to_string()),
                            ArgValue::VariableReserved(1),
                        ],
                    })),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#complex-macro-name-2 !roll 3d8min8max16+3 !say \"Smite!\" !roll 2d20-5kh1 >> !say \"I rolled a $1\"").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("test-assignment")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Assign(Assign {
                        left: ArgValue::Variable("foo".to_string()),
                        right: vec![ ArgValue::Text("bar".to_string()) ],
                    }),
                ],
                op: MacroOp::Lambda,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#test-assignment $foo = 'bar' !r 1d20").unwrap();
    assert_eq!(result, program);

    let program = Program {
        name: MacroOp::Name(String::from("test")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::Number(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20)))
                ],
                op: MacroOp::Roll,
                result: StepResult::Save,
            },
            Step {
                args: vec![
                    Arg::Conditional(Conditional {
                        left: ArgValue::VariableReserved(1),
                        comparison: ComparisonArg::GreaterThan,
                        right: ArgValue::Number(10),
                        success: Some(Step {
                            args: vec![
                                Arg::Say(SayArg::Message(TextInterpolated {
                                    parts: vec![
                                        ArgValue::Text("Success".to_string()),
                                    ],
                                })),
                            ],
                            op: MacroOp::Say,
                            result: StepResult::Ignore,
                        }),
                        failure: None,
                    }),
                ],
                op: MacroOp::Lambda,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#test !r 1d20 >> $1 > 10 ? !say \"Success\" :|").unwrap();
    assert_eq!(result, program);

    // Make sure we can set variables on a condition and parse a step after
    let program = Program {
        name: MacroOp::Name(String::from("test")),
        steps: vec![
            Step {
                args: vec![
                    Arg::Conditional(Conditional {
                        left: ArgValue::Number(5),
                        comparison: ComparisonArg::LessThan,
                        right: ArgValue::Number(10),
                        success: Some(Step {
                            args: vec![
                                Arg::Assign(Assign {
                                    left: ArgValue::Variable("mod".to_string()),
                                    right: vec![ ArgValue::Number(1) ],
                                }),
                            ],
                            op: MacroOp::Lambda,
                            result: StepResult::Ignore,
                        }),
                        failure: Some(Step {
                            args: vec![
                                Arg::Assign(Assign {
                                    left: ArgValue::Variable("mod".to_string()),
                                    right: vec![ ArgValue::Number(2) ],
                                }),
                            ],
                            op: MacroOp::Lambda,
                            result: StepResult::Ignore,
                        }),
                    }),
                ],
                op: MacroOp::Lambda,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message(TextInterpolated {
                        parts: vec![
                            ArgValue::Text("Mod is ".to_string()),
                            ArgValue::Variable("mod".to_string()),
                            ArgValue::Text(" trailing space test  ".to_string()),
                        ],
                    })),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#test 5 < 10 ? $mod = 1 : $mod = 2 !say \"Mod is $mod trailing space test  \"").unwrap();
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

    // let bad_result = name_p(b"macro_name");
    // assert_eq!(bad_result, IResult::Error(ErrorKind::Custom(1)))
}

#[test]
fn test_command_parser_test() {
    let (_, result) = command_p(b"!test true").unwrap();
    assert_eq!(result, MacroOp::TestMode);

    let (_, result) = command_p(b"!test false").unwrap();
    assert_eq!(result, MacroOp::TestMode);
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
fn test_command_parser_prompt() {
    let (_, result) = command_p(b"!prompt 'Choose your style' [Style 1, Style 2]").unwrap();
    assert_eq!(result, MacroOp::Prompt);
    let (_, result) = command_p(b"!p 'Enter some text'").unwrap();
    assert_eq!(result, MacroOp::Prompt);
}

#[test]
fn test_command_parser_target() {
    let (_, result) = command_p(b"!target 'Choose a target'").unwrap();
    assert_eq!(result, MacroOp::Target);
    let (_, result) = command_p(b"!t 'Choose a target'").unwrap();
    assert_eq!(result, MacroOp::Target);
}

#[test]
fn test_command_parser_template() {
    let (_, result) = command_p(b"!template 'template_name'").unwrap();
    assert_eq!(result, MacroOp::Template);
}

#[test]
fn test_command_parser_input() {
    let (_, result) = command_p(b"!input 'Type your input'").unwrap();
    assert_eq!(result, MacroOp::Input);
    let (_, result) = command_p(b"!i 'Enter some text'").unwrap();
    assert_eq!(result, MacroOp::Input);
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
    assert_eq!(result, Arg::Unrecognized(ArgValue::TextInterpolated(TextInterpolated {
        parts: vec![ ArgValue::Text("hello".to_string()) ],
    })));

    let (_, result) = arguments_p(b"   Hello  ").unwrap();
    assert_eq!(result, Arg::Unrecognized(ArgValue::Text("Hello".to_string())));

    let (_, result) = arguments_p(b"'   Single String Args'").unwrap();
    assert_eq!(result, Arg::Unrecognized(ArgValue::Text("   Single String Args".to_string())));
}

#[test]
fn test_quoted_interpolated_parser() {
    let (_, result) = quoted_interpolated_p(b"\"Hello @{token}.attribute, it's good to see you\"").unwrap();
    assert_eq!(result, TextInterpolated {
        parts: vec![
            ArgValue::Text("Hello ".to_string()),
            ArgValue::Token(TokenArg {
                name: "token".to_string(),
                attribute: Some("attribute".to_string()),
                macro_name: None,
            }),
            ArgValue::Text(", it's good to see you".to_string())
        ],
    });

    let (_, result) = quoted_interpolated_p(b"\"There is activity at $place bar\"").unwrap();
    assert_eq!(result, TextInterpolated {
        parts: vec![
            ArgValue::Text("There is activity at ".to_string()),
            ArgValue::Variable("place".to_string()),
            ArgValue::Text(" bar".to_string())
        ],
    });

    let (_, result) = quoted_interpolated_p(b"\"Hey bartender, @{bartender}.name! Get me an ale of ${beer}!\"").unwrap();
    assert_eq!(result, TextInterpolated {
        parts: vec![
            ArgValue::Text("Hey bartender, ".to_string()),
            ArgValue::Token(TokenArg {
                name: "bartender".to_string(),
                attribute: Some("name".to_string()),
                macro_name: None,
            }),
            ArgValue::Text("! Get me an ale of ".to_string()),
            ArgValue::Variable("beer".to_string()),
            ArgValue::Text("!".to_string()),
        ],
    });
}

#[test]
fn test_single_quoted_parser() {
    let (_, result) = single_quoted_p(b"'test 123'").unwrap();
    assert_eq!(result, String::from("test 123"));
    let (_, result) = single_quoted_p(b"'   Single String Args'").unwrap();
    assert_eq!(result, String::from("   Single String Args"));
}

#[test]
fn test_step_result_parser() {
    let (_, result) = step_result_p(b">>").unwrap();
    assert_eq!(result, StepResult::Save);

    let (_, result) = step_result_p(b" ").unwrap();
    assert_eq!(result, StepResult::Ignore);

    let (_, result) = step_result_p(b"|").unwrap();
    assert_eq!(result, StepResult::Ignore);
}

#[test]
fn test_roll_parse_step_p () {
    let (_, result) = parse_step_p(b"!r 1d[-1, -2, 3, 7,9]").unwrap();
    assert_eq!(result, Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::Sides(vec![
                ArgValue::Number(-1),
                ArgValue::Number(-2),
                ArgValue::Number(3),
                ArgValue::Number(7),
                ArgValue::Number(9),
            ]))
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    });
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

    // min
    let (_, result) = roll_flag_min_p(b"min2").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Min(ArgValue::Number(2))));

    // max
    let (_, result) = roll_flag_max_p(b"max22").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Max(ArgValue::Number(22))));

    // Comment
    let (_, result) = arguments_roll_p(b"\"I am a comment\"").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::TextInterpolated(TextInterpolated {
        parts: vec![ ArgValue::Text("I am a comment".to_string()) ],
    }))));

    let (_, result) = arguments_roll_p(b"[I am also a comment]").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::Text("I am also a comment".to_string()))));

    let (_, result) = arguments_roll_p(b"['I am a comment in single quotes']").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::Text("I am a comment in single quotes".to_string()))));

    let (_, result) = arguments_roll_p(b"[\"Interpolated @{me}.attribute\"]").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::TextInterpolated(TextInterpolated {
        parts: vec![
            ArgValue::Text("Interpolated ".to_string()),
            ArgValue::Token(TokenArg {
                name: "me".to_string(),
                attribute: Some("attribute".to_string()),
                macro_name: None,
            }),
        ],
    }))));

    // Modifier
    let (_, result) = arguments_roll_p(b"+5").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::ModifierPos(ArgValue::Number(5))));

    let (_, result) = arguments_roll_p(b"+$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::ModifierPos(ArgValue::VariableReserved(1))));

    // gt, gte, lt, lte
    let (_, result) = arguments_roll_p(b"gt12").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::GT(ArgValue::Number(12))));

    let (_, result) = arguments_roll_p(b"gte20").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::GTE(ArgValue::Number(20))));

    let (_, result) = arguments_roll_p(b"lt$1").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::LT(ArgValue::VariableReserved(1))));

    let (_, result) = arguments_roll_p(b"lte${foo}").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::LTE(ArgValue::Variable("foo".to_string()))));

    // Token Modifier
    let (_, result) = arguments_roll_p(b"+@me.dexterity").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::ModifierPos(ArgValue::Token(TokenArg {
        name: "me".to_string(),
        attribute: Some("dexterity".to_string()),
        macro_name: None,
    }))));

    // Token argument
    let (_, result) = arguments_roll_p(b"@me").unwrap();
    assert_eq!(result, Arg::Token(TokenArg {
        name: "me".to_string(),
        attribute: None,
        macro_name: None,
    }));

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
fn test_arguments_roll_parses_token_attributes() {
    let (_, result) = roll_modifier_pos_p(b"+@me.dexterity").unwrap();
    assert_eq!(
        result,
        Arg::Roll(RollArg::ModifierPos(ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("dexterity".to_string()),
            macro_name: None,
        })))
    );
}

#[test]
fn test_arguments_test_mode_parser() {
    let (_, result) = parse_p(b"#test !test true").unwrap();
    let arg = &result.steps[0].args[0];
    assert_eq!(arg, &Arg::TestMode(true));

    let (_, result) = parse_p(b"#test !test false").unwrap();
    let arg = &result.steps[0].args[0];
    assert_eq!(arg, &Arg::TestMode(false));
}

#[test]
fn it_parses_a_complete_say_command() {
    // we should be able to combine strings
    let (_, result) = parse_p(b"#test !say \"@{target}.name's AC\"").unwrap();
    let steps = result.steps;
    assert_eq!(steps[0].args[0], Arg::Say(SayArg::Message(TextInterpolated {
        parts: vec! [
            ArgValue::Token(TokenArg {
                name: "target".to_string(),
                attribute: Some("name".to_string()),
                macro_name: None,
            }),
            ArgValue::Text("'s AC".to_string()),
        ],
    })));
}

#[test]
fn it_parses_a_complete_roll_command() {
    // we should be able to combine strings
    let (_, result) = parse_p(b"#test !roll 1d20 + 4 [Comment]").unwrap();
    let steps = result.steps;

    assert_eq!(steps[0].args, vec![
        Arg::Roll(RollArg::N(ArgValue::Number(1))),
        Arg::Roll(RollArg::D(ArgValue::Number(20))),
        Arg::Roll(RollArg::ModifierPos(ArgValue::Number(4))),
        Arg::Roll(RollArg::Comment(ArgValue::Text("Comment".to_string()))),
    ]);
}

#[test]
fn it_parses_a_complete_hidden_roll_command() {
    // we should be able to combine strings
    let (_, result) = parse_p(b"#test !hroll 4d20+2").unwrap();
    let steps = result.steps;

    assert_eq!(steps[0].op, MacroOp::RollHidden);
    assert_eq!(steps[0].args, vec![
        Arg::Roll(RollArg::N(ArgValue::Number(4))),
        Arg::Roll(RollArg::D(ArgValue::Number(20))),
        Arg::Roll(RollArg::ModifierPos(ArgValue::Number(2))),
    ]);

    let (_, result) = parse_p(b"#test !hr ${foo}d4+@{me}.dexterity [Comment with spaces]").unwrap();
    let steps = result.steps;

    assert_eq!(steps[0].op, MacroOp::RollHidden);
    assert_eq!(steps[0].args, vec![
        Arg::Roll(RollArg::N(ArgValue::Variable("foo".to_string()))),
        Arg::Roll(RollArg::D(ArgValue::Number(4))),
        Arg::Roll(RollArg::ModifierPos(ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("dexterity".to_string()),
            macro_name: None,
        }))),
        Arg::Roll(RollArg::Comment(ArgValue::Text("Comment with spaces".to_string())))
    ]);
}

#[test]
fn it_parses_a_complete_whisper_roll_command() {
    // we should be able to combine strings
    let (_, result) = parse_p(b"#test !wroll @gm 4d20+2").unwrap();
    let steps = result.steps;

    assert_eq!(steps[0].op, MacroOp::RollWhisper);
    assert_eq!(steps[0].args, vec![
        Arg::Token(TokenArg {
            name: "gm".to_string(),
            attribute: None,
            macro_name: None,
        }),
        Arg::Roll(RollArg::N(ArgValue::Number(4))),
        Arg::Roll(RollArg::D(ArgValue::Number(20))),
        Arg::Roll(RollArg::ModifierPos(ArgValue::Number(2))),
    ]);
}

#[test]
fn test_arguments_whisper_parser() {
    let (_, result) = arguments_whisper_p(b"\"I am a message\"").unwrap();
    assert_eq!(result, Arg::Say(SayArg::Message(TextInterpolated {
        parts: vec![
            ArgValue::Text("I am a message".to_string()),
        ],
    })));

    let (_, result) = arguments_whisper_p(b"@me").unwrap();
    assert_eq!(result, Arg::Say(SayArg::To(TokenArg {
        name: "me".to_string(),
        attribute: None,
        macro_name: None,
    })));

    // we should be able to combine strings
    let (_, result) = parse_p(b"#whisper !w @npc1 \"Rolled a ${foo}\"").unwrap();
    let steps = result.steps;
    assert_eq!(steps[0].args[0], Arg::Say(SayArg::To(TokenArg {
        name: "npc1".to_string(),
        attribute: None,
        macro_name: None,
    })));
    assert_eq!(steps[0].args[1], Arg::Say(SayArg::Message(TextInterpolated {
        parts: vec! [
            ArgValue::Text("Rolled a ".to_string()),
            ArgValue::Variable("foo".to_string()),
        ],
    })));
}

#[test]
fn test_token_parser() {
    let (_, result) = token_p(b"@foo").unwrap();
    assert_eq!(result, TokenArg { name: "foo".to_string(), attribute: None, macro_name: None });

    let (_, result) = token_p(b"@{faz}").unwrap();
    assert_eq!(result, TokenArg { name: "faz".to_string(), attribute: None, macro_name: None });

    let (_, result) = token_p(b"@foo123bar.baz").unwrap();
    assert_eq!(result, TokenArg { name: "foo123bar".to_string(), attribute: Some("baz".to_string()), macro_name: None });

    let (_, result) = token_p(b"@foo_53_test").unwrap();
    assert_eq!(result, TokenArg { name: "foo_53_test".to_string(), attribute: None, macro_name: None });

    let (_, result) = token_p(b"@foo_bar.baz_bo").unwrap();
    assert_eq!(result, TokenArg { name: "foo_bar".to_string(), attribute: Some("baz_bo".to_string()), macro_name: None });

    let (_, result) = token_p(b"@fooZ->my_test_func").unwrap();
    assert_eq!(result, TokenArg { name: "fooZ".to_string(), attribute: None, macro_name: Some("my_test_func".to_string()) });

    let (_, result) = token_p(b"@foo.{attacks.0.bar}").unwrap();
    assert_eq!(result, TokenArg {
        name: "foo".to_string(), 
        attribute: Some("attacks.0.bar".to_string()),
        macro_name: None,
    });

    let (_, result) = token_p(b"@{foo}.{0.1}").unwrap();
    assert_eq!(result, TokenArg {
        name: "foo".to_string(), 
        attribute: Some("0.1".to_string()),
        macro_name: None,
    });
}

#[test]
fn test_variable_parser() {
    let (_, result) = variable_p(b"$foo").unwrap();
    assert_eq!(result, "foo".to_string());

    let (_, result) = variable_p(b"$foo123bar").unwrap();
    assert_eq!(result, "foo123bar".to_string());

    let (_, result) = variable_p(b"$foo_bar").unwrap();
    assert_eq!(result, "foo_bar".to_string());

    let (_, result) = variable_p(b"${foobar}").unwrap();
    assert_eq!(result, "foobar".to_string());
}

#[test]
fn test_variable_reserved_parser() {
    let (_, result) = variable_reserved_p(b"$1").unwrap();
    assert_eq!(result, 1);

    let (_, result) = variable_reserved_p(b"$12").unwrap();
    assert_eq!(result, 12);
}

#[test]
fn test_assign_token_parser() {
    // assign strings
    let (_, result) = arguments_p(b"@me.test = 'foo'").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("test".to_string()),
            macro_name: None,
        }),
        right: vec![ ArgValue::Text("foo".to_string()) ],
    });

    assert_eq!(result, assign);

    let (_, result) = arguments_p(b" @me.test  =   \"foo\"   ").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("test".to_string()),
            macro_name: None,
        }),
        right: vec![ ArgValue::TextInterpolated(TextInterpolated {
            parts: vec! [ ArgValue::Text("foo".to_string()) ],
        }) ],
    });

    assert_eq!(result, assign);

    // assign numbers
    let (_, result) = arguments_p(b"@me.test  =  42").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("test".to_string()),
            macro_name: None,
        }),
        right: vec![ ArgValue::Number(42) ],
    });

    assert_eq!(result, assign);

    // assign floats
    let (_, result) = arguments_p(b"@me.test  =  -124.222").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("test".to_string()),
            macro_name: None,
        }),
        right: vec![ ArgValue::Float(-124.222) ],
    });

    assert_eq!(result, assign);

    // assign expressions
    let (_, result) = arguments_p(b"@me.bar= 1 / 3").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("bar".to_string()),
            macro_name: None,
        }),
        right: vec![
            ArgValue::Number(1),
            ArgValue::Primitive(Primitive::Divide),
            ArgValue::Number(3),
        ],
    });

    assert_eq!(result, assign);
}

#[test]
fn test_assign_variable_parser() {
    // assign strings
    let (_, result) = arguments_p(b"$foo = 'baz'").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("foo".to_string()),
        right: vec![ ArgValue::Text("baz".to_string()) ],
    });

    assert_eq!(result, assign);

    let (_, result) = arguments_p(b" $foo  =   \"foo\"   ").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("foo".to_string()),
        right: vec![
            ArgValue::TextInterpolated(TextInterpolated {
                parts: vec![
                    ArgValue::Text("foo".to_string()),
                ],
            }),
        ],
    });

    assert_eq!(result, assign);

    // assign numbers
    let (_, result) = arguments_p(b"$foo  =  42").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("foo".to_string()),
        right: vec![ ArgValue::Number(42) ],
    });

    assert_eq!(result, assign);

    // assign expressions
    let (_, result) = arguments_p(b"$foo  =  1 + 2").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("foo".to_string()),
        right: vec![
            ArgValue::Number(1),
            ArgValue::Primitive(Primitive::Add),
            ArgValue::Number(2),
        ],
    });

    assert_eq!(result, assign);

    // assign booleans
    let (_, result) = arguments_p(b"$baz= true").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("baz".to_string()),
        right: vec![ ArgValue::Boolean(true) ],
    });

    assert_eq!(result, assign);

    let (_, result) = arguments_p(b"$bal = false").unwrap();
    let assign = Arg::Assign(Assign {
        left: ArgValue::Variable("bal".to_string()),
        right: vec![ ArgValue::Boolean(false) ],
    });

    assert_eq!(result, assign);
}

#[test]
fn test_arguments_prompt_parser() {
    // with options
    let options = vec![
        PromptOption { key: Some("Label".to_string()), value: ArgValue::Text("Label".to_string()) },
        PromptOption { key: Some("Label 2".to_string()), value: ArgValue::Text("Label 2".to_string()) },
        PromptOption { key: Some("Label 3".to_string()), value: ArgValue::Text("Label 3".to_string()) },
    ];

    let prompt = Arg::Prompt(Prompt {
        message: TextInterpolated {
            parts: vec![
                ArgValue::Text("Choose your style".to_string()),
            ],
        },
        options,
    });
    let (_, result) = arguments_prompt_p(b"'Choose your style' [Label, Label 2, 'Label 3']").unwrap();
    assert_eq!(result, prompt);

    // with options and values
    let options = vec![
        PromptOption { key: Some("foo".to_string()), value: ArgValue::Text("bar".to_string()) },
        PromptOption {
            key: None,
            value: ArgValue::Token(TokenArg {
                name: "me".to_string(),
                attribute: Some("attribute".to_string()),
                macro_name: None,
            })
        },
        PromptOption {
            key: Some("baz".to_string()),
            value: ArgValue::TextInterpolated(TextInterpolated {
                parts: vec![ ArgValue::Text("boo".to_string()) ],
            }),
        },
    ];

    let prompt = Arg::Prompt(Prompt {
        message: TextInterpolated {
            parts: vec![
                ArgValue::Text("Choose your thing".to_string()),
            ],
        },
        options,
    });
    let (_, result) = arguments_prompt_p(b"\"Choose your thing\" [foo:bar, @me.attribute, 'baz':\"boo\"]").unwrap();
    assert_eq!(result, prompt);

    // // no options
    // let prompt = Arg::Prompt(Prompt {
        // message: "Choose your style".to_string(),
        // options: HashMap::new(),
    // });
    // let (_, result) = arguments_prompt_p(b"'Choose your style' []").unwrap();
    // assert_eq!(result, prompt);

    // let prompt = Arg::Prompt(Prompt {
        // message: "What's your beef?".to_string(),
        // options: HashMap::new(),
    // });
    // let (_, result) = arguments_prompt_p(b"\"What's your beef?\"").unwrap();
    // assert_eq!(&result, &prompt);
}

#[test]
fn test_conditional_parser() {
    // compare greater than
    let (_, result) = arguments_p(b"$foo > 1 ? !r 1d20 : !r 1d8").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Variable("foo".to_string()),
        comparison: ComparisonArg::GreaterThan,
        right: ArgValue::Number(1),
        success: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20)))
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
        failure: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(8)))
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
    });

    assert_eq!(result, compare);

    // ignoring results
    let (_, result) = arguments_p(b"$foo <= 5 ? !r 1d20 : |").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Variable("foo".to_string()),
        comparison: ComparisonArg::LessThanOrEqual,
        right: ArgValue::Number(5),
        success: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20)))
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
        failure: None,
    });

    assert_eq!(result, compare);

    let (_, result) = arguments_p(b"$foo >= -5 ? | : !r 1d20").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Variable("foo".to_string()),
        comparison: ComparisonArg::GreaterThanOrEqual,
        right: ArgValue::Number(-5),
        success: None,
        failure: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20)))
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
    });

    assert_eq!(result, compare);

    // equal to
    let (_, result) = arguments_p(b"$foo == 10 ? !r 1d20+5 : !r 1d20").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Variable("foo".to_string()),
        comparison: ComparisonArg::EqualTo,
        right: ArgValue::Number(10),
        success: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20))),
                Arg::Roll(RollArg::ModifierPos(ArgValue::Number(5))),
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
        failure: Some(Step {
            args: vec![
                Arg::Roll(RollArg::N(ArgValue::Number(1))),
                Arg::Roll(RollArg::D(ArgValue::Number(20))),
            ],
            op: MacroOp::Roll,
            result: StepResult::Ignore,
        }),
    });

    assert_eq!(result, compare);
}

#[test]
fn test_conditional_parser_does_assignments() {
    let (_, result) = arguments_p(b"10 == 10 ? $foo = 1 : $foo = 2").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Number(10),
        comparison: ComparisonArg::EqualTo,
        right: ArgValue::Number(10),
        success: Some(Step {
            args: vec![
                Arg::Assign(Assign {
                    left: ArgValue::Variable("foo".to_string()),
                    right: vec![ ArgValue::Number(1) ]
                })
            ],
            op: MacroOp::Lambda,
            result: StepResult::Ignore,
        }),
        failure: Some(Step {
            args: vec![
                Arg::Assign(Assign {
                    left: ArgValue::Variable("foo".to_string()),
                    right: vec![ ArgValue::Number(2) ]
                })
            ],
            op: MacroOp::Lambda,
            result: StepResult::Ignore,
        }),
    });

    assert_eq!(result, compare);

    let (_, result) = arguments_p(b"@me.bar >= @me.foo ? $foo = 1 : $foo = 2").unwrap();
    let compare = Arg::Conditional(Conditional {
        left: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("bar".to_string()),
            macro_name: None,
        }),
        comparison: ComparisonArg::GreaterThanOrEqual,
        right: ArgValue::Token(TokenArg {
            name: "me".to_string(),
            attribute: Some("foo".to_string()),
            macro_name: None,
        }),
        success: Some(Step {
            args: vec![
                Arg::Assign(Assign {
                    left: ArgValue::Variable("foo".to_string()),
                    right: vec![ ArgValue::Number(1) ]
                })
            ],
            op: MacroOp::Lambda,
            result: StepResult::Ignore,
        }),
        failure: Some(Step {
            args: vec![
                Arg::Assign(Assign {
                    left: ArgValue::Variable("foo".to_string()),
                    right: vec![ ArgValue::Number(2) ]
                })
            ],
            op: MacroOp::Lambda,
            result: StepResult::Ignore,
        }),
    });

    assert_eq!(result, compare);
}

#[test]
fn test_json_parser() {
    let (_, result) = json_p(r#"{
        'foo': @me.attribute,
        "bar": 'Single quoted string',
        baz: "String interpolated",
        boo: -45.2,
        far: {
            out: $var_name
        },
        arr: [
            11,
            $1
        ]
    }"#.as_bytes()).unwrap();

    let mut object = HashMap::new();
    let mut nested_object = HashMap::new();
    nested_object.insert("out".to_string(), ArgValue::Variable("var_name".to_string()));

    object.insert("foo".to_string(), ArgValue::Token(TokenArg {
        name: "me".to_string(),
        attribute: Some("attribute".to_string()),
        macro_name: None,
    }));
    object.insert("bar".to_string(), ArgValue::Text("Single quoted string".to_string()));
    object.insert("baz".to_string(), ArgValue::TextInterpolated(TextInterpolated {
        parts: vec![ ArgValue::Text("String interpolated".to_string()) ],
    }));
    object.insert("boo".to_string(), ArgValue::Float(-45.2));
    object.insert("far".to_string(), ArgValue::Object(nested_object));
    object.insert("arr".to_string(), ArgValue::Array(vec![
        ArgValue::Number(11),
        ArgValue::VariableReserved(1),
    ]));

    assert_eq!(result, ArgValue::Object(object));
}

#[test]
fn test_template_parser() {
    let (_, result) = arguments_template_p(b"'template_name'").unwrap();
    assert_eq!(result, Arg::Template(TemplateArg::Name("template_name".to_string())));

    let (_, result) = arguments_template_p(b"template_name_2").unwrap();
    assert_eq!(result, Arg::Template(TemplateArg::Name("template_name_2".to_string())));

    let mut attributes = HashMap::new();
    attributes.insert("foo".to_string(), ArgValue::TextInterpolated(TextInterpolated {
        parts: vec![
            ArgValue::Text("bar".to_string()),
        ],
    }));
    let (_, result) = arguments_template_p(r#" {
        foo: "bar"
    }"#.as_bytes()).unwrap();
    assert_eq!(result, Arg::Template(TemplateArg::Attributes(ArgValue::Object(attributes))));

    let (_, result) = parse_p(r#"#test !template template_name {
        "foo": 'bar'
    }"#.as_bytes()).unwrap();

    let mut object = HashMap::new();
    object.insert("foo".to_string(), ArgValue::Text("bar".to_string()));
    let step = &result.steps[0];
    assert_eq!(step.op, MacroOp::Template);
    assert_eq!(step.args, vec![
        Arg::Template(TemplateArg::Name("template_name".to_string())),
        Arg::Template(TemplateArg::Attributes(ArgValue::Object(object))),
    ]);
}
