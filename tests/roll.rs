extern crate ttml;

use ttml::roll::Roll;
use ttml::die::Die;
use ttml::die::DieType;

#[test]
fn it_can_create_a_roll() {
    // Create some random dice
    let d20 = Die::new(DieType::D20);
    let d8 = Die::new(DieType::D8);
    let dice = vec![ d20, d8 ];
    let roll = Roll::new(dice);

    assert!(roll.value >= 1);
    assert!(roll.value <= 28);
    assert!(roll.raw_value >= 1);
    assert!(roll.raw_value <= 28);
    assert_eq!(roll.dice.len(), 2);
}

#[test]
fn it_can_add_a_token_to_roll() {
    // Create some random dice
    let d4 = Die::new(DieType::D4);
    let d8 = Die::new(DieType::D8);
    let dice = vec![ d4, d8 ];
    let mut roll = Roll::new(dice);
    let token = String::from("test token");
    roll.add_token(token.clone());

    assert_eq!(roll.token, Some(token));
}
