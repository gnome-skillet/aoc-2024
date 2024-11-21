pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day1a(day1a::Day1a),
    Day1b(day1b::Day1b),
    Day2a(day2a::Day2a),
    Day2b(day2b::Day2b),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
