use std::path::PathBuf;

use clap::Parser;

use std::collections::HashMap;
use std::fs;

use super::{CommandImpl, DynError};
use itertools::Itertools;

#[derive(Parser, Debug)]
pub struct Day12 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Clone, Hash)]
pub struct Garden {
    plants: Vec<Plant>,
}

impl Garden {
    pub fn new(plants: Vec<Plant>) -> Self {
        Self { plants }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Plant {
    plant: char,
    row: usize,
    col: usize,
    component: Option<usize>,
}

impl Plant {
    pub fn new(plant: char, row: usize, col: usize) -> Self {
        Self { plant, row, col, component: None }
    }

    pub fn dist(&self, other: &Plant) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    pub fn neighbor(&self, other: &Plant) -> bool {
        self.dist(other) == 1 && self.plant == other.plant
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
    let nedges = nodes.iter().combinations(2).filter(|x| neighbor(*x[0], *x[1])).count();
    Some((ntotal, 4 * ntotal - 2 * nedges))
}

impl CommandImpl for Day12 {
    fn main(&self) -> Result<(), DynError> {
        let plant_string = fs::read_to_string(&self.input)?;
        let plantvec: Vec<Vec<char>> =
            plant_string.split_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let plants = plantvec
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| Plant::new(*v, x, y)))
            .collect::<Vec<Plant>>();
        let mut plantmap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for plant in plants.iter() {
            plantmap
                .entry(plant.plant)
                .and_modify(|x| {
                    x.push((plant.row, plant.col));
                })
                .or_insert(vec![(plant.row, plant.col)]);
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
