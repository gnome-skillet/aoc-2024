use std::path::PathBuf;

use clap::Parser;

use env_logger;
use log::{debug, error, info, log_enabled, Level};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fs;

use std::collections::HashSet;
use std::collections::VecDeque;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day18 {
    #[clap(long, short)]
    input: PathBuf,
}

fn my_digit(input: &str) -> IResult<&str, usize> {
    let (input, digits) = digit1(input)?;
    let x: usize = digits.parse::<usize>().unwrap();
    Ok((input, x))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum Register {
    A(i32),
    B(i32),
    C(i32),
}

fn parse_command(input: &str) -> IResult<&str, Point> {
    debug!("parse_command: {input}");
    let (input, command) = separated_pair(my_digit, tag(","), my_digit)(input)?;
    Ok((input, command))
}

fn parse_corrupted_bytes(input: &str) -> IResult<&str, Vec<Point>> {
    debug!("parse_corrupted_bytes: {input}");
    let (input, corrupted_bytes) = separated_list1(line_ending, parse_command)(input)?;
    Ok((input, corrupted_bytes))
}

pub fn largest_row(barriers: &Vec<Point>) -> usize {
    *barriers.iter().map(|(r, _)| r).max().unwrap()
}

pub fn largest_column(barriers: &Vec<Point>) -> usize {
    *barriers.iter().map(|(_, c)| c).max().unwrap()
}

pub type Point = (usize, usize);
impl Indexable for Point {
    fn row(&self) -> usize {
        self.0
    }

    fn column(&self) -> usize {
        self.1
    }

    fn offset(&self, rlen: usize) -> usize {
        self.0 * rlen + self.1
    }
}

impl Neighborly for Point {
    type Item = Point;

    fn neighbors(&self, nrows: usize, ncols: usize) -> Vec<Self::Item> {
        let mut neighbors: Vec<Point> = Vec::new();
        if self.0 > 0 {
            let above: Point = (self.0 - 1, self.1);
            neighbors.push(above);
        }
        if self.1 > 0 {
            let left: Point = (self.0, self.1 - 1);
            neighbors.push(left);
        }
        if self.0 < nrows - 1 {
            let below: Point = (self.0 + 1, self.1);
            neighbors.push(below);
        }
        if self.1 < ncols - 1 {
            let right: Point = (self.0, self.1 + 1);
            neighbors.push(right);
        }
        neighbors
    }
}

trait Indexable {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn offset(&self, rlen: usize) -> usize;
}
trait Neighborly {
    type Item;
    fn neighbors(&self, nrows: usize, ncols: usize) -> Vec<Self::Item>;
}

#[derive(Debug)]
pub struct Maze {
    dimension: Point,
    visited: HashSet<Point>,
}

impl Maze {
    pub fn new(barriers: Vec<Point>) -> Self {
        let nrows: usize = largest_row(&barriers) + 1 as usize;
        let ncols: usize = largest_column(&barriers) + 1 as usize;
        let mut visited: HashSet<Point> = HashSet::from_iter(barriers);
        Maze { dimension: (nrows, ncols), visited }
    }

    pub fn shortest_path(&mut self) -> Option<usize> {
        let mut queue: VecDeque<Point> = VecDeque::new();
        let mut prev: Vec<usize> = vec![usize::MAX; self.dimension.column() * self.dimension.row()];
        let mut steps: Vec<usize> =
            vec![usize::MAX; self.dimension.column() * self.dimension.row()];
        let rlen: usize = self.dimension.column();
        let target: Point = (self.dimension.0 - 1, self.dimension.1 - 1);
        prev[0] = 0;
        steps[0] = 0;

        queue.push_front((0usize, 0usize));
        self.visited.insert((0usize, 0usize));
        while let Some(curr) = queue.pop_front() {
            if curr == target {
                return Some(steps[curr.offset(rlen)]);
            }
            let neighbors = curr.neighbors(self.dimension.0, self.dimension.1);
            println!("{:?} neigbors: {:?}", curr, neighbors);
            for neighbor in neighbors {
                if !self.visited.contains(&neighbor) {
                    println!("    push {:?}", neighbor);
                    queue.push_back(neighbor);
                    self.visited.insert(neighbor);
                    prev[neighbor.offset(rlen)] = curr.offset(rlen);
                    steps[neighbor.offset(rlen)] = steps[curr.offset(rlen)] + 1;
                }
            }
        }
        None
    }
}

impl CommandImpl for Day18 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        debug!("main");
        let blob_string = fs::read_to_string(&self.input)?;
        match parse_corrupted_bytes(&blob_string) {
            Ok((_, corrupted_bytes)) => {
                let mut maze: Maze = Maze::new(corrupted_bytes[0..3014].to_vec());
                if let Some(steps) = maze.shortest_path() {
                    println!("shortest path: {steps}");
                } else {
                    println!("unable to find path");
                }
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    #[test]
    fn test_parse_command() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "4,2";
        let (_, command) = parse_command(input)?;
        assert_eq!(command, Command::BXC(2usize));
        Ok(())
    }

    #[test]
    fn test_parse_register() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Register A: 30";
        let (_, register) = parse_register(input)?;
        assert_eq!(register, Register::A(30));
        Ok(())
    }
}
