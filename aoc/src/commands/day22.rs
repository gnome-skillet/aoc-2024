use std::path::PathBuf;

use clap::Parser;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fs;

use std::collections::HashSet;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day22 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum Sequence {
    One(u64),
    Two(u64),
    Three(u64),
}

pub trait Unwrappable {
    type Item;
    fn unwrap(&self) -> Self::Item;
}

impl Unwrappable for Sequence {
    type Item = u64;
    fn unwrap(&self) -> Self::Item {
        match *self {
            Sequence::One(s) => s,
            Sequence::Two(s) => s,
            Sequence::Three(s) => s,
        }
    }
}

pub fn div_floor(x: u64, y: u64) -> u64 {
    let x2: f64 = x as f64;
    let y2: f64 = y as f64;
    let result = x2 / y2;
    result.floor() as u64
}

pub fn mix(x: u64, y: u64) -> u64 {
    let result: u64 = x ^ y;
    result
}

pub fn prune(x: u64) -> u64 {
    let result: u64 = x % 16777216_u64;
    result
}

pub fn step1(secret_number: u64) -> u64 {
    let value: u64 = secret_number * 64_u64;
    let mix_value: u64 = mix(secret_number, value);
    prune(mix_value)
}

impl Sequence {
    fn calculate(self) -> Self {
        match self {
            Sequence::One(s) => Sequence::Two(step1(s)),
            Sequence::Two(s) => Sequence::Three(prune(mix(s, s / 32u64))),
            Sequence::Three(s) => Sequence::One(prune(mix(s, s * 2048u64))),
        }
    }

    fn mutate(self) -> Self {
        let mut x = self;
        for _ in 0..6000 {
            x = x.calculate();
        }
        x
    }
}

fn my_digit(input: &str) -> IResult<&str, Sequence> {
    let (input, digits) = digit1(input)?;
    let x: u64 = digits.parse::<u64>().unwrap();
    Ok((input, Sequence::One(x)))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<Sequence>> {
    let (input, numbers) = separated_list1(line_ending, my_digit)(input)?;
    Ok((input, numbers))
}

impl CommandImpl for Day22 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        if let Ok((_, mut numbers)) = parse_numbers(&blob_string) {
            let results = numbers.iter_mut().map(|x| x.mutate()).collect::<Vec<Sequence>>();
            let sum: u64 = results.iter().map(|x| x.unwrap()).sum();
            let max: u64 = results.iter().map(|x| x.unwrap()).max().unwrap();
            let occurrences: Vec<usize> = results
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.unwrap()))
                .filter(|(_, v)| *v == max)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            println!("sum {:?}", sum);
            println!("the max value is {:?}", max);
            println!("occurrences: {:?}", occurrences);
        } else {
            println!("unable to read {:?}", &self.input);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1u64, 8685429u64)]
    #[case(10u64, 4700978u64)]
    #[case(100u64, 15273692u64)]
    #[case(2024u64, 8667524u64)]
    fn test_mutate(
        #[case] input: u64,
        #[case] expected: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let x: Sequence = Sequence::One(input);
        let step2: Sequence = x.mutate();
        match step2 {
            Sequence::One(observed) => assert_eq!(observed, expected),
            _ => panic!(),
        }
        Ok(())
    }

    #[test]
    fn test_first_sequence() -> Result<(), Box<dyn std::error::Error>> {
        let one: Sequence = Sequence::One(123);
        let two: Sequence = one.calculate();
        let three: Sequence = two.calculate();
        let one: Sequence = three.calculate();
        println!("one: {:?}", one);
        let expected: u64 = 15887950;
        match one {
            Sequence::One(observed) => assert_eq!(observed, expected),
            _ => panic!(),
        }
        Ok(())
    }

    #[rstest]
    #[case(42u64, 15u64, 37u64)]
    fn test_mix(
        #[case] lhs: u64,
        #[case] rhs: u64,
        #[case] expected: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let observed: u64 = lhs ^ rhs;
        assert_eq!(observed, expected);
        Ok(())
    }

    #[rstest]
    #[case(100000000_u64, 16113920_u64)]
    fn test_prune(
        #[case] operand: u64,
        #[case] expected: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let observed: u64 = prune(operand);
        assert_eq!(observed, expected);
        Ok(())
    }

    #[rstest]
    #[case(8u64, 3u64, 2u64)]
    #[case(9u64, 4u64, 2u64)]
    fn test_div_floor(
        #[case] dividend: u64,
        #[case] divisor: u64,
        #[case] expected: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //let observed: u64 = div_floor(dividend, divisor);
        let observed: u64 = dividend / divisor;
        assert_eq!(observed, expected);
        Ok(())
    }
}
