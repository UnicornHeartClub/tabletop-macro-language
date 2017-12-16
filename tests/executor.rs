extern crate ttml;

use ttml::arg::*;
use ttml::parser::*;
use ttml::step::*;
use ttml::output::Output;
use ttml::die::DieType;
use ttml::executor::{execute_macro, execute_roll};

#[test]
fn it_returns_a_roll() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    };

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 1);
    assert!(roll.value <= 20);
    assert_eq!(roll.dice.len(), 1);
}

#[test]
fn it_executes_roll_with_min_max_flags() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
            Arg::Roll(RollArg::Min(ArgValue::Number(2))),
            Arg::Roll(RollArg::Max(ArgValue::Number(3))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    };

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 2);
    assert!(roll.value <= 3);
    assert_eq!(roll.dice.len(), 1);

    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
            Arg::Roll(RollArg::Min(ArgValue::Number(200))),
            Arg::Roll(RollArg::Max(ArgValue::Number(300))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    };

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 200);
    assert!(roll.value <= 300);
    assert_eq!(roll.dice.len(), 1);
}

#[test]
fn it_uses_variables() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    };

    let mut output = Output::new("#test".to_string());
    output.results.insert("1".to_string(), StepValue::Number(5));
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 5);
    assert!(roll.value <= 100);
    assert_eq!(roll.dice.len(), 5);
}

#[test]
fn it_executes_simple_input() {
    let input = "#test !r 1d20min20max40+@me.dexterity".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 5
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], 5);
    assert_eq!(rolls[0].value - rolls[0].raw_value, 5);
}

#[test]
fn it_executes_positive_modifier() {
    let input = "#test $foo = 10 !r 1d20+$foo".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], 10);
    assert_eq!(rolls[0].value - rolls[0].raw_value, 10);
}

#[test]
fn it_executes_negative_modifier() {
    let input = "#test !r 1d20-@me.dexterity".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 5
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], -5);
    assert_eq!(rolls[0].value - rolls[0].raw_value, -5);
}

#[test]
fn it_assigns_and_updates_token_attributes() {
    let input = "#test @me.dexterity = 25".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("dexterity").unwrap();
    assert_eq!(attr, &StepValue::Float(25.00));

    // test assigning a variable
    let input = "#test !roll 1d20 >> @me.dexterity = $1".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("dexterity").unwrap();
    assert_ne!(attr, &StepValue::Number(21));

    // assign booleans
    let input = "#test !r 1d20 >> @me.is_test = true".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "is_test": {
                    "Boolean": false
                },
                "is_not_test": {
                    "Boolean": false
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let is_test = token.attributes.get("is_test").unwrap();
    let is_not_test = token.attributes.get("is_not_test").unwrap();
    assert_eq!(is_test, &StepValue::Boolean(true));
    assert_eq!(is_not_test, &StepValue::Boolean(false));
}

#[test]
fn it_executes_true_false_statements() {
    let input = "#test @me.dexterity > 25 ? !roll 1d8+5 : !r 1d8".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.rolls.len(), 1);
}

#[test]
fn it_execute_say_commands() {
    let input = "#test !say 'Hello!'".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.messages.len(), 1);
    assert_eq!(output.messages[0].message, "Hello!".to_string());

    let input = "#test !say 'Hello from token!' @token1".to_string().into_bytes();
    let token_input = r#"{
        "token1": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.messages.len(), 1);
    assert_eq!(output.messages[0].message, "Hello from token!".to_string());
    assert_eq!(output.messages[0].from, Some("token1".to_string()));
}

#[test]
fn it_execute_whisper_commands() {
    let input = "#test !w @gm 'Hello!'".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.messages.len(), 1);
    assert_eq!(output.messages[0].message, "Hello!".to_string());
    assert_eq!(output.messages[0].to, Some("gm".to_string()));

    let input = "#test $foo = 'From Variable' | $bar = 12 !w @gm 'Hello ' $foo '-' $bar".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.messages.len(), 1);
    assert_eq!(output.messages[0].message, "Hello From Variable-12".to_string());
    assert_eq!(output.messages[0].to, Some("gm".to_string()));
}

#[test]
fn it_executes_roll_comparisons() {
    let input = "#test !r 1d20gt20".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);

    assert_eq!(output.rolls.len(), 1);
    assert_eq!(output.rolls[0].dice[0].is_dropped, true);
}

#[test]
fn it_executes_primitive_operations() {
    // Add
    let input = "#test @me.hp = @me.hp + 5".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "hp": {
                    "Number": 50
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("hp").unwrap();
    assert_eq!(attr, &StepValue::Float(55.00));

    // Divide
    let input = "#test @me.foo = @me.bar / 10".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "foo": {
                    "Number": 100
                },
                "bar": {
                    "Number": 50
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("foo").unwrap();
    assert_eq!(attr, &StepValue::Float(5.00));
    let attr = token.attributes.get("bar").unwrap();
    assert_eq!(attr, &StepValue::Number(50));

    // Subtract
    let input = "#test @me.hp = @me.hp - 5".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "hp": {
                    "Number": 50
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("hp").unwrap();
    assert_eq!(attr, &StepValue::Float(45.00));

    // Multiply
    let input = "#test @me.bar = @me.hp * -2.5".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "hp": {
                    "Number": 100
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("bar").unwrap();
    assert_eq!(attr, &StepValue::Float(-250.00));
}

#[test]
fn it_executes_token_macros() {
    let input = "#test @me->test_macro".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {
                "test_macro": {
                    "Text": "!r 1d20+1 >> !say @me \"rolled a \" $1"
                }
            }
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    let messages = output.messages;
    assert_eq!(rolls.len(), 1);
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].message, "rolled a ".to_owned() + &rolls[0].value.to_string());
}
