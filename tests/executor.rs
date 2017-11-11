extern crate ttml;

use ttml::parser::*;
use ttml::executor::execute_roll;

#[test]
fn it_returns_a_roll() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let results = vec![];
    let roll = execute_roll(&step, &results);

    assert!(roll.value >= 1);
    assert!(roll.value <= 20);
    assert_eq!(roll.dice.len(), 1);
}

#[test]
fn it_uses_variables() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let results = vec![
        StepValue::Number(5),
    ];
    let roll = execute_roll(&step, &results);

    assert!(roll.value >= 5);
    assert!(roll.value <= 100);
    assert_eq!(roll.dice.len(), 5);
}

// #[test]
// fn it_executes_simple_input() {
    // let chars = CString::new("#test!say \"Hello\"").unwrap().into_raw();
    // let raw_output = parse(chars);
    // let json = safe_string(raw_output);
    // let output: Output = serde_json::from_str(&json).unwrap();

    // assert_eq!(output.input, "#test!say \"Hello\"");
    // assert_eq!(output.version, "0.1.0");
// }
