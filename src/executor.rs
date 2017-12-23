use die::{Die, DieType};
use message::Message;
use output::Output;
use arg::{
    Arg,
    ArgValue,
    ComparisonArg,
    Conditional,
    MacroOp,
    Primitive,
    RollArg,
    SayArg,
};
use step::{
    Step,
    StepResult,
    StepValue,
};
use parser::{
    error_to_string,
    parse_p,
};
use roll::*;
use serde_json;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use token::Token;

/// Executes macro input and outputs a completed program
pub fn execute_macro(input: Vec<u8>, input_tokens: Vec<u8>) -> Output {
    // Start the timer
    let start = Instant::now();

    // Parse tokens, create a list of tokens we can use (e.g. @me, @selected)
    let input_tokens_str = str::from_utf8(&input_tokens).unwrap();
    let tokens: HashMap<String, Token> = serde_json::from_str(input_tokens_str).unwrap();

    // Parse input into a Program
    let input_clone = input.clone();
    let prog = parse_p(input_clone.as_slice());

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
            match step.op {
                MacroOp::Exit => {
                    break;
                },
                _ => {
                    execute_step(&step, &mut output);
                }
            }
        };

        output.program = Some(program);
        let elapsed = start.elapsed();
        output.execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;
        output
    }
}

pub fn execute_inline_macro(input: String, mut output: &mut Output) {
    let prog = parse_p(input.as_ref());

    if prog.is_err() {
        // Push the error
        let error = prog.unwrap_err();
        output.errors.push(error_to_string(error));
    } else {
        let (_, mut program) = prog.unwrap();

        for step in &mut program.steps {
            execute_step(&step, &mut output);
        };
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
    message.from = output.run_as.clone();
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
                Some(&StepValue::Float(ref value)) => {
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
                    let ref right = assign.right;
                    let mut operator = None;
                    let mut attribute: Option<StepValue> = None;

                    for ref right_arg in right.into_iter() {
                        match get_arg_value(right_arg, &output) {
                            Some(ArgValue::Boolean(boolean)) => {
                                attribute = Some(StepValue::Boolean(boolean));
                            },
                            Some(ArgValue::Float(v)) => {
                                let current_value = v.to_owned();

                                if let Some(op) = operator {
                                    if let Some(StepValue::Float(last_value)) = attribute {
                                        let new_value = match op {
                                            Primitive::Add => last_value + current_value,
                                            Primitive::Subtract => last_value - current_value,
                                            Primitive::Divide => last_value / current_value,
                                            Primitive::Multiply => last_value * current_value,
                                        };
                                        attribute = Some(StepValue::Float(new_value));
                                    } else {
                                        attribute = Some(StepValue::Float(current_value));
                                    }
                                    operator = None;
                                } else {
                                    attribute = Some(StepValue::Float(current_value));
                                }
                            },
                            Some(ArgValue::Number(v)) => {
                                let current_value = v.to_owned();

                                if let Some(op) = operator {
                                    if let Some(StepValue::Float(last_value)) = attribute {
                                        let new_value = match op {
                                            Primitive::Add => (last_value as i32) + current_value,
                                            Primitive::Subtract => (last_value as i32) - current_value,
                                            Primitive::Divide => (last_value as i32) / current_value,
                                            Primitive::Multiply => (last_value as i32) * current_value,
                                        };
                                        attribute = Some(StepValue::Float(new_value as f32));
                                    } else {
                                        attribute = Some(StepValue::Float(current_value as f32));
                                    }
                                    operator = None;
                                } else {
                                    attribute = Some(StepValue::Float(current_value as f32));
                                }
                            },
                            Some(ArgValue::Primitive(primitive)) => {
                                operator = Some(primitive);
                            },
                            Some(ArgValue::Text(text)) => {
                                attribute = Some(StepValue::Text(text.to_owned()));
                            },
                            _ => {
                                // Ignore anything else
                            }
                        }
                    }

                    if let Some(attr) = attribute {
                        output.results.insert(k.to_owned(), attr);
                    }
                },
                ArgValue::Token(ref t) => {
                    // Insert our token if it doesn't exist, new scope because of the borrow checker
                    {
                        let name = t.name.clone();
                        output.tokens.entry(name).or_insert(Token {
                            attributes: HashMap::new(),
                            macros: HashMap::new(),
                        });
                    }

                    let ref right = assign.right;
                    let attr = t.attribute.clone();
                    match attr {
                        Some(a) => {
                            let mut operator = None;
                            let mut attribute: Option<StepValue> = None;

                            for ref right_arg in right.into_iter() {
                                match get_arg_value(right_arg, &output) {
                                    Some(ArgValue::Boolean(boolean)) => {
                                        attribute = Some(StepValue::Boolean(boolean));
                                    },
                                    Some(ArgValue::Float(v)) => {
                                        let current_value = v.to_owned();

                                        if let Some(op) = operator {
                                            if let Some(StepValue::Float(last_value)) = attribute {
                                                let new_value = match op {
                                                    Primitive::Add => last_value + current_value,
                                                    Primitive::Subtract => last_value - current_value,
                                                    Primitive::Divide => last_value / current_value,
                                                    Primitive::Multiply => last_value * current_value,
                                                };
                                                attribute = Some(StepValue::Float(new_value));
                                            } else {
                                                attribute = Some(StepValue::Float(current_value));
                                            }
                                            operator = None;
                                        } else {
                                            attribute = Some(StepValue::Float(current_value));
                                        }
                                    },
                                    Some(ArgValue::Number(v)) => {
                                        let current_value = v.to_owned();

                                        if let Some(op) = operator {
                                            if let Some(StepValue::Float(last_value)) = attribute {
                                                let new_value = match op {
                                                    Primitive::Add => (last_value as i32) + current_value,
                                                    Primitive::Subtract => (last_value as i32) - current_value,
                                                    Primitive::Divide => (last_value as i32) / current_value,
                                                    Primitive::Multiply => (last_value as i32) * current_value,
                                                };
                                                attribute = Some(StepValue::Float(new_value as f32));
                                            } else {
                                                attribute = Some(StepValue::Float(current_value as f32));
                                            }
                                            operator = None;
                                        } else {
                                            attribute = Some(StepValue::Float(current_value as f32));
                                        }
                                    },
                                    Some(ArgValue::Primitive(primitive)) => {
                                        operator = Some(primitive);
                                    },
                                    Some(ArgValue::Text(text)) => {
                                        attribute = Some(StepValue::Text(text.to_owned()));
                                    },
                                    _ => {
                                        // we should not get a return value from anything else
                                        // throw error here?
                                    }
                                }
                            }

                            if let Some(token) = output.tokens.get_mut(&t.name) {
                                if let Some(attr) = attribute {
                                    token.attributes.insert(a.to_owned(), attr);
                                }
                            }
                        },
                        None => {}
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

            // Get the values from the left and right side
            if let Some(left_value) = get_arg_value(left, &output) {
                if let Some(right_value) = get_arg_value(right, &output) {
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
            }
        } else if let &Arg::Token(ref token) = arg {
            // Let's run a token macro
            let token_macro_name = token.macro_name.clone();
            let mut token_macro = None;
            match token_macro_name {
                Some(inline_macro_name) => {
                    let token_result = output.tokens.get(&token.name).unwrap();

                    // Lookup the macro in the attributes
                    if let Some(&StepValue::Text(ref macro_text)) = token_result.macros.get(&inline_macro_name) {
                        // combine the name and the macro
                        let space = " ";
                        token_macro = Some("#".to_owned() + &inline_macro_name + space + macro_text);
                        output.run_as = Some(token.name.clone());
                    }
                },
                None => {}
            }

            if let Some(inline_macro) = token_macro {
                // execute this macro by parsing the output
                execute_inline_macro(inline_macro, output);
            }
        }
    };
}

pub fn execute_roll (step: &Step, output: &mut Output) -> Roll {
    // Compose the roll
    let mut composed_roll = ComposedRoll {
        advantage: false,
        comment: None,
        d: 0,
        die: DieType::Other,
        disadvantage: false,
        e: 0,
        gt: 0,
        gte: 0,
        h: 0,
        l: 0,
        lt: 0,
        lte: 0,
        max: 0,
        min: 1,
        modifiers: vec![],
        n: 0,
        ro: 0,
        rr: 0,
    };


    // build the calculated equation to output with our roll
    let mut equation = "".to_owned();
    let mut token = output.run_as.clone();

    for arg in &step.args {
        if let &Arg::Token(ref t) = arg {
            token = Some(t.name.clone());
        } else if let &Arg::Roll(RollArg::N(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.n = n as i16;
                    equation = equation + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::D(ref value)) = arg {
            match get_arg_value(value, &output) {
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

                    equation = equation + &"d" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::H(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.h = n as i16;
                    equation = equation + &"kh" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::L(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.l = n as i16;
                    equation = equation + &"kl" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::GT(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.gt = n as u16;
                    equation = equation + &"gt" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::GTE(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.gte = n as u16;
                    equation = equation + &"gte" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::LT(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.lt = n as u16;
                    equation = equation + &"lt" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::LTE(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.lte = n as u16;
                    equation = equation + &"lte" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RR(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.rr = n as i16;
                    equation = equation + &"rr" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Number(n)) => {
                    composed_roll.ro = n as i16;
                    equation = equation + &"ro" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierPos(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Float(n)) => {
                    composed_roll.modifiers.push(n as i16);
                    equation = equation + &"+" + &n.to_string();
                },
                Some(ArgValue::Number(n)) => {
                    composed_roll.modifiers.push(n as i16);
                    equation = equation + &"+" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::ModifierNeg(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Float(n)) => {
                    composed_roll.modifiers.push(n as i16);
                    equation = equation + &"-" + &n.to_string();
                },
                Some(ArgValue::Number(n)) => {
                    composed_roll.modifiers.push(n as i16 * -1);
                    equation = equation + &"-" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Max(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Float(n)) => {
                    composed_roll.max = n as i16;
                    equation = equation + &"max" + &n.to_string();
                },
                Some(ArgValue::Number(n)) => {
                    composed_roll.max = n as i16;
                    equation = equation + &"max" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Min(ref value)) = arg {
            match get_arg_value(value, &output) {
                Some(ArgValue::Float(n)) => {
                    composed_roll.min = n as i16;
                    equation = equation + &"min" + &n.to_string();
                },
                Some(ArgValue::Number(n)) => {
                    composed_roll.min = n as i16;
                    equation = equation + &"min" + &n.to_string();
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::Comment(ArgValue::Text(ref n))) = arg {
            composed_roll.comment = Some(n.to_owned());
            equation = equation + &" '" + &n + &"'";
        }
    }

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
    roll.add_equation(equation);

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

    if composed_roll.gt != 0 {
        roll.keep_greater_than(composed_roll.gt);
    } else if composed_roll.gte != 0 {
        roll.keep_greater_than_or_equal_to(composed_roll.gte);
    } else if composed_roll.lt != 0 {
        roll.keep_less_than(composed_roll.lt);
    } else if composed_roll.lte != 0 {
        roll.keep_less_than_or_equal_to(composed_roll.lte);
    } else if composed_roll.h != 0 {
        roll.keep_high(composed_roll.h as u16);
    } else if composed_roll.l != 0 {
        roll.keep_low(composed_roll.l as u16);
    }

    // Add a comment
    match composed_roll.comment {
        Some(c) => roll.add_comment(c),
        None => {}
    }

    // Associate a token
    match token {
        Some(t) => roll.add_token(t),
        None => {}
    }

    roll
}

/// Gets the value of the argvalue, whether from a variable, token, etc.
pub fn get_arg_value (value: &ArgValue, output: &Output) -> Option<ArgValue> {
    let ref results = output.results;
    let ref tokens = output.tokens;

    match value {
        &ArgValue::Boolean(ref n) => {
            Some(ArgValue::Boolean(n.clone()))
        },
        &ArgValue::Number(ref n) => {
            Some(ArgValue::Number(n.clone()))
        },
        &ArgValue::Float(ref n) => {
            Some(ArgValue::Float(n.clone()))
        },
        &ArgValue::Text(ref n) => {
            Some(ArgValue::Text(n.clone()))
        },
        &ArgValue::Token(ref token) => {
            let token_result = tokens.get(&token.name).unwrap();
            let token_attr = token.attribute.clone();
            match token_attr {
                Some(a) => {
                    let attr = token_result.attributes.get(&a);
                    match attr {
                        Some(&StepValue::Boolean(n)) => {
                            Some(ArgValue::Boolean(n.clone()))
                        },
                        Some(&StepValue::Number(n)) => {
                            Some(ArgValue::Number(n.clone()))
                        },
                        Some(&StepValue::Float(n)) => {
                            Some(ArgValue::Float(n.clone()))
                        },
                        Some(&StepValue::Text(ref n)) => {
                            Some(ArgValue::Text(n.clone()))
                        },
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
        &ArgValue::Primitive(ref n) => {
            // there has to be a better way to do this ....
            let prim = match n {
                &Primitive::Add => ArgValue::Primitive(Primitive::Add),
                &Primitive::Subtract => ArgValue::Primitive(Primitive::Subtract),
                &Primitive::Divide => ArgValue::Primitive(Primitive::Divide),
                &Primitive::Multiply => ArgValue::Primitive(Primitive::Multiply),
            };
            Some(prim)
        },
        &ArgValue::Variable(ref var) => {
            match results.get(&var.to_string()) {
                Some(&StepValue::Boolean(n)) => {
                    Some(ArgValue::Boolean(n.clone()))
                },
                Some(&StepValue::Number(n)) => {
                    Some(ArgValue::Number(n))
                },
                Some(&StepValue::Float(n)) => {
                    Some(ArgValue::Float(n.clone()))
                },
                Some(&StepValue::Text(ref n)) => {
                    Some(ArgValue::Text(n.clone()))
                },
                None => {
                    None
                }
            }
        },
        &ArgValue::VariableReserved(ref var) => {
            match results.get(&var.to_string()) {
                Some(&StepValue::Boolean(n)) => {
                    Some(ArgValue::Boolean(n.clone()))
                },
                Some(&StepValue::Float(n)) => {
                    Some(ArgValue::Float(n.clone()))
                },
                Some(&StepValue::Number(n)) => {
                    Some(ArgValue::Number(n.clone()))
                },
                Some(&StepValue::Text(ref n)) => {
                    Some(ArgValue::Text(n.clone()))
                },
                None => {
                    None
                }
            }
        },
    }
}
