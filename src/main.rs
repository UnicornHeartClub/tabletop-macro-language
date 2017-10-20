#[macro_use] extern crate log;
#[macro_use] extern crate structopt_derive;
extern crate chrono;
extern crate loggerv;
extern crate structopt;
extern crate ttml;

use chrono::NaiveDateTime;
use structopt::StructOpt;
use ttml::die::Die;
use ttml::parser::parse_ttml;
use ttml::parser::execute_ast;
use ttml::token::Token;

#[derive(StructOpt, Debug)]
#[structopt(name="ttml-parser")]
struct Cli {
    /// Input to be interpretted by the TTML parser
    input: String,

    /// Enable logging, use multiple 'v's to increase verbosity
    #[structopt(short="v", long="verbose")]
    verbosity: u64,
}

struct Output {
    executed: NaiveDateTime,
    execution_time: i64,
    rolls: Vec<Die>,
    tokens: Vec<Token>,
}

pub fn main() {
    let args = Cli::from_args();

    // Init the logger
    loggerv::init_with_verbosity(args.verbosity).unwrap();

    // Run the parser
    info!("parsing ast for input: {}", args.input);
    let ast = parse_ttml(&args.input);

    info!("executing ast");
    execute_ast(&ast)
}
