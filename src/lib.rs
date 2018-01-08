#![recursion_limit="128"]
#![feature(proc_macro, conservative_impl_trait, generators)]

// Compiling for the web requires some extra modules
#[cfg(feature = "web")]
#[macro_use] extern crate stdweb;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate futures_await as futures;
extern crate rand;
extern crate serde_json;
extern crate uuid;

pub mod arg;
pub mod die;
pub mod errors;
pub mod executor;
pub mod message;
pub mod output;
pub mod parser;
pub mod roll;
pub mod step;
pub mod token;
pub mod wasm;
