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
    Day3a(day3a::Day3a),
    Day4a(day4a::Day4a),
    Day4b(day4b::Day4b),
    Day5a(day5a::Day5a),
    Day5b(day5b::Day5b),
    Day6a(day6a::Day6a),
    Day7a(day7a::Day7a),
    Day9a(day9a::Day9a),
    Day10(day10::Day10),
    Day11(day11::Day11),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
