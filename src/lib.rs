#[macro_use] extern crate error_chain;
extern crate chrono;
extern crate rand;
extern crate uuid;

pub mod die;
pub mod errors;
pub mod parser;
pub mod roll;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
