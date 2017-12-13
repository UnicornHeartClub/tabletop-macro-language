use die::{Die, DieType};
use message::Message;
use output::Output;
use parser::{
    Arg,
    ArgValue,
    ComparisonArg,
    Conditional,
    MacroOp,
    RollArg,
    SayArg,
    Step,
    StepResult,
    StepValue,
    TokenArg,
    error_to_string,
    parse_p,
};
use roll::*;
use serde_json;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use token::{Token, TokenAttributeValue};

/// Executes macro input and outputs a completed program
pub fn execute_macro(input: Vec<u8>, input_tokens: Vec<u8>) -> Output {
    // Start the timer
    let start = Instant::now();

    // Parse input
    let input_clone = input.clone();
    let prog = parse_p(input_clone.as_slice());

    // Parse tokens, create a list of tokens we can use (e.g. @me, @selected)
    let input_tokens_str = str::from_utf8(&input_tokens).unwrap();
    let tokens: HashMap<String, Token> = serde_json::from_str(input_tokens_str).unwrap();

    // Start the output
    let mut output = Output::new(String::from_utf8(input).unwrap());
    output.tokens = tokens;

    if prog.is_err() {
        // Push the error
        let error = prog.unwrap_err();
        output.errors.push(error_to_string(error));

        let elapsed = start.elapsed();
        output.execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;
        output
    } else {
        let (_, mut program) = prog.unwrap();

        for step in &mut program.steps {
            execute_step(&step, &mut output);
        };

        output.program = Some(program);
        let elapsed = start.elapsed();
        output.execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;
        output
    }
}

pub fn execute_step (step: &Step, mut output: &mut Output) {
    match step.op {
        MacroOp::Lambda => {
            execute_step_lambda(&step, &mut output);
        },
        MacroOp::Say |
        MacroOp::Whisper => {
            execute_step_say(&step, &mut output);
        },
        MacroOp::Roll => {
            // execute the roll and update the step value
            let roll = execute_roll(&step, output);

            // pass the result if needed
            if step.result == StepResult::Save {
                let index = output.results.len() + 1;
                output.results.insert(index.to_string(), StepValue::Number(roll.value));
            }

            // push to the tracked rolls
            output.rolls.push(roll);
        },
        _ => println!("Not yet implemented {:?}", step.op)
    }
}

pub fn execute_step_say(step: &Step, output: &mut Output) {
    let mut message = Message::new("".to_string());
    for arg in &step.args {
        if let &Arg::Say(SayArg::Message(ref value)) = arg {
            // Concat the message string
            let mut original_message = message.message.clone();
            original_message.push_str(value);
            message.message = original_message;
        } else if let &Arg::Variable(ref var) = arg {
            match output.results.get(&var.to_string()) {
                Some(&StepValue::Text(ref value)) => {
                    let mut original_message = message.message.clone();
                    original_message.push_str(value);
                    message.message = original_message;
                },
                Some(&StepValue::Number(ref value)) => {
                    let mut original_message = message.message.clone();
                    original_message.push_str(&value.to_string());
                    message.message = original_message;
                },
                _ => {}
            };
        } else if let &Arg::Say(SayArg::To(ref token)) = arg {
            message.to = Some(token.name.clone());
        } else if let &Arg::Say(SayArg::From(ref token)) = arg {
            message.from = Some(token.name.clone());
        }
    }
    output.messages.push(message);
}

