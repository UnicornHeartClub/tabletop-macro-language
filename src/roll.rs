use die::Die;
use die::DieType;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Roll {
    /// Unique identifier for the roll
    pub _id: String,

    /// The dice that compose this roll
    pub dice: Vec<Die>,

    /// Modifiers to apply to the combined value
    pub modifiers: Vec<i8>,

    /// The combined value of the die before modifiers
    pub raw_value: i16,

    /// The associated token (optional)
    pub token: Option<String>,

    /// The final combined value of the die after modifiers
    pub value: i16,
}

impl Roll {
    pub fn new(mut dice: Vec<Die>) -> Roll {
        // Roll each dice
        for die in &mut dice {
            die.roll();
        }

        let value = dice.iter().fold(0, |sum, d| sum + d.value as i16);

        Roll {
            _id: Uuid::new_v4().to_string(),
            dice,
            modifiers: Vec::new(),
            raw_value: value,
            token: None,
            value,
        }
    }

    /// Associate this roll with a token
    pub fn add_token(&mut self, token: String) {
        self.token = Some(token)
    }

    /// Keep the highest rolled dice
    pub fn keep_high(&mut self, keep: u8) {
        // Sort the dice by value, drop everything below the keep value
        let mut count = 0;
        self.dice.sort_by(|a, b| b.value.cmp(&a.value));
        for die in &mut self.dice {
            if count >= keep {
                die.drop();
                self.value -= die.value as i16;
            }
            count += 1;
        }
        // sort by timestamp again before finishing the method
        self.dice.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Keep the lowest rolled dice
    pub fn keep_low(&mut self, keep: u8) {
        // Sort the dice by value, drop everything below the keep value
        let mut count = 0;
        self.dice.sort_by(|a, b| a.value.cmp(&b.value));
        for die in &mut self.dice {
            if count >= keep {
                die.drop();
                self.value -= die.value as i16;
            }
            count += 1;
        }
        // sort by timestamp again before finishing the method
        self.dice.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    pub fn reroll_dice_once_below(&mut self, threshold: u8) {
    }

    pub fn reroll_dice_forever_below(&mut self, threshold: u8) {
    }
}

pub fn roll_and_keep_high(count: i8, die: DieType, keep: u8) -> Roll {
    let mut roll = roll_type(count, die);
    roll.keep_high(keep);
    roll
}

pub fn roll_and_keep_low(count: i8, die: DieType, keep: u8) -> Roll {
    let mut roll = roll_type(count, die);
    roll.keep_low(keep);
    roll
}

pub fn roll_d4(count: i8) -> Roll {
    roll_type(count, DieType::D4)
}

pub fn roll_d6(count: i8) -> Roll {
    roll_type(count, DieType::D6)
}

pub fn roll_d8(count: i8) -> Roll {
    roll_type(count, DieType::D8)
}

pub fn roll_d10(count: i8) -> Roll {
    roll_type(count, DieType::D10)
}

pub fn roll_d12(count: i8) -> Roll {
    roll_type(count, DieType::D12)
}

pub fn roll_d20(count: i8) -> Roll {
    roll_type(count, DieType::D20)
}

fn roll_type(count: i8, die: DieType) -> Roll {
    let mut dice = Vec::new();
    for _ in 0..count {
        dice.push(Die::new(die));
    }
    Roll::new(dice)
}

pub fn roll_with_advantage() -> Roll {
    let mut roll = roll_d20(2);
    roll.keep_high(1);
    roll
}

pub fn roll_with_disadvantage() -> Roll {
    let mut roll = roll_d20(2);
    roll.keep_low(1);
    roll
}
