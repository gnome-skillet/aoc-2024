use std::path::PathBuf;

use clap::Parser;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day24 {
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

pub type Initialization = (String, u64);
pub trait Initializable {
    fn initialize(&self, lookup_table: &mut HashMap<String, u64>);
}

impl Initializable for Initialization {
    fn initialize(&self, lookup_table: &mut HashMap<String, u64>) {
        lookup_table.insert(self.0.clone(), self.1);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LogicalStatement {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl LogicalStatement {
    fn new(opstring: &str, lhs: &str, rhs: &str) -> Option<Self> {
        match opstring {
            "AND" => Some(LogicalStatement::And(lhs.to_string(), rhs.to_string())),
            "OR" => Some(LogicalStatement::Or(lhs.to_string(), rhs.to_string())),
            "XOR" => Some(LogicalStatement::Xor(lhs.to_string(), rhs.to_string())),
            _ => None,
        }
    }

    fn lhs(&self) -> String {
        match self {
            LogicalStatement::And(lhs, _) => lhs.to_string(),
            LogicalStatement::Or(lhs, _) => lhs.to_string(),
            LogicalStatement::Xor(lhs, _) => lhs.to_string(),
        }
    }

    fn rhs(&self) -> String {
        match self {
            LogicalStatement::And(_, rhs) => rhs.to_string(),
            LogicalStatement::Or(_, rhs) => rhs.to_string(),
            LogicalStatement::Xor(_, rhs) => rhs.to_string(),
        }
    }
}

pub trait Evaluatable {
    fn calculate(&self, lookup_table: &mut HashMap<String, u64>) -> u64;
}

pub trait Calculable {
    fn calculable(&self, lookup_table: &mut HashMap<String, u64>) -> bool;
}

impl Evaluatable for LogicalStatement {
    fn calculate(&self, lookup_table: &mut HashMap<String, u64>) -> u64 {
        match self {
            LogicalStatement::And(lhs, rhs) => lookup_table[lhs] & lookup_table[rhs],
            LogicalStatement::Or(lhs, rhs) => lookup_table[lhs] | lookup_table[rhs],
            LogicalStatement::Xor(lhs, rhs) => lookup_table[lhs] ^ lookup_table[rhs],
        }
    }
}

impl Calculable for LogicalStatement {
    fn calculable(&self, lookup_table: &mut HashMap<String, u64>) -> bool {
        lookup_table.contains_key(&self.lhs()) && lookup_table.contains_key(&self.rhs())
    }
}

impl Calculable for Assignment {
    fn calculable(&self, lookup_table: &mut HashMap<String, u64>) -> bool {
        lookup_table.contains_key(&self.0.lhs()) && lookup_table.contains_key(&self.0.rhs())
    }
}

pub type Assignment = (LogicalStatement, String);
pub trait Assignable {
    type Statement;
    fn assign(statement: Self::Statement, wire: String, lookup_table: &mut HashMap<String, u64>);
}
impl Assignable for Assignment {
    type Statement = LogicalStatement;
    fn assign(statement: Self::Statement, wire: String, lookup_table: &mut HashMap<String, u64>) {
        let statement_value = statement.calculate(lookup_table);
        lookup_table.insert(wire, statement_value);
    }
}

fn parse_logical_statement(input: &str) -> IResult<&str, Option<LogicalStatement>> {
    let (input, lhs) = take_till(|c| c == ' ')(input)?;
    let (input, _) = space1(input)?;
    let (input, op) = alt((tag("AND"), tag("OR"), tag("XOR")))(input)?;
    let (input, _) = space1(input)?;
    let (input, rhs) = take_till(|c| c == ' ')(input)?;
    Ok((input, LogicalStatement::new(op, lhs, rhs)))
}

fn parse_assignment(input: &str) -> IResult<&str, Assignment> {
    let (input, statement) = parse_logical_statement(input)?;
    let (input, _) = space1(input)?;
    let (input, _op) = tag("->")(input)?;
    let (input, _) = space1(input)?;
    let (input, rhs) = take_till(|c| c == '\n')(input)?;
    Ok((input, (statement.unwrap(), rhs.to_string())))
}

fn parse_assignments(input: &str) -> IResult<&str, VecDeque<Assignment>> {
    let (input, assignments) = separated_list1(line_ending, parse_assignment)(input)?;
    let assignments: VecDeque<Assignment> = VecDeque::from(assignments);
    Ok((input, assignments))
}

fn parse_wire(input: &str) -> IResult<&str, Initialization> {
    let (input, wire) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = one_of("01")(input)?;
    let digit: u64 = value.to_digit(10).unwrap().into();
    Ok((input, (wire.into(), digit)))
}

fn parse_wires(input: &str) -> IResult<&str, Vec<Initialization>> {
    let (input, wires) = separated_list1(line_ending, parse_wire)(input)?;
    Ok((input, wires))
}

fn parse_challenge(input: &str) -> IResult<&str, (Vec<Initialization>, VecDeque<Assignment>)> {
    let (input, wires) = parse_wires(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, assignments) = parse_assignments(input)?;

    Ok((input, (wires, assignments)))
}

fn build_z_binary(lookup_table: &HashMap<String, u64>) -> u64 {
    let mut result: u64 = 0;
    for (key, value) in lookup_table.iter().filter(|(x, _)| x.starts_with("z")) {
        let tmp: u64 = key[1..].to_string().parse::<u64>().expect("not a number");
        result |= value << tmp;
    }
    result
}

impl CommandImpl for Day24 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let mut lookup_table: HashMap<String, u64> = HashMap::new();
        if let Ok((_, (wires, mut assignments))) = parse_challenge(&blob_string) {
            wires.into_iter().for_each(|x| x.initialize(&mut lookup_table));
            while !assignments.is_empty() {
                if let Some(assignment) = assignments.pop_front() {
                    if assignment.calculable(&mut lookup_table) {
                        let value: u64 = assignment.0.calculate(&mut lookup_table);
                        lookup_table.insert(assignment.1, value);
                    } else {
                        assignments.push_back((assignment.0, assignment.1));
                    }
                }
            }
            let zbinary: u64 = build_z_binary(&lookup_table);
            println!("zbinary = {zbinary}");
        } else {
            println!("unable to read {:?}", &self.input);
        }

        Ok(())
    }
}
