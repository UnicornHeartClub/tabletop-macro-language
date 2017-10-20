extern crate chrono;
extern crate uuid;

use self::chrono::DateTime;
use self::chrono::prelude::Utc;
use self::uuid::Uuid;

#[derive(Debug,PartialEq)]
pub enum DieType {
    d3,
    d4,
    d6,
    d8,
    d10,
    d20,
    d100,
    other,
}

#[derive(Debug)]
pub struct Die {
    /// Unique identifier of the die
    pub _id: Uuid,

    /// The type of die (e.g. d20, d100)
    pub die: DieType,

    /// Modifiers to apply to the value
    pub modifiers: Vec<u8>,

    /// Value of the rolled die
    pub raw_value: u8,

    /// Timestamp of the roll
    pub timestamp: DateTime<Utc>,

    /// The number of faces the die has
    pub sides: i8,

    /// The token rolling the die (optional)
    pub token: Option<String>,

    /// The total calculated from the raw_value and modifiers
    pub total: u16,
}

impl Die {
    pub fn new (sides: i8, token: Option<String>) -> Die {
        Die {
            _id: Uuid::new_v4(),
            sides,
            token,
            modifiers: Vec::new(),
            die: DieType::d20,
            raw_value: 0,
            total: 0,
            timestamp: Utc::now(),
        }
    }

    /// Roll the die, generating a random number and calculating any modifiers
    pub fn roll(&self) -> &Die {
        // generate a random number
        self
    }
}

/// Roll any sided die
pub fn roll_die(sides: i8, count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    let number = count.unwrap_or(1); // number of die to roll
    let count = [ 0..number ];
    let iter = count.iter();
    let mut dice = Vec::new();
    for x in iter {
        let die = Die::new(sides, Some(token.clone()).unwrap_or_default());
        die.roll();
        dice.push(die);
    }
    dice
}

pub fn roll_d4(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(4, count, token, modifier)
}

pub fn roll_d6(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(6, count, token, modifier)
}

pub fn roll_d8(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(8, count, token, modifier)
}

pub fn roll_d10(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(10, count, token, modifier)
}

pub fn roll_d12(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(12, count, token, modifier)
}

pub fn roll_d20(count: Option<i8>, token: Option<String>, modifier: Option<u8>) -> Vec<Die> {
    roll_die(20, count, token, modifier)
}
