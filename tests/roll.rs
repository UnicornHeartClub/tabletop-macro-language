extern crate ttml;

use ttml::roll::Roll;
use ttml::die::Die;
use ttml::die::DieType;

#[test]
fn it_can_create_a_roll() {
    // Create some random dice
    let d20 = Die::new(DieType::d20);
    let d8 = Die::new(DieType::d8);

    let dice = vec![ d20, d8 ];

    let roll = Roll::new(dice);

    assert!(roll.value >= 1);
    assert!(roll.value <= 28);
    assert!(roll.raw_value >= 1);
    assert!(roll.raw_value <= 28);
    assert_eq!(roll.dice.len(), 2);
}
