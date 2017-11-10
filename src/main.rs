// #[macro_use] extern crate error_chain;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate ttml;

use chrono::prelude::Utc;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Instant;
use ttml::executor::execute_roll;
use ttml::output::Output;
use ttml::parser::{MacroOp, StepResult, StepValue, parse_p};

fn main() {
    // IGNORE ME!
}

fn safe_string(input: *mut c_char) -> Vec<u8> {
    unsafe {
        CStr::from_ptr(input).to_bytes().to_owned()
    }
}
        
/// Run input and return a typed array for use in javascript
#[no_mangle]
pub fn run_macro(raw_input: *mut c_char, raw_tokens: *mut c_char) -> *mut c_char {
    // Start the timer
    let start = Instant::now();
    let executed = Utc::now();
    // Take the input and safely covert it to a String
    let input = safe_string(raw_input);
    // keep a copy of the raw input to display in our output
    let output_input = input.clone();

    // @todo build our vectors
    let errors = Vec::new();
    let messages = Vec::new();
    let mut rolls = Vec::new();
    let version = String::from("0.1.0");

    // Build our tokens
    let tokens = Vec::new();

    // parse the macro into an executable program
    let prog = parse_p(input.as_slice());
    if prog.is_err() {
        // @todo actually handle the error
        CString::new("{}").unwrap().into_raw()
    } else {
        let (_, mut program) = prog.unwrap();
        // Anything we marked as "Pass" will be stored in the variable vector
        // We can reference any result by calling $# where # = the index of the vec
        // this is so we can use results in any subsequent macro function
        // e.g. $2 would resolve to the second passed result
        let mut results = Vec::new();

        for step in &mut program.steps {
            match step.op {
                MacroOp::Roll => {
                    // @todo check if we have a variable to replace

                    // execute the roll and update the step value
                    let roll = execute_roll(&step);
                    step.value = Some(StepValue::Int(roll.value));

                    // pass the result if needed
                    if step.result == StepResult::Pass {
                        results.push(StepValue::Int(roll.value));
                    }

                    // push to the tracked rolls
                    rolls.push(roll);
                },
                _ => println!("Not yet implemented {:?}", step.op)
            }
        };

        let elapsed = start.elapsed();
        let execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;

        let output = Output {
            input: String::from_utf8(output_input).unwrap(),
            executed,
            execution_time,
            errors,
            messages,
            program,
            rolls,
            tokens,
            version,
        };

        let json = serde_json::to_string(&output).unwrap();
        CString::new(json).unwrap().into_raw()
    }
}

// #[test]
// fn it_parses_simple_input() {
    // let chars = CString::new("#test!say \"Hello\"").unwrap().into_raw();
    // let raw_output = parse(chars);
    // let json = safe_string(raw_output);
    // let output: Output = serde_json::from_str(&json).unwrap();

    // assert_eq!(output.input, "#test!say \"Hello\"");
    // assert_eq!(output.version, "0.1.0");
// }
