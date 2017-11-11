use chrono::prelude::Utc;
use die::{Die, DieType};
use std::time::Instant;
use parser::{Arg, ArgValue, MacroOp, Step, StepResult, StepValue, RollArg, error_to_string, parse_p};
use output::Output;
use roll::*;

// Rolls all the arguments into a single struct
struct ComposedRoll {
    Advantage: bool,
    Disadvantage: bool,
    E: i16,
    H: i16,
    D: i16,
    L: i16,
    N: i16,
    RO: i16,
    RR: i16,
    Type: DieType,
}

pub fn execute_macro(input: Vec<u8>) -> Output {
    // Start the timer
    let start = Instant::now();
    let executed = Utc::now();

    let mut errors = Vec::new();
    let mut messages = Vec::new();
    let mut results = Vec::new();
    let mut rolls = Vec::new();
    let mut tokens = Vec::new();
    let version = String::from(env!("CARGO_PKG_VERSION"));

    // Parse the input
    let input_clone = input.clone();
    let prog = parse_p(input_clone.as_slice());

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
                MacroOp::Roll => {
                    // @todo check if we have a variable to replace

                    // execute the roll and update the step value
                    let roll = execute_roll(&step, &results);
                    step.value = Some(StepValue::Number(roll.value));

                    // pass the result if needed
                    if step.result == StepResult::Pass {
                        results.push(StepValue::Number(roll.value));
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

pub fn execute_roll (step: &Step, results: &Vec<StepValue>) -> Roll {
    // Compose the roll
    let mut composed_roll = ComposedRoll {
        Advantage: false,
        Disadvantage: false,
        E: 0,
        H: 0,
        D: 0,
        L: 0,
        N: 0,
        RO: 0,
        RR: 0,
        Type: DieType::Other,
    };

    // @todo I am not a huge fan of how this looks, there must be an easier way ...
    for arg in &step.args {
        if let &Arg::Roll(RollArg::N(ArgValue::Number(n))) = arg {
            composed_roll.N = n;
        } else if let &Arg::Roll(RollArg::N(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.N = n;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::D(ArgValue::Number(n))) = arg {
            composed_roll.D = n;
            composed_roll.Type = match n {
                100   => DieType::D100,
                20    => DieType::D20,
                12    => DieType::D12,
                10    => DieType::D10,
                8     => DieType::D8,
                6     => DieType::D6,
                4     => DieType::D4,
                _     => DieType::Other,
            };
        } else if let &Arg::Roll(RollArg::D(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.D = n;
                    composed_roll.Type = match n {
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
        } else if let &Arg::Roll(RollArg::H(ArgValue::Number(n))) = arg {
            composed_roll.H = n;
        } else if let &Arg::Roll(RollArg::H(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.H = n;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::L(ArgValue::Number(n))) = arg {
            composed_roll.L = n;
        } else if let &Arg::Roll(RollArg::L(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.L = n;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RR(ArgValue::Number(n))) = arg {
            composed_roll.RR = n;
        } else if let &Arg::Roll(RollArg::RR(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.RR = n;
                },
                _ => {}
            }
        } else if let &Arg::Roll(RollArg::RO(ArgValue::Number(n))) = arg {
            composed_roll.RO = n;
        } else if let &Arg::Roll(RollArg::RO(ArgValue::VariableReserved(n))) = arg {
            // Lookup the variable in the index
            match results.get(n as usize - 1) {
                Some(&StepValue::Number(n)) => {
                    composed_roll.RO = n;
                },
                _ => {}
            }
        }
    }

    let mut roll = match composed_roll.Type {
        DieType::D100   => roll_d100(composed_roll.N as u16),
        DieType::D20    => roll_d20(composed_roll.N as u16),
        DieType::D12    => roll_d12(composed_roll.N as u16),
        DieType::D10    => roll_d10(composed_roll.N as u16),
        DieType::D10    => roll_d10(composed_roll.N as u16),
        DieType::D8     => roll_d8(composed_roll.N as u16),
        DieType::D6     => roll_d6(composed_roll.N as u16),
        DieType::D4     => roll_d4(composed_roll.N as u16),
        _ => {
            // Build the custom sided die
            let mut dice = Vec::new();
            for _ in 0..composed_roll.N {
                let mut die = Die::new(composed_roll.Type);
                die.set_sides(composed_roll.D as u8);
                die.set_min(1);
                die.set_max(composed_roll.D as i16);
                dice.push(die);
            }
            Roll::new(dice)
        }
    };

    if composed_roll.E > 0 {
        // todo
    } else if composed_roll.E < 0 {
        // todo
    } else if composed_roll.RR > 0 {
        roll.reroll_dice_forever_above(composed_roll.RR);
    } else if composed_roll.RR < 0 {
        roll.reroll_dice_forever_below(composed_roll.RR);
    } else if composed_roll.RO > 0 {
        roll.reroll_dice_once_above(composed_roll.RO);
    } else if composed_roll.RO < 0 {
        roll.reroll_dice_once_below(composed_roll.RO);
    }

    if composed_roll.H != 0 {
        roll.keep_high(composed_roll.H as u16);
    } else if composed_roll.L != 0 {
        roll.keep_low(composed_roll.L as u16);
    }

    roll
}
