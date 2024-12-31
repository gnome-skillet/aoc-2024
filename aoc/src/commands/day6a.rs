use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use itertools::Itertools;

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

#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub enum DirectedParticle {
    East(usize, usize),
    South(usize, usize),
    West(usize, usize),
    North(usize, usize),
}

impl DirectedParticle {
    pub fn new(r: usize, c: usize) -> Self {
        DirectedParticle::North(r, c)
    }

    pub fn advance(&self) -> Self {
        match *self {
            DirectedParticle::North(r, c) => DirectedParticle::North(r - 1, c),
            DirectedParticle::East(r, c) => DirectedParticle::East(r, c + 1),
            DirectedParticle::South(r, c) => DirectedParticle::South(r + 1, c),
            DirectedParticle::West(r, c) => DirectedParticle::West(r, c - 1),
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            DirectedParticle::North(r, c) => DirectedParticle::East(r, c),
            DirectedParticle::East(r, c) => DirectedParticle::South(r, c),
            DirectedParticle::South(r, c) => DirectedParticle::West(r, c),
            DirectedParticle::West(r, c) => DirectedParticle::North(r, c),
        }
    }
    pub fn row(&self) -> usize {
        match *self {
            DirectedParticle::North(r, _) => r,
            DirectedParticle::East(r, _) => r,
            DirectedParticle::South(r, _) => r,
            DirectedParticle::West(r, _) => r,
        }
    }

    pub fn column(&self) -> usize {
        match *self {
            DirectedParticle::North(_, c) => c,
            DirectedParticle::East(_, c) => c,
            DirectedParticle::South(_, c) => c,
            DirectedParticle::West(_, c) => c,
        }
    }

    pub fn coord(&self) -> (usize, usize) {
        match *self {
            DirectedParticle::North(r, c) => (r, c),
            DirectedParticle::East(r, c) => (r, c),
            DirectedParticle::South(r, c) => (r, c),
            DirectedParticle::West(r, c) => (r, c),
        }
    }
    pub fn exiting_map(&self, dimensions: &(usize, usize)) -> bool {
        match *self {
            DirectedParticle::North(r, _) => r == 0,
            DirectedParticle::South(r, _) => r == dimensions.0 - 1,
            DirectedParticle::West(_, c) => c == 0,
            DirectedParticle::East(_, c) => c == dimensions.1 - 1,
        }
    }
}

pub fn starting_position(mapped_area: &[Vec<char>]) -> Option<DirectedParticle> {
    for (r, row) in mapped_area.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '^' {
                return Some(DirectedParticle::new(r, c));
            }
        }
    }
    None
}

pub fn obstacles(mapped_area: &[Vec<char>]) -> HashSet<(usize, usize)> {
    (0..mapped_area.len())
        .cartesian_product(0..mapped_area[0].len())
        .filter(|p| mapped_area[p.0][p.1] == '#')
        .collect::<HashSet<(usize, usize)>>()
}

pub fn dimensions(mapped_area: &[Vec<char>]) -> (usize, usize) {
    (mapped_area.len(), mapped_area[0].len())
}

pub fn simulate(
    start: DirectedParticle,
    obstacles: &HashSet<(usize, usize)>,
    dimensions: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut guard = start;

    visited.insert(guard.coord());
    while !guard.exiting_map(dimensions) {
        let next_p = guard.advance();
        if !obstacles.contains(&next_p.coord()) {
            guard = next_p;
            visited.insert(guard.coord());
        } else {
            guard = guard.rotate();
        }
    }
    visited
}

pub fn stuck_in_a_loop(
    start: DirectedParticle,
    obstacles: &HashSet<(usize, usize)>,
    dimensions: &(usize, usize),
) -> bool {
    let mut visited: HashSet<DirectedParticle> = HashSet::new();
    let mut guard: DirectedParticle = start;

    visited.insert(guard);
    while !guard.exiting_map(dimensions) {
        let next_p = guard.advance();
        if visited.contains(&next_p) {
            return true;
        } else if !obstacles.contains(&next_p.coord()) {
            guard = next_p;
            visited.insert(guard);
        } else {
            guard = guard.rotate();
        }
    }
    false
}

impl CommandImpl for Day6a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mapped_area: Vec<Vec<char>> =
            lines.iter().map(|s| s.split("").flat_map(|x| x.parse::<char>()).collect()).collect();
        let mut obstacles = obstacles(&mapped_area);
        let dimensions = dimensions(&mapped_area);
        let mut nloops: usize = 0usize;
        if let Some(guard) = starting_position(&mapped_area) {
            let visited = simulate(guard, &obstacles, &dimensions);
            println!("guard visited {:?} unique positions", visited.len());
            for p in visited.iter() {
                obstacles.insert(*p);
                if stuck_in_a_loop(guard, &obstacles, &dimensions) {
                    nloops += 1;
                }
                obstacles.remove(p);
            }
            println!("there are {nloops} loops");
        }

        Ok(())
    }
}
