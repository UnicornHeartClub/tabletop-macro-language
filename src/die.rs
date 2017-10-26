extern crate chrono;
extern crate rand;
extern crate uuid;

use self::chrono::DateTime;
use self::chrono::prelude::Utc;
use self::rand::distributions::{IndependentSample, Range};
use self::uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DieType {
    d4,
    d6,
    d8,
    d10,
    d12,
    d20,
    d100,
    fate,
    other,
}

/// Determine the minimum number to roll based on the die type
fn get_die_min(die: &DieType) -> i8 {
    match die {
        &DieType::d4 => 1,
        &DieType::d6 => 1,
        &DieType::d8 => 1,
        &DieType::d10 => 1,
        &DieType::d12 => 1,
        &DieType::d20 => 1,
        &DieType::d100 => 1,
        &DieType::fate => -1,
        &DieType::other => 0,
    }
}

/// Determine the minimum number to roll based on the die type
fn get_die_max(die: &DieType) -> i8 {
    match die {
        &DieType::d4 => 4,
        &DieType::d6 => 6,
        &DieType::d8 => 8,
        &DieType::d10 => 10,
        &DieType::d12 => 12,
        &DieType::d20 => 20,
        &DieType::d100 => 100,
        &DieType::fate => 1,
        &DieType::other => 0,
    }
}

/// Determine the number of sides based on the die type
fn get_die_sides(die: &DieType) -> u8 {
    match die {
        &DieType::d4 => 4,
        &DieType::d6 => 6,
        &DieType::d8 => 8,
        &DieType::d10 => 10,
        &DieType::d12 => 12,
        &DieType::d20 => 20,
        &DieType::d100 => 100,
        &DieType::fate => 3,
        &DieType::other => 0,
    }
}

#[derive(Debug)]
pub struct Die {
    /// Unique identifier of the die
    pub _id: Uuid,

    /// The type of die (e.g. d20, d100)
    pub die: DieType,

    /// Maximum number to roll
    pub max: i8,

    /// Minimum number to roll
    pub min: i8,

    /// The number of faces the die has
    pub sides: u8,

    /// Timestamp of the roll
    pub timestamp: DateTime<Utc>,

    /// The determined value of the dice roll
    pub value: i8,
}

impl Die {
    pub fn new (die: DieType) -> Die {
        Die {
            _id: Uuid::new_v4(),
            die,
            max: get_die_max(&die),
            min: get_die_min(&die),
            sides: get_die_sides(&die),
            timestamp: Utc::now(),
            value: 0,
        }
    }

    pub fn set_sides(&mut self, sides: u8) {
        self.sides = sides
    }

    pub fn set_min(&mut self, min: i8) {
        self.min = min
    }

    pub fn set_max(&mut self, max: i8) {
        self.max = max
    }

    /// Roll the die, generating a random number and calculating any modifiers
    pub fn roll(&mut self) -> &Die {
        // generate a random number
        let between = Range::new(self.min, self.max);
        let mut rng = rand::thread_rng();
        let roll = between.ind_sample(&mut rng);
        println!("roll is {}", roll);
        self.value = roll;
        self
    }
}
