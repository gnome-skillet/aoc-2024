use std::path::PathBuf;

use clap::Parser;

use nom::lib::std::cmp::Ordering;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day11 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Space {
    id: Option<usize>,
    range: Range<usize>,
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.range.start == other.range.start
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

#[derive(Debug)]
pub struct TopographicMap {
    map: Vec<Vec<Space>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    id: u64,
    height: u64,
    coord: (usize, usize),
}

impl Node {
    pub fn new(id: u64, height: u64, coord: (usize, usize)) -> Self {
        Self { id: id, height, coord }
    }

    pub fn dist(&self, other: &Self) -> usize {
        self.coord.0.abs_diff(other.coord.0) + self.coord.1.abs_diff(other.coord.1)
    }

    pub fn descendent(&self, other: &Self) -> Node {
        Self { id: self.id.clone(), height: other.height, coord: other.coord }
    }

    pub fn connected(&self, other: &Self) -> Option<Node> {
        if self.dist(other) == 1 {
            Some(self.descendent(other))
        } else {
            None
        }
    }
}

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// engraved with an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
const RADIX: u64 = 10u64;

impl CommandImpl for Day11 {
    fn main(&self) -> Result<(), DynError> {
        let stone_string = fs::read_to_string(&self.input)?;
        let stonevec: Vec<(String, usize)> =
            stone_string.split_whitespace().map(|x| (x.to_string(), 1)).collect();
        //let n: usize = 75;
        let n: usize = 75;
        let mut stone_map = stonevec.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.0.clone()).or_insert(0) += c.1;
            acc
        });
        println!("stone_map: {:?}", stone_map);
        for i in 0..n {
            let stonevec: Vec<(String, usize)> =
                stone_map.iter().map(|(x, i)| morph((x.to_string(), *i))).flatten().collect();
            stone_map = stonevec.iter().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c.0.clone()).or_insert(0) += c.1;
                acc
            });
            //println!("stone_map({:?}): {:?}", i, stone_map);
        }

        let sum: usize = stone_map.values().sum();
        println!("length converted stones({n}): {:?}", sum);
        Ok(())
    }
}

pub fn morph(stone: (String, usize)) -> Vec<(String, usize)> {
    let re = Regex::new(r"^([0-9]{2}+)$").unwrap();
    let caps = re.captures(&stone.0);
    if let Some(s) = caps {
        let x = s.get(0).unwrap().as_str();
        let strlen: usize = x.len() / 2;
        let lhs: u64 = x[..strlen].parse::<u64>().unwrap();
        let lhs: String = lhs.to_string();
        let rhs: u64 = x[strlen..].parse::<u64>().unwrap();
        let rhs: String = rhs.to_string();
        vec![(lhs, stone.1), (rhs, stone.1)]
    } else if stone.0 == "0" {
        vec![("1".to_string(), stone.1)]
    } else {
        let my_int = stone.0.parse::<u64>().unwrap();
        let x: u64 = my_int * 2024;
        vec![(x.to_string(), stone.1)]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
}
