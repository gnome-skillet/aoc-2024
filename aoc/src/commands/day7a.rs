use std::path::PathBuf;

use clap::Parser;
use std::fs;

use super::{CommandImpl, DynError};
use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;
//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day7a {
    #[clap(long, short)]
    input: PathBuf,
}

fn my_digit(input: &str) -> IResult<&str, u64> {
    let (input, digits) = digit1(input)?;
    let x: u64 = digits.parse::<u64>().unwrap();
    Ok((input, x))
}

fn parse_test(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (input, test_value) = my_digit(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, rhs) = separated_list1(space1, my_digit)(input)?;
    Ok((input, (test_value, rhs)))
}

fn concat(lhs: u64, rhs: u64) -> Result<u64, DynError> {
    let l: String = lhs.to_string();
    let r: String = rhs.to_string();
    let lr: String = format!("{l}{r}");
    Ok(lr.parse::<u64>()?)
}

fn parse_tests(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let (input, rhs) = separated_list1(line_ending, parse_test)(input)?;
    Ok((input, rhs))
}

fn solvable(test_value: u64, operands: &Vec<u64>) -> bool {
    let base: usize = 2;
    let capacity = operands.len() as u32;
    let mut solution_tree: VecDeque<(usize, u64)> = VecDeque::with_capacity(base.pow(capacity));
    solution_tree.push_back((0, operands[0]));

    while let Some(top) = solution_tree.pop_front() {
        if top.0 == operands.len() - 1 {
            if top.1 == test_value {
                return true;
            }
            continue;
        }
        let lhs: u64 = top.1 + operands[top.0 + 1];
        let rhs: u64 = top.1 * operands[top.0 + 1];
        let mid: u64 = concat(top.1, operands[top.0 + 1]).unwrap();
        if lhs <= test_value {
            solution_tree.push_back((top.0 + 1, lhs));
        }
        if rhs <= test_value {
            solution_tree.push_back((top.0 + 1, rhs));
        }
        if mid <= test_value {
            solution_tree.push_back((top.0 + 1, mid));
        }
    }
    false
}

impl CommandImpl for Day7a {
    fn main(&self) -> Result<(), DynError> {
        let string = fs::read_to_string(&self.input)?;
        if let Ok((_, tests)) = parse_tests(&string) {
            let sum: u64 = tests.iter().filter(|x| solvable(x.0, &x.1)).map(|x| x.0).sum();
            println!("sum = {:?}", sum);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("190: 10 19", 2usize)]
    fn test_parse_test(
        #[case] input: &'static str,
        #[case] expected: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expression = "190: 10 19";
        let (_, (x, v)) = parse_test(input)?;
        assert_eq!(x, 190u64);
        assert_eq!(v.len(), expected);
        Ok(())
    }

    #[rstest]
    #[case("190: 10 19", 2usize)]
    fn test_solvable(
        #[case] input: &'static str,
        #[case] expected: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expression = "190: 10 19";
        let (_, (x, v)) = parse_test(input)?;
        assert!(solvable(x, v));
        Ok(())
    }
}