pub fn execute_step_lambda(step: &Step, output: &mut Output) {
    for arg in &step.args {
        if let &Arg::Assign(ref assign) = arg {
            match assign.left {
                ArgValue::Variable(ref k) => {
                    match assign.right {
                        ArgValue::Number(ref v) => {
                            output.results.insert(k.to_owned(), StepValue::Number(v.to_owned()));
                        },
                        ArgValue::Text(ref v) => {
                            output.results.insert(k.to_owned(), StepValue::Text(v.to_owned()));
                        },
                        _ => {}
                    }
                },
                ArgValue::Token(ref t) => {
                    let attr = t.attribute.clone();
                    let name = t.name.clone();
                    let token = output.tokens.entry(name).or_insert(Token {
                        attributes: HashMap::new(),
                        macros: HashMap::new(),
                    });
                    match attr {
                        Some(a) => {
                            match assign.right {
                                ArgValue::Number(ref v) => {
                                    &token.attributes.insert(a, TokenAttributeValue::Number(v.to_owned()));
                                },
                                ArgValue::Text(ref v) => {
                                    &token.attributes.insert(a, TokenAttributeValue::Text(v.to_owned()));
                                },
                                ArgValue::VariableReserved(ref v) => {
                                    // Lookup the variable in the index
                                    match output.results.get(&v.to_string()) {
                                        Some(&StepValue::Number(ref n)) => {
                                            &token.attributes.insert(a, TokenAttributeValue::Number(n.to_owned()));
                                        },
                                        Some(&StepValue::Text(ref n)) => {
                                            &token.attributes.insert(a, TokenAttributeValue::Text(n.to_owned()));
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        } else if let &Arg::Conditional(ref conditional) = arg {
            let &Conditional {
                ref left,
                ref right,
                ref comparison,
                ref success,
                ref failure,
            } = conditional;

            // Get the value from the left side, we only allow numbers right now
            let left_value = match get_arg_value(left, &output.results, &output.tokens) {
                Some(ArgValue::Number(l)) => { l },
                _ => { 0 }
            };

            let right_value = match get_arg_value(right, &output.results, &output.tokens) {
                Some(ArgValue::Number(l)) => { l },
                _ => { 0 }
            };

            // compare the left and right values
            let is_success = match comparison {
                &ComparisonArg::EqualTo => {
                    left_value == right_value
                },
                &ComparisonArg::GreaterThan => {
                    left_value > right_value
                },
                &ComparisonArg::GreaterThanOrEqual => {
                    left_value >= right_value
                },
                &ComparisonArg::LessThan => {
                    left_value < right_value
                },
                &ComparisonArg::LessThanOrEqual => {
                    left_value <= right_value
                }
            };

            if is_success {
                match success {
                    &Some(ref step) => {
                        execute_step(&step, output);
                    },
                    &None => {}
                }
            } else {
                match failure {
                    &Some(ref step) => {
                        execute_step(&step, output);
                    },
                    &None => {}
                }
            }
        }
    };
}

pub fn execute_roll (step: &Step, output: &mut Output) -> Roll {
    // Compose the roll
    let mut composed_roll = ComposedRoll {
        advantage: false,
        comment: None,
        die: DieType::Other,
        disadvantage: false,
        e: 0,
        h: 0,
        d: 0,
        l: 0,
        max: 0,
        min: 1,
        modifiers: vec![],
        n: 0,
        ro: 0,
        rr: 0,
    };

    for arg in &step.args {
        if let &Arg::Roll(RollArg::N(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.n = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::D(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.d = n as i16;
                    composed_roll.max = n as i16;
                    composed_roll.die = match n {
                        100   => DieType::D100,
                        20    => DieType::D20,
                        12    => DieType::D12,
                        10    => DieType::D10,
                        8     => DieType::D8,
                        6     => DieType::D6,
                        4     => DieType::D4,
                        _     => DieType::Other,
                    };
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::H(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.h = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::L(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.l = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RR(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.rr = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.ro = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.ro = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierPos(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.modifiers.push(n as i16);
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierNeg(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.modifiers.push(n as i16 * -1);
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Max(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.max = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Min(ref value)) = arg {
            match get_arg_value(value, &output.results, &output.tokens) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.min = n as i16;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Comment(ArgValue::Text(ref n))) = arg {
            composed_roll.comment = Some(n.to_owned());
        }
    }

    // @todo build custom dice always

    // Build the custom sided die
    let mut dice = Vec::new();
    for _ in 0..composed_roll.n {
        let mut die = Die::new(composed_roll.die);
        die.set_sides(composed_roll.d as u8);
        die.set_min(composed_roll.min);
        die.set_max(composed_roll.max);
        dice.push(die);
    }
    let mut roll = Roll::new(dice);

    if composed_roll.modifiers.len() > 0 {
        for i in composed_roll.modifiers.into_iter() {
            roll.apply_modifier(i);
        }
    }

    if composed_roll.e > 0 {
        // todo
    } else if composed_roll.e < 0 {
        // todo
    } else if composed_roll.rr > 0 {
        roll.reroll_dice_forever_above(composed_roll.rr);
    } else if composed_roll.rr < 0 {
        roll.reroll_dice_forever_below(composed_roll.rr);
    } else if composed_roll.ro > 0 {
        roll.reroll_dice_once_above(composed_roll.ro);
    } else if composed_roll.ro < 0 {
        roll.reroll_dice_once_below(composed_roll.ro);
    }

    if composed_roll.h != 0 {
        roll.keep_high(composed_roll.h as u16);
    } else if composed_roll.l != 0 {
        roll.keep_low(composed_roll.l as u16);
    }

    match composed_roll.comment {
        Some(c) => roll.add_comment(c),
        None => {}
    }

    roll
}

/// Gets the value of the argvalue, whether from a variable, token, etc.
pub fn get_arg_value (value: &ArgValue, results: &HashMap<String, StepValue>, tokens: &HashMap<String, Token>) -> Option<ArgValue> {
    match value {
        &ArgValue::Number(ref n) => {
            Some(ArgValue::Number(n.clone()))
        },
        &ArgValue::Text(ref n) => {
            None
        },
        &ArgValue::Token(ref token) => {
            let token_result = tokens.get(&token.name);
            let token_attr = token.attribute.clone();
            match token_result {
                Some(t) => {
                    match token_attr {
                        Some(a) => {
                            let attr = t.attributes.get(&a);
                            match attr {
                                Some(&TokenAttributeValue::Number(n)) => {
                                    Some(ArgValue::Number(n.clone()))
                                }
                                _ => {
                                    None
                                }
                            }
                        }
                        _ => {
                            None
                        }
                    }
                },
                None => {
                    None
                }
            }
        },
        &ArgValue::Variable(ref var) => {
            match results.get(&var.to_string()) {
                Some(&StepValue::Number(n)) => {
                    Some(ArgValue::Number(n))
                },
                _ => {
                    None
                }
            }
        },
        &ArgValue::VariableReserved(ref var) => {
            match results.get(&var.to_string()) {
                Some(&StepValue::Number(n)) => {
                    Some(ArgValue::Number(n.clone()))
                },
                Some(&StepValue::Text(ref n)) => {
                    None
                },
                None => {
                    None
                }
            }
        },
    }
}

