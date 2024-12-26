use std::path::PathBuf;

use clap::Parser;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use super::{CommandImpl, DynError};
use itertools::Itertools;

#[derive(Parser, Debug)]
pub struct Day12 {
    #[clap(long, short)]
    input: PathBuf,
}

type Node = (usize, usize, char);

pub trait GraphColorable {
    type Color;
    fn color(&self) -> Self::Color;
}

impl GraphColorable for Node {
    type Color = char;

    fn color(&self) -> Self::Color {
        self.2
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
    pub fn get_neighbor(&self, node: Node) -> Option<Node> {
        match self {
            Neighbor::East => Some((node.0, node.1 + 1, node.2)),
            Neighbor::South => Some((node.0 + 1, node.1, node.2)),
            Neighbor::West if node.1 > 0 => Some((node.0, node.1 - 1, node.2)),
            Neighbor::North if node.0 > 0 => Some((node.0 - 1, node.1, node.2)),
            _ => None,
        }
    }
}

pub fn connected_components(nodes: &[Node]) -> Option<Vec<(usize, usize)>> {
    if nodes.len() < 2 {
        return Some(vec![(nodes.len(), 4 * nodes.len())]);
    }
    let mut unevaluated: BTreeSet<Node> = nodes.iter().map(|&x| x).collect();
    let mut exists: HashSet<Node> = nodes.iter().map(|&x| x).collect();
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut values: Vec<(usize, usize)> = Vec::new();

    while let Some(node) = unevaluated.pop_first() {
        queue.push_back(node);
        unevaluated.remove(&node);
        let mut nedges: usize = 0;
        let mut nnodes: usize = 0;

        while let Some(parent) = queue.pop_front() {
            nnodes += 1;
            for neighbor in Neighbor::iter() {
                if let Some(n) = neighbor.get_neighbor(parent) {
                    if parent.color() == n.color() && exists.contains(&n) {
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
        let plantvec: Vec<Vec<char>> =
            plant_string.split_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let nodes = plantvec
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (x, y, *v)))
            .collect::<Vec<Node>>();
        if let Some(counts) = connected_components(&nodes) {
            //let mut answer: usize = 0;
            //for x in counts.iter() {
            //    answer += x.0 * x.1;
            //}
            let answer: usize = counts.iter().map(|(x, y)| x * y).sum();
            println!("answer: {:?}", answer);
        }
        Ok(())
    }
}
