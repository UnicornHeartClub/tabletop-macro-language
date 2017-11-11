// #[macro_use] extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate ttml;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use ttml::executor::execute_macro;

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
    let input = safe_string(raw_input);
    let output = execute_macro(input);

    let json = serde_json::to_string(&output).unwrap();
    CString::new(json).unwrap().into_raw()
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
