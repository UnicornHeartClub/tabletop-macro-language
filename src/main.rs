#[cfg(feature = "web")]
extern crate stdweb;

extern crate serde;
extern crate serde_json;
extern crate ttml;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use ttml::executor::execute_macro;

// Initialize stdweb here since main() is called once on instantiation from JS-land
fn main() {
    #[cfg(feature = "web")]
    stdweb::initialize();

    #[cfg(feature = "web")]
    stdweb::event_loop();
}
        
/// Run input and return a typed array for use in javascript
#[no_mangle]
pub extern "C" fn run_macro(raw_input: *mut c_char, raw_tokens: *mut c_char) -> *mut c_char {
    // Parse the input
    let input = safe_string(raw_input);

    // Parse the token input
    let input_tokens = safe_string(raw_tokens);

    // Run the macro
    let output = execute_macro(input, input_tokens);

    // Return as JSON
    let json = serde_json::to_string(&output).unwrap();
    CString::new(json).unwrap().into_raw()
}

fn safe_string(input: *mut c_char) -> Vec<u8> {
    unsafe {
        CStr::from_ptr(input).to_bytes().to_owned()
    }
}
