// setup limit for error-chain
#![recursion_limit = "1024"]

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
use ttml::output::Output;
use ttml::parser::{parse_p, Program};

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
pub fn parse(raw_input: *mut c_char) -> *mut c_char {
    // Start the timer
    let start = Instant::now();
    let executed = Utc::now();
    // Take the input and safely covert it to a String
    let input = safe_string(raw_input);
    // keep a copy of the raw input to displau in our output
    let output_input = input.clone();

    // @todo build our vectors
    let errors = Vec::new();
    let messages = Vec::new();
    let rolls = Vec::new();
    let tokens = Vec::new();
    let version = String::from("0.1.0");

    // parse the macro into an executable program
    let prog = parse_p(input.as_slice());
    if prog.is_err() {
        // @todo actually handle the error
        CString::new("{}").unwrap().into_raw()
    } else {
        let (_, program) = prog.unwrap();

        let elapsed = start.elapsed();
        let execution_time = ((elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64);

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
// fn it_parses_input() {
    // let chars = CString::new("#test!say \"Hello\"").unwrap().into_raw();
    // let raw_output = parse(chars);
    // let json = safe_string(raw_output);
    // let output: Output = serde_json::from_str(&json).unwrap();

    // assert_eq!(output.input, "#test!say \"Hello\"");
    // assert_eq!(output.version, "0.1.0");
// }
