use std::ffi::CString;
use std::ffi::CStr;

/// Safe wrapper for our JS function `prompt`.
pub fn prompt(x: &str) -> String {
    let x = CString::new(x).unwrap();
    let mut ptr = x.as_ptr();
    let data = unsafe {
        CStr::from_ptr(ffi::prompt(ptr))
    };
    data.to_string_lossy().into_owned()
}

// Lovingly adopted from https://github.com/kainino0x/wasm-call-js-from-rust
mod ffi {
    use libc::*;

    extern "C" {
        // This extern is defined in `html/library.js`.
        pub fn prompt(x: *const c_char) -> *mut c_char;
    }
}

