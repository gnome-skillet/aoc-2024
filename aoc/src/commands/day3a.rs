use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use regex::Regex;
//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day3a {
    #[clap(long, short)]
    input: PathBuf,
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
