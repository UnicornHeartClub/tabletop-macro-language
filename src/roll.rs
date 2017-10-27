use die::Die;
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
    pub raw_value: i8,

    /// The associated token (optional)
    pub token: Option<String>,

    /// The final combined value of the die after modifiers
    pub value: i8,
}

impl Roll {
    pub fn new(mut dice: Vec<Die>) -> Roll {
        // Roll each dice
        for die in &mut dice {
            die.roll();
        }

        let value = dice.iter().fold(0, |sum, d| sum + d.value);

        Roll {
            _id: Uuid::new_v4().to_string(),
            dice,
            modifiers: Vec::new(),
            raw_value: value,
            token: None,
            value,
        }
    }

    pub fn add_token(&mut self, token: String) {
        self.token = Some(token)
    }
}
