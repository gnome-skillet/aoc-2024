pub mod day0;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day3a;
pub mod day4a;
pub mod day4b;
pub mod day5a;
pub mod day5b;
pub mod day6a;
pub mod day7a;
pub mod day9a;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}
