use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use std::collections::HashSet;

#[derive(Parser, Debug)]
pub struct Day6a {
    #[clap(long, short)]
    input: PathBuf,
}

pub fn differences(vec: &[i32]) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'V' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        }
    }

    pub fn advance(&self, p: (usize, usize)) -> (usize, usize) {
        match *self {
            Direction::Up => (p.0 - 1, p.1),
            Direction::Right => (p.0, p.1 + 1),
            Direction::Down => (p.0 + 1, p.1),
            Direction::Left => (p.0, p.1 - 1),
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn exiting_map(d: &Direction, p: &(usize, usize), mapped_area: &[Vec<char>]) -> bool {
    match *d {
        Direction::Up => p.0 == 0,
        Direction::Down => p.0 == mapped_area.len() - 1,
        Direction::Left => p.1 == 0,
        Direction::Right => p.1 == mapped_area[0].len() - 1,
    }
}

pub fn starting_position(mapped_area: &[Vec<char>]) -> Option<((usize, usize), Direction)> {
    for (r, row) in mapped_area.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '^' {
                return Some(((r, c), Direction::new(*col)));
            }
        }
    }
    None
}

pub fn blocked(mapped_area: &[Vec<char>], p: Option<(usize, usize)>) -> bool {
    if let Some((row, col)) = p {
        return mapped_area[row][col] == '#';
    };
    true
}

impl CommandImpl for Day6a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mapped_area: Vec<Vec<char>> =
            lines.iter().map(|s| s.split("").flat_map(|x| x.parse::<char>()).collect()).collect();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        if let Some((mut p, mut d)) = starting_position(&mapped_area) {
            visited.insert(p);
            while !exiting_map(&d, &p, &mapped_area) {
                let next_p = d.advance(p);
                if !blocked(&mapped_area, Some(next_p)) {
                    p = next_p;
                    visited.insert(p);
                } else {
                    d = d.rotate();
                }
            }
        }

        println!("visited {:?} unique positions", visited.len());
        Ok(())
    }
}
