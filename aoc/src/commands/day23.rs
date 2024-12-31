use std::path::PathBuf;

use clap::Parser;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fs;

use std::collections::HashSet;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day23 {
    #[clap(long, short)]
    input: PathBuf,
}

type Node = String;
type Vertex = (Node, Node);
type VertexGraph = HashSet<Vertex>;

pub trait Unwrappable {
    type Item;
    fn unwrap(&self) -> Vec<Self::Item>;
}

pub trait Containable {
    type Item;
    fn contains(&self, x: Self::Item) -> bool;
}

pub trait Swappable {
    fn swap(self) -> Self;
}

impl Swappable for Vertex {
    fn swap(self) -> Self {
        if self.0 < self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }
}

impl Containable for VertexGraph {
    type Item = Vertex;
    fn contains(&self, x: Self::Item) -> bool {
        self.contains(&x)
    }
}

impl Unwrappable for Vertex {
    type Item = Node;

    fn unwrap(&self) -> Vec<Node> {
        vec![self.0.clone(), self.1.clone()]
    }
}

fn parse_connection(input: &str) -> IResult<&str, Vertex> {
    let (input, (lhs, rhs)) = separated_pair(alpha1, tag("-"), alpha1)(input)?;
    let (lhs, rhs) = if lhs < rhs { (lhs, rhs) } else { (rhs, lhs) };
    Ok((input, (lhs.into(), rhs.into())))
}

fn parse_connections(input: &str) -> IResult<&str, VertexGraph> {
    let (input, mut tuples) = separated_list1(line_ending, parse_connection)(input)?;
    tuples.sort();
    let connections: HashSet<Vertex> = HashSet::from_iter(tuples);
    Ok((input, connections))
}

impl CommandImpl for Day23 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        if let Ok((_, vertices)) = parse_connections(&blob_string) {
            let commutitive = |x: Node, y: Node, z: Node| -> bool {
                vertices.contains(&(x.clone(), y.clone()).swap())
                    && vertices.contains(&(x.clone(), z.clone()).swap())
                    && vertices.contains(&(y.clone(), z.clone()).swap())
            };
            let nodes = vertices.iter().flat_map(|x| x.unwrap()).collect::<HashSet<Node>>();
            let ntriplets: usize = nodes
                .iter()
                .combinations(3)
                .filter(|x| x.iter().any(|s| s.starts_with("t")))
                .filter(|x| commutitive(x[0].to_string(), x[1].to_string(), x[2].to_string()))
                .count();

            println!("triplets {:?}", ntriplets);
        } else {
            println!("unable to read {:?}", &self.input);
        }

        Ok(())
    }
}
