use std::{error::Error, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;
use regex::Regex;
//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day3a {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_token(input: &str) -> IResult<&str, Vec<u64>> {
    many0(parse_expression)(input)
}

fn parse_expression(input: &str) -> IResult<&str, u64> {
    println!("parse_mult");
    let (input, _) = take_until("mul")(input)?;
    let (input, product) = parse_mult(input)?;
    Ok((input, product))
}

fn parse_mult(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("mul")(input)?;
    let (input, product) = delimited(tag("("), parse_product, tag(")"))(input)?;
    println!("remaining input: {input}, product: {product}");
    Ok((input, product))
}

fn my_digit(input: &str) -> IResult<&str, u64> {
    let (input, digits) = digit1(input)?;
    println!("remaining input: {input}, digits: {:?}", digits);
    let x: u64 = digits.parse::<u64>().unwrap();
    Ok((input, x))
}

fn parse_product(input: &str) -> IResult<&str, u64> {
    let (input, lhs) = alt((my_digit, parse_mult))(input)?;
    println!("remaining input: {input}, lhs: {:?}", lhs);
    let (input, _) = tag(",")(input)?;
    let (input, rhs) = alt((my_digit, parse_mult))(input)?;
    println!("remaining input: {input}, rhs: {:?}", rhs);
    Ok((input, lhs * rhs))
}

impl CommandImpl for Day3a {
    fn main(&self) -> Result<(), DynError> {
        let string: Vec<String> = slurp_file(&self.input)?;
        println!("length of string is {:?}", string.len());
        //let re_set =
        //    RegexSet::new(&[r"do\(\)", r"don't\(\)", r"mul\(([0-9]{1,3}),([0-9]{1,3})\)"]).unwrap();
        let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))")?;
        let re_mult = Regex::new(r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)")?;
        let mut pushable: bool = true;
        let mut products = vec![];
        for (_, s) in string.iter().enumerate() {
            for (_, [msg]) in re.captures_iter(&s).map(|c| c.extract()) {
                //println!("{msg}");
                if msg == "do()" {
                    pushable = true;
                } else if msg == "don't()" {
                    pushable = false;
                } else if pushable {
                    let caps = re_mult.captures(msg).unwrap();
                    let lhs: u64 = caps["lhs"].parse::<u64>()?;
                    let rhs: u64 = caps["rhs"].parse::<u64>()?;
                    products.push(lhs * rhs);
                };
            }
        }
        println!("products: {:?}", products);
        let sum_product: u64 = products.iter().sum();
        println!("Results: {:?}", sum_product);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn test_expression() -> Result<(), Box<dyn std::error::Error>> {
        let expression = "mul(1,2)";
        let (_, result) = parse_mult(expression)?;
        println!("{:?}", result);
        assert_eq!(result, 2u64);
        Ok(())
    }

    #[rstest]
    #[case("mul(1,2)", 2)]
    #[case("mul(0,1)", 0)]
    #[case("xxxmul(5,1)", 5)]
    #[case("mul(5,mul(2,2))", 20)]
    #[case("mul(mul(2,2),mul(3,3))", 36)]
    fn test_cases(
        #[case] input: &'static str,
        #[case] expected: u64,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let (_, observed) = parse_expression(input)?;
        assert_eq!(expected, observed);
        Ok(())
    }
}
