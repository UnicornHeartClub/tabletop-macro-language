use chrono::DateTime;
use chrono::prelude::Utc;
use rand::distributions::{IndependentSample, Range};
use rand;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DieType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
    Fate,
    Other,
}

/// Determine the minimum number to roll based on the die type
fn get_die_min(die: &DieType) -> i16 {
    match die {
        &DieType::D4 => 1,
        &DieType::D6 => 1,
        &DieType::D8 => 1,
        &DieType::D10 => 1,
        &DieType::D12 => 1,
        &DieType::D20 => 1,
        &DieType::D100 => 1,
        &DieType::Fate => -1,
        &DieType::Other => 0,
    }
}

/// Determine the minimum number to roll based on the die type
fn get_die_max(die: &DieType) -> i16 {
    match die {
        &DieType::D4 => 4,
        &DieType::D6 => 6,
        &DieType::D8 => 8,
        &DieType::D10 => 10,
        &DieType::D12 => 12,
        &DieType::D20 => 20,
        &DieType::D100 => 100,
        &DieType::Fate => 1,
        &DieType::Other => 0,
    }
}

/// Determine the number of sides based on the die type
fn get_die_sides(die: &DieType) -> u8 {
    match die {
        &DieType::D4 => 4,
        &DieType::D6 => 6,
        &DieType::D8 => 8,
        &DieType::D10 => 10,
        &DieType::D12 => 12,
        &DieType::D20 => 20,
        &DieType::D100 => 100,
        &DieType::Fate => 3,
        &DieType::Other => 0,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Die {
    /// Unique identifier of the die
    pub _id: String,

    /// If the die was re-rolled, it will have a child
    pub child: Option<String>,

    /// The type of die (e.g. d20, d100)
    pub die: DieType,

    /// If the die is dropped in the final roll
    pub is_dropped: bool,

    /// If the die is dropped in the final roll
    pub is_rerolled: bool,

    /// If the die is successful when we have a comparison
    pub is_successful: bool,

    /// Maximum number to roll
    pub max: i16,

    /// Minimum number to roll
    pub min: i16,

    /// The number of faces the die has
    pub sides: u8,

    /// Timestamp of the roll
    pub timestamp: DateTime<Utc>,

    /// The determined value of the dice roll
    pub value: i16,
}

impl Die {
    pub fn new (die: DieType) -> Die {
        Die {
            _id: Uuid::new_v4().to_string(),
            child: None,
            die,
            is_dropped: false,
            is_rerolled: false,
            is_successful: false,
            max: get_die_max(&die),
            min: get_die_min(&die),
            sides: get_die_sides(&die),
            timestamp: Utc::now(),
            value: 0,
        }
    }

    /// Drop the die from the final roll
    pub fn drop(&mut self) {
        self.is_dropped = true
    }

    /// Mark the die as successful to the a comparison
    pub fn success(&mut self) {
        self.is_successful = true
    }

    pub fn rerolled (&mut self, die: &Die) {
        self.is_rerolled = true;
        let id = &die._id;
        self.child = Some(id.to_owned());
    }

    /// Roll the die, generating a random number and calculating any modifiers
    pub fn roll(&mut self) -> &Die {
        // generate a random number
        let between = Range::new(self.min, self.max + 1);
        let mut rng = rand::thread_rng();
        let roll = between.ind_sample(&mut rng);
        self.value = roll;
        self.is_successful = true;
        self
    }

    pub fn set_sides(&mut self, sides: u8) {
        self.sides = sides
    }

    pub fn set_min(&mut self, min: i16) {
        self.min = min
    }

    pub fn set_max(&mut self, max: i16) {
        self.max = max
    }

}
