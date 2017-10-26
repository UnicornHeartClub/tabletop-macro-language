extern crate ttml;

use ttml::die::Die;
use ttml::die::DieType;

#[test]
fn it_can_create_dice() {
    // Create some random dice
    let d20 = Die::new(DieType::D20);
    assert_eq!(d20.die, DieType::D20);
    assert_eq!(d20.sides, 20);
    assert_eq!(d20.min, 1);
    assert_eq!(d20.max, 20);

    let d4 = Die::new(DieType::D4);
    assert_eq!(d4.die, DieType::D4);
    assert_eq!(d4.sides, 4);
    assert_eq!(d4.min, 1);
    assert_eq!(d4.max, 4);

    let fate = Die::new(DieType::Fate);
    assert_eq!(fate.die, DieType::Fate);
    assert_eq!(fate.sides, 3);
    assert_eq!(fate.min, -1);
    assert_eq!(fate.max, 1);
}

#[test]
fn it_can_set_die_sides() {
    let mut custom = Die::new(DieType::Other);
    custom.set_sides(4);
    assert_eq!(custom.die, DieType::Other);
    assert_eq!(custom.sides, 4);
}

#[test]
fn it_can_set_die_min() {
    let mut custom = Die::new(DieType::Other);
    custom.set_min(-5);
    assert_eq!(custom.die, DieType::Other);
    assert_eq!(custom.min, -5);
}

#[test]
fn it_can_set_die_max() {
    let mut custom = Die::new(DieType::Other);
    custom.set_max(-50);
    assert_eq!(custom.die, DieType::Other);
    assert_eq!(custom.max, -50);
}

#[test]
fn it_can_roll_die() {
    let mut die = Die::new(DieType::D20);
    die.roll();
    assert!(die.value >= 1);
    assert!(die.value <= 20);

    let mut custom = Die::new(DieType::Other);
    custom.set_max(-5);
    custom.set_min(-8);
    custom.roll();
    assert!(custom.value >= -8);
    assert!(custom.value <= -5);
}
