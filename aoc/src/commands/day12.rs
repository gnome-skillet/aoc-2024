use std::path::PathBuf;

use clap::Parser;

use std::collections::{HashMap, HashSet};
use std::fs;

use super::{CommandImpl, DynError};
use itertools::Itertools;

#[derive(Parser, Debug)]
pub struct Day12 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point {
    color: char,
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(color: char, row: usize, col: usize) -> Self {
        Self { color, row, col }
    }

    pub fn dist(&self, other: &Point) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    pub fn neighbor(&self, other: &Point) -> bool {
        self.dist(other) == 1 && self.color == other.color
    }
}

pub fn neighbor(x: (usize, usize), y: (usize, usize)) -> bool {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1) == 1
}

pub fn connected_components(nodes: &[(usize, usize)]) -> Option<(usize, usize)> {
    if nodes.len() < 2 {
        return Some((nodes.len(), 4 * nodes.len()));
    }
    let ntotal: usize = nodes.len(); // get ntotal
    let nedges = nodes.into_iter().combinations(2).filter(|x| neighbor(*x[0], *x[1])).count();
    Some((ntotal, 4 * ntotal - 2 * nedges))
}

impl CommandImpl for Day12 {
    fn main(&self) -> Result<(), DynError> {
        let plant_string = fs::read_to_string(&self.input)?;
        let plantvec: Vec<Vec<char>> =
            plant_string.split_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let plots = plantvec
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| Point::new(*v, x, y)))
            .collect::<Vec<Point>>();
        let mut plantmap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for plant in plots.iter() {
            plantmap
                .entry(plant.color)
                .and_modify(|x| {
                    x.push((plant.row, plant.col));
                })
                .or_insert({
                    let mut h = Vec::new();
                    h.push((plant.row, plant.col));
                    h
                });
        }
        for k in plantmap.keys() {
            println!("key: {k}");
        }
        let v: Vec<(usize, usize)> =
            plantmap.into_iter().filter_map(|(_, x)| connected_components(&x)).collect();
        let answer: usize = v.iter().map(|(x, y)| x * y).sum();
        println!("{:?}", v);
        println!("answer: {:?}", answer);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
}
