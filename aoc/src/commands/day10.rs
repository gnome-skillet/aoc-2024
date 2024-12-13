use std::{error::Error, path::PathBuf, str::FromStr};

use clap::Parser;
use itertools::Itertools;

use nom::lib::std::cmp::Ordering;
use std::cmp::min;
use std::fs;
use std::ops::Range;

use crate::utils::{slurp_file, ParseError};
use std::collections::HashMap;

use super::{CommandImpl, DynError};

//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day10 {
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
    height: u32,
    coord: (usize, usize),
}

impl Node {
    pub fn new(id: u64, height: u32, coord: (usize, usize)) -> Self {
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

const RADIX: u32 = 10u32;

impl CommandImpl for Day10 {
    fn main(&self) -> Result<(), DynError> {
        let mut gen_id = (0..u64::MAX);
        let lines: Vec<String> = slurp_file(&self.input)?;
        let topographic_map: Vec<Vec<u32>> =
            lines.iter().map(|c| c.chars().map(|d| d.to_digit(RADIX).unwrap()).collect()).collect();
        let _nodes: Vec<Node> = (0..topographic_map.len())
            .cartesian_product(0..topographic_map[0].len())
            .map(|(row, col)| {
                Node::new(gen_id.next().unwrap(), topographic_map[row][col], (row, col))
            })
            .collect();
        //let hashmap: HashMap<u32, Vec<(usize, usize)>> = nodes
        //    .into_iter()
        //    .map(|t| (t.height, t.coord))
        //    .collect::<HashMap<u32, (usize, usize)>>();
        //println!("hashmap {:?} starts", hashmap);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
}
