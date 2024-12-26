use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::utils::slurp_file;
use env_logger;
use log::{debug, info};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

use super::{CommandImpl, DynError};

//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day10 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct TopographicMap {
    trail_map: Vec<Vec<u32>>,
}

impl TopographicMap {
    pub fn new(trail_map: Vec<Vec<u32>>) -> Self {
        Self { trail_map }
    }
}

pub type Point = (usize, usize);
impl TopographicMap {
    pub fn find_trailheads(&self) -> Vec<Point> {
        let zeroes: Vec<Point> = self
            .trail_map
            .iter()
            .enumerate()
            .map(|(r, x)| {
                x.iter().enumerate().filter(|&(_, &val)| val == 0).map(move |(c, _)| (r, c))
            })
            .flatten()
            .collect();
        zeroes
    }

    pub fn trails(&self) -> HashSet<Point> {
        self.trail_map
            .iter()
            .enumerate()
            .map(|(r, x)| {
                x.iter().enumerate().filter(|&(_, &val)| val != 0).map(move |(c, _)| (r, c))
            })
            .flatten()
            .collect()
    }

    pub fn count_all_trailheads(&self) -> usize {
        let mut candidate_trailhead: Vec<Point> = self.find_trailheads();
        let mut trailhead_count: Vec<usize> = Vec::new();
        while let Some(trailhead) = candidate_trailhead.pop() {
            trailhead_count.push(self.count_distinct_trails(trailhead));
        }
        trailhead_count.iter().sum()
    }

    pub fn count_trailheads(&self, p: Point) -> usize {
        let mut trailheads: VecDeque<Point> = VecDeque::new();
        let mut visited: HashSet<Point> = HashSet::new();
        trailheads.push_back(p);
        let trails: HashSet<Point> = self.trails();
        let mut ntrails: usize = 0;
        while let Some(p) = trailheads.pop_front() {
            let value: u32 = self.trail_map[p.0][p.1];
            if value == 9 {
                ntrails += 1;
            }
            for d in Direction::iter() {
                if let Some(point) = d.add(p) {
                    if trails.contains(&point)
                        && self.trail_map[point.0][point.1] == value + 1
                        && !visited.contains(&point)
                    {
                        trailheads.push_back(point);
                        visited.insert(point);
                    }
                }
            }
        }
        ntrails
    }

    pub fn count_distinct_trails(&self, p: Point) -> usize {
        let mut trailheads: VecDeque<Point> = VecDeque::new();
        trailheads.push_back(p);
        let trails: HashSet<Point> = self.trails();
        let mut ntrails: usize = 0;
        while let Some(p) = trailheads.pop_front() {
            let value: u32 = self.trail_map[p.0][p.1];
            if value == 9 {
                ntrails += 1;
            }
            for d in Direction::iter() {
                if let Some(point) = d.add(p) {
                    if trails.contains(&point) && self.trail_map[point.0][point.1] == value + 1 {
                        trailheads.push_back(point);
                    }
                }
            }
        }
        ntrails
    }
}

#[derive(Debug, EnumIter)]
pub enum Direction {
    Above,
    Right,
    Below,
    Left,
}

impl Direction {
    pub fn add(&self, p: Point) -> Option<Point> {
        match self {
            Direction::Above if p.0 > 0 => Some((p.0 - 1, p.1)),
            Direction::Right => Some((p.0, p.1 + 1)),
            Direction::Below => Some((p.0 + 1, p.1)),
            Direction::Left if p.1 > 0 => Some((p.0, p.1 - 1)),
            _ => None,
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = digit1(input)?;
    let numbers = digits.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    Ok((input, numbers))
}

fn parse_challenge(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, numbers) = separated_list1(line_ending, parse_numbers)(input)?;
    Ok((input, numbers))
}

const RADIX: u32 = 10u32;

impl CommandImpl for Day10 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        println!("parse day 10");
        match parse_challenge(&blob_string) {
            Ok((_, numbers)) => {
                let trail_map: TopographicMap = TopographicMap::new(numbers);
                let n_trailheads: usize = trail_map.count_all_trailheads();
                println!("there are {:?} trailheads", n_trailheads);
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
        Ok(())
    }
}
