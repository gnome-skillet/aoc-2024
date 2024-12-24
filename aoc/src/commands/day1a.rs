use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;
use std::str::FromStr;

use super::{CommandImpl, DynError};
use std::collections::HashMap;

/// Find the elf with the most calories in their pack.
#[derive(Parser, Debug)]
pub struct Day1a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let x: Vec<(u32, u32)> = lines
            .iter()
            .map(|s| s.split("   ").collect::<Vec<&str>>())
            .map(|s| (u32::from_str(s[0]).unwrap(), u32::from_str(s[1]).unwrap()))
            .collect();
        let mut left: Vec<u32> = x.iter().map(|x| x.0).collect();
        println!("x is {:?}", x);
        let mut right: Vec<u32> = x.iter().map(|x| x.1).collect();
        left.sort();
        right.sort();
        println!("left is {:?}", left);
        println!("right is {:?}", right);
        //let tuples: Vec<(u32, u32)> = std::iter::zip(left, right).collect();
        //let sum_diffs: u32 = tuples.iter().map(|x| x.0.abs_diff(x.1)).sum();
        let mut map: HashMap<u32, u32> = HashMap::new();
        for key in right {
            map.entry(key).or_insert(0);
            let count = map.get(&key).unwrap();
            map.insert(key, count + 1);
        }
        let part_b_sum: u32 = left.iter().filter(|x| map.contains_key(x)).map(|x| x * map[x]).sum();
        //println!("part a: sum diffs is {:?}", sum_diffs);
        println!("part b: sum diffs is {:?}", part_b_sum);
        Ok(())
    }
}
