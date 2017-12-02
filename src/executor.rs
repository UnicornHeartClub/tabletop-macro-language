use chrono::prelude::Utc;
use die::{Die, DieType};
use output::Output;
use parser::{Arg, ArgValue, MacroOp, Step, StepResult, StepValue, RollArg, error_to_string, parse_p};
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
    let executed = Utc::now();

    let mut errors = Vec::new(); // error messages, points to the col/row that error happened
    let mut messages = Vec::new(); // messages to send to chat
    let mut results: HashMap<String, StepValue> = HashMap::new(); // a list of variables that we can use (e.g. $1, $2)
    let mut rolls = Vec::new(); // rolls
    let version = String::from(env!("CARGO_PKG_VERSION"));

    // Parse the input
    let input_clone = input.clone();
    let prog = parse_p(input_clone.as_slice());

    // Parse tokens
    let input_tokens_str = str::from_utf8(&input_tokens).unwrap();
    let mut tokens: HashMap<String, Token> = serde_json::from_str(input_tokens_str).unwrap();

    if prog.is_err() {
        // Push the error
        let error = prog.unwrap_err();
        errors.push(error_to_string(error));

        let elapsed = start.elapsed();
        let execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;

        Output {
            input: String::from_utf8(input).unwrap(),
            executed,
            execution_time,
            errors,
            messages,
            program: None,
            rolls,
            tokens,
            version,
        }
    } else {
        let (_, mut program) = prog.unwrap();

        for step in &mut program.steps {
            match step.op {
                MacroOp::Lambda => execute_step_lambda(&step, &mut results, &mut tokens),
                MacroOp::Roll => {
                    // execute the roll and update the step value
                    let roll = execute_roll(&step, &results, &tokens);
                    step.value = Some(StepValue::Number(roll.value));

                    // pass the result if needed
                    if step.result == StepResult::Save {
                        let index = results.len() + 1;
                        results.insert(index.to_string(), StepValue::Number(roll.value));
                    }

                    // push to the tracked rolls
                    rolls.push(roll);
                },
                _ => println!("Not yet implemented {:?}", step.op)
            }
        };

        let elapsed = start.elapsed();
        let execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;

        Output {
            input: String::from_utf8(input).unwrap(),
            executed,
            execution_time,
            errors,
            messages,
            program: Some(program),
            rolls,
            tokens,
            version,
        }
    }
}

pub fn execute_step_lambda(step: &Step, results: &mut HashMap<String, StepValue>, tokens: &mut HashMap<String, Token>) {
    for arg in &step.args {
        if let &Arg::Assign(ref assign) = arg {
            match assign.left {
                ArgValue::Variable(ref k) => {
                    match assign.right {
                        ArgValue::Number(ref v) => {
                            results.insert(k.to_owned(), StepValue::Number(v.to_owned()));
                        },
                        ArgValue::Text(ref v) => {
                            results.insert(k.to_owned(), StepValue::Text(v.to_owned()));
                        },
                        _ => {}
                    }
                },
                ArgValue::Token(ref t) => {
                    let attr = t.attribute.clone();
                    let name = t.name.clone();
                    let mut token = tokens.entry(name).or_insert(Token {
                        attributes: HashMap::new(),
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
                                    match results.get(&v.to_string()) {
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
        }
    };
}

pub fn execute_roll (step: &Step, results: &HashMap<String, StepValue>, tokens: &HashMap<String, Token>) -> Roll {
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
        min: 0,
        modifiers: vec![],
        n: 0,
        ro: 0,
        rr: 0,
    };

    for arg in &step.args {
        if let &Arg::Roll(RollArg::N(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.n = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::D(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.d = n;
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
                None => {}
            }
        } else if let &Arg::Roll(RollArg::H(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.h = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::L(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.l = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::RR(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.rr = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.ro = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.ro = n;
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierPos(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.modifiers.push(n);
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierNeg(ref value)) = arg {
            match get_roll_value(value, results, tokens) {
                Some(n) => {
                    composed_roll.modifiers.push(n * -1);
                },
                None => {}
            }
        } else if let &Arg::Roll(RollArg::Max(ref value)) = arg {
        } else if let &Arg::Roll(RollArg::Min(ref value)) = arg {
        } else if let &Arg::Roll(RollArg::Comment(ArgValue::Text(ref n))) = arg {
            composed_roll.comment = Some(n.to_owned());
        }
    }

    let mut roll = match composed_roll.die {
        DieType::D100   => roll_d100(composed_roll.n as u16),
        DieType::D20    => roll_d20(composed_roll.n as u16),
        DieType::D12    => roll_d12(composed_roll.n as u16),
        DieType::D10    => roll_d10(composed_roll.n as u16),
        DieType::D8     => roll_d8(composed_roll.n as u16),
        DieType::D6     => roll_d6(composed_roll.n as u16),
        DieType::D4     => roll_d4(composed_roll.n as u16),
        _ => {
            // Build the custom sided die
            let mut dice = Vec::new();
            for _ in 0..composed_roll.n {
                let mut die = Die::new(composed_roll.die);
                die.set_sides(composed_roll.d as u8);
                die.set_min(1);
                die.set_max(composed_roll.d as i16);
                dice.push(die);
            }
            Roll::new(dice)
        }
    };

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

/// Gets the value of the flag, whether from a variable, token, etc.
pub fn get_roll_value (value: &ArgValue, results: &HashMap<String, StepValue>, tokens: &HashMap<String, Token>) -> Option<i16> {
    match value {
        &ArgValue::Number(ref n) => {
            Some(n.clone() as i16)
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
                                    Some(n.clone() as i16)
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
                    Some(n as i16)
                },
                _ => {
                    None
                }
            }
        },
        &ArgValue::VariableReserved(ref var) => {
            match results.get(&var.to_string()) {
                Some(&StepValue::Number(n)) => {
                    Some(n.clone() as i16)
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

