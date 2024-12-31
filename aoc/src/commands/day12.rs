use std::path::PathBuf;

use clap::Parser;

use itertools::Itertools;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day12 {
    #[clap(long, short)]
    input: PathBuf,
}

type GardenPlot = (usize, usize, char);

pub trait Plantable {
    type Plant;
    fn plant_type(&self) -> Self::Plant;
}

impl Plantable for GardenPlot {
    type Plant = char;

    fn plant_type(&self) -> Self::Plant {
        self.2
    }
}

pub trait CornerCountable {
    type Node;
    type Graph;

    fn count_exterior_corners(&self, graph: &Self::Graph) -> usize;
    fn count_interior_corners(&self, graph: &Self::Graph) -> usize;
}

impl CornerCountable for GardenPlot {
    type Node = GardenPlot;
    type Graph = HashSet<GardenPlot>;

    fn count_exterior_corners(&self, graph: &Self::Graph) -> usize {
        let columns = [Neighbor::North, Neighbor::South];
        let rows = [Neighbor::East, Neighbor::West];
        columns
            .iter()
            .cartesian_product(rows.iter())
            .filter(|(r, c)| !r.has_neighbor(*self, graph) && !c.has_neighbor(*self, graph))
            .count()
    }

    fn count_interior_corners(&self, graph: &Self::Graph) -> usize {
        let columns = [Neighbor::North, Neighbor::South];
        let rows = [Neighbor::East, Neighbor::West];
        columns
            .iter()
            .cartesian_product(rows.iter())
            .filter(|(r, c)| {
                r.has_neighbor(*self, graph)
                    && c.has_neighbor(*self, graph)
                    && !graph.contains(&r.get_neighbor(c.get_neighbor(*self).unwrap()).unwrap())
            })
            .count()
    }
}

#[derive(Debug, EnumIter)]
pub enum Neighbor {
    East,
    South,
    West,
    North,
}

impl Neighbor {
    pub fn has_neighbor(&self, node: GardenPlot, neighborhood: &HashSet<GardenPlot>) -> bool {
        if let Some(neighbor) = self.get_neighbor(node) {
            neighborhood.contains(&neighbor)
        } else {
            false
        }
    }

    pub fn get_neighbor(&self, node: GardenPlot) -> Option<GardenPlot> {
        match self {
            Neighbor::East => Some((node.0, node.1 + 1, node.2)),
            Neighbor::South => Some((node.0 + 1, node.1, node.2)),
            Neighbor::West if node.1 > 0 => Some((node.0, node.1 - 1, node.2)),
            Neighbor::North if node.0 > 0 => Some((node.0 - 1, node.1, node.2)),
            _ => None,
        }
    }
}

pub fn count_corners(nodes: &HashSet<GardenPlot>) -> Option<Vec<(usize, usize)>> {
    if nodes.len() < 2 {
        return Some(vec![(nodes.len(), 4 * nodes.len())]);
    }
    let mut unevaluated: BTreeSet<GardenPlot> = nodes.iter().copied().collect();
    let mut queue: VecDeque<GardenPlot> = VecDeque::new();
    let mut values: Vec<(usize, usize)> = Vec::new();

    while let Some(node) = unevaluated.pop_first() {
        queue.push_back(node);
        let mut ncorners: usize = 0;
        let mut nnodes: usize = 0;

        // get the next adjacent plant of the same type
        while let Some(parent) = queue.pop_front() {
            nnodes += 1;
            ncorners += parent.count_exterior_corners(nodes);
            ncorners += parent.count_interior_corners(nodes);
            for neighborhood in Neighbor::iter() {
                if let Some(n) = neighborhood.get_neighbor(parent) {
                    // if the adjacent plant is of the same type
                    if parent.plant_type() == n.plant_type()
                        && nodes.contains(&n)
                        && unevaluated.contains(&n)
                    {
                        queue.push_back(n);
                        unevaluated.remove(&n);
                    }
                }
            }
        }
        // println!("{nnodes} * {ncorners} = {}", nnodes * ncorners);
        values.push((nnodes, nnodes * ncorners));
    }

    Some(values)
}

pub fn count_exposed_sides(nodes: &HashSet<GardenPlot>) -> Option<Vec<(usize, usize)>> {
    if nodes.len() < 2 {
        return Some(vec![(nodes.len(), 4 * nodes.len())]);
    }
    let mut unevaluated: BTreeSet<GardenPlot> = nodes.iter().copied().collect();
    let mut queue: VecDeque<GardenPlot> = VecDeque::new();
    let mut values: Vec<(usize, usize)> = Vec::new();

    // for each plant type
    while let Some(node) = unevaluated.pop_first() {
        queue.push_back(node);
        let mut nedges: usize = 0;
        let mut nnodes: usize = 0;

        // get the next adjacent plant of the same type
        while let Some(parent) = queue.pop_front() {
            nnodes += 1;
            for neighborhood in Neighbor::iter() {
                if let Some(n) = neighborhood.get_neighbor(parent) {
                    // if the adjacent plant is of the same type
                    if parent.plant_type() == n.plant_type() && nodes.contains(&n) {
                        nedges += 1;
                        if unevaluated.contains(&n) {
                            queue.push_back(n);
                            unevaluated.remove(&n);
                        }
                    }
                }
            }
        }
        values.push((nnodes, 4 * nnodes - nedges));
    }

    Some(values)
}

impl CommandImpl for Day12 {
    fn main(&self) -> Result<(), DynError> {
        let plant_string = fs::read_to_string(&self.input)?;
        let plant_array: Vec<Vec<char>> =
            plant_string.split_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let plants = plant_array
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (x, y, *v)))
            .collect::<HashSet<GardenPlot>>();
        if let Some(counts) = count_exposed_sides(&plants) {
            let answer: usize = counts.iter().map(|(x, y)| x * y).sum();
            println!("answer: {:?}", answer);
        }
        if let Some(counts) = count_corners(&plants) {
            let answer: usize = counts.iter().map(|(_, y)| y).sum();
            println!("answer: {:?}", answer);
        }
        Ok(())
    }
}
