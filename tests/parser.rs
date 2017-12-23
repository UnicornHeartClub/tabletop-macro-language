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
                    Arg::Say(SayArg::Message("Hello, world!".to_string())),
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
                    Arg::Roll(RollArg::N(ArgValue::Number(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(20))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Save,
            },
            Step {
                args: vec![
                    Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))),
                    Arg::Roll(RollArg::D(ArgValue::Number(8))),
                    Arg::Roll(RollArg::Comment(ArgValue::Text("A cool roll comment".to_string()))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message("Smite!".to_string())),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
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
                    Arg::Roll(RollArg::Min(ArgValue::Number(8))),
                    Arg::Roll(RollArg::Max(ArgValue::Number(16))),
                    Arg::Roll(RollArg::ModifierPos(ArgValue::Number(3))),
                ],
                op: MacroOp::Roll,
                result: StepResult::Ignore,
            },
            Step {
                args: vec![
                    Arg::Say(SayArg::Message("Smite!".to_string())),
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
                    Arg::Say(SayArg::Message("I rolled a ".to_string())),
                    Arg::Variable("1".to_string()),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#complex-macro-name-2 !roll 3d8min8max16+3 !say \"Smite!\" !roll 2d20-5kh1 >> !say \"I rolled a \" $1").unwrap();
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
                                Arg::Say(SayArg::Message("Success".to_string())),
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
                    Arg::Say(SayArg::Message("Mod is ".to_string())),
                    Arg::Variable("mod".to_string()),
                ],
                op: MacroOp::Say,
                result: StepResult::Ignore,
            },
        ],
    };
    let (_, result) = parse_p(b"#test 5 < 10 ? $mod = 1 : $mod = 2 !say 'Mod is ' $mod").unwrap();
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
    assert_eq!(result, StepResult::Save);

    let (_, result) = step_result_p(b" ").unwrap();
    assert_eq!(result, StepResult::Ignore);

    let (_, result) = step_result_p(b"|").unwrap();
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

    // min
    let (_, result) = roll_flag_min_p(b"min2").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Min(ArgValue::Number(2))));

    // max
    let (_, result) = roll_flag_max_p(b"max22").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Max(ArgValue::Number(22))));

    // Comment
    let (_, result) = arguments_roll_p(b"\"I am a comment\"").unwrap();
    assert_eq!(result, Arg::Roll(RollArg::Comment(ArgValue::Text("I am a comment".to_string()))));

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

    let (_, result) = arguments_roll_p(b"lte$foo").unwrap();
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
fn test_arguments_whisper_parser() {
    let (_, result) = arguments_whisper_p(b"\"I am a message\"").unwrap();
    assert_eq!(result, Arg::Say(SayArg::Message("I am a message".to_string())));

    let (_, result) = arguments_whisper_p(b"@me").unwrap();
    assert_eq!(result, Arg::Say(SayArg::To(TokenArg {
        name: "me".to_string(),
        attribute: None,
        macro_name: None,
    })));

    let (_, result) = arguments_whisper_p(b"$var").unwrap();
    assert_eq!(result, Arg::Variable("var".to_string()));

    // we should be able to combine strings
    let (_, result) = parse_p(b"#whisper !w 'Rolled a ' $foo @npc1").unwrap();
    let steps = result.steps;
    assert_eq!(steps[0].args[0], Arg::Say(SayArg::Message("Rolled a ".to_string())));
    assert_eq!(steps[0].args[1], Arg::Variable("foo".to_string()));
    assert_eq!(steps[0].args[2], Arg::Say(SayArg::To(TokenArg {
        name: "npc1".to_string(),
        attribute: None,
        macro_name: None,
    })));
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
    assert_eq!(result, TokenArg { name: "foo".to_string(), attribute: None, macro_name: None });

    let (_, result) = token_p(b"@foo123bar.baz").unwrap();
    assert_eq!(result, TokenArg { name: "foo123bar".to_string(), attribute: Some("baz".to_string()), macro_name: None });

    let (_, result) = token_p(b"@foo_53_test").unwrap();
    assert_eq!(result, TokenArg { name: "foo_53_test".to_string(), attribute: None, macro_name: None });

    let (_, result) = token_p(b"@foo_bar.baz_bo").unwrap();
    assert_eq!(result, TokenArg { name: "foo_bar".to_string(), attribute: Some("baz_bo".to_string()), macro_name: None });

    let (_, result) = token_p(b"@fooZ->my_test_func").unwrap();
    assert_eq!(result, TokenArg { name: "fooZ".to_string(), attribute: None, macro_name: Some("my_test_func".to_string()) });
}

#[test]
fn test_variable_parser() {
    let (_, result) = variable_p(b"$foo").unwrap();
    assert_eq!(result, "foo".to_string());

    let (_, result) = variable_p(b"$foo123bar").unwrap();
    assert_eq!(result, "foo123bar".to_string());

    let (_, result) = variable_p(b"$foo_bar").unwrap();
    assert_eq!(result, "foo_bar".to_string());
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
        right: vec![ ArgValue::Text("foo".to_string()) ],
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
        right: vec![ ArgValue::Text("foo".to_string()) ],
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
    let mut options = HashMap::new();
    options.insert("Label".to_string(), ArgValue::Text("Label".to_string()));
    options.insert("Label 2".to_string(), ArgValue::Text("Label 2".to_string()));
    options.insert("Label 3".to_string(), ArgValue::Text("Label 3".to_string()));

    let prompt = Arg::Prompt(Prompt {
        message: "Choose your style".to_string(),
        options,
    });
    let (_, result) = arguments_prompt_p(b"'Choose your style' [Label, Label 2, 'Label 3']").unwrap();
    assert_eq!(result, prompt);

    // with options and values
    let mut options = HashMap::new();
    options.insert("foo".to_string(), ArgValue::Text("bar".to_string()));
    options.insert("1".to_string(), ArgValue::Token(TokenArg {
        name: "me".to_string(),
        attribute: Some("attribute".to_string()),
        macro_name: None,
    }));
    options.insert("baz".to_string(), ArgValue::Text("boo".to_string()));

    let prompt = Arg::Prompt(Prompt {
        message: "Choose your thing".to_string(),
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
