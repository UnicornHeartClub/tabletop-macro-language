use die::{Die, DieType};
use parser::{Arg, MacroOp, Step, RollArg};
use roll::*;

pub fn execute_roll (step: &Step) -> Roll {
    // Take the arguments determine what we need to do
    let mut number_of_dice = 0;
    let mut number_of_sides = 0;
    let mut die_type = DieType::Other;
    for arg in &step.args {
        if let &Arg::Roll(RollArg::N(num)) = arg {
            number_of_dice = num;
        } else if let &Arg::Roll(RollArg::D(num)) = arg {
            number_of_sides = num;
            die_type = match num {
                100   => DieType::D100,
                20    => DieType::D20,
                12    => DieType::D12,
                10    => DieType::D10,
                8     => DieType::D8,
                6     => DieType::D6,
                4     => DieType::D4,
                _     => DieType::Other,
            };
        }
    }

    match die_type {
        DieType::D100   => roll_d100(number_of_dice as i8),
        DieType::D20    => roll_d20(number_of_dice as i8),
        DieType::D12    => roll_d12(number_of_dice as i8),
        DieType::D10    => roll_d10(number_of_dice as i8),
        DieType::D10    => roll_d10(number_of_dice as i8),
        DieType::D8     => roll_d8(number_of_dice as i8),
        DieType::D6     => roll_d6(number_of_dice as i8),
        DieType::D4     => roll_d4(number_of_dice as i8),
        _ => {
            // Build the custom sided die
            let mut dice = Vec::new();
            for _ in 0..number_of_dice {
                let mut die = Die::new(die_type);
                die.set_sides(number_of_sides);
                die.set_min(1);
                die.set_max(number_of_sides as i8);
                dice.push(die);
            }
            Roll::new(dice)
        }
    }
}
