use std::path::PathBuf;

use clap::Parser;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;
use regex::Regex;
use std::fs;

use std::collections::HashSet;
use std::collections::VecDeque;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day19 {
    #[clap(long, short)]
    input: PathBuf,
}

pub enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Stripe {
    pub fn new(patterns: &str) -> Vec<Stripe> {
        todo!()
    }
}

pub type Towel = Vec<Stripe>;

fn parse_available_towel_patterns(input: &str) -> IResult<&str, HashSet<&str>> {
    let (input, patterns) = separated_list1(tag(", "), alpha1)(input)?;
    let patterns: HashSet<&str> = HashSet::from_iter(patterns.iter().map(|&x| x));
    Ok((input, patterns))
}

fn parse_towel_designs(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, patterns) = separated_list1(line_ending, alpha1)(input)?;
    Ok((input, patterns))
}

fn parse_challenge(input: &str) -> IResult<&str, (HashSet<&str>, Vec<&str>)> {
    let (input, available_patterns) = parse_available_towel_patterns(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, displayed_patterns) = parse_towel_designs(input)?;
    Ok((input, (available_patterns, displayed_patterns)))
}

#[derive(Debug)]
pub struct PatternBuilder<'a> {
    patterns: HashSet<&'a str>,
}

pub trait Designable<'a> {
    fn designable(&self, design: &'a str) -> bool;
}

impl<'a> PatternBuilder<'a> {
    fn new(patterns: HashSet<&'a str>) -> Self {
        Self { patterns }
    }
}

impl<'a> Designable<'a> for PatternBuilder<'a> {
    fn designable(&self, design: &'a str) -> bool {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let string = design.to_string();
        let mut memoizer: HashSet<usize> = HashSet::new();
        queue.push_back(0usize);
        while let Some(offset) = queue.pop_front() {
            if offset == string.len() {
                return true;
            }
            if offset > string.len() {
                continue;
            }
            for prefix in self.patterns.iter() {
                if string[offset..].starts_with(prefix) {
                    let new_offset: usize = offset + prefix.len();
                    if !memoizer.contains(&new_offset) && new_offset <= string.len() {
                        queue.push_back(offset + prefix.len());
                    }
                }
            }
        }
        false
    }
}

impl CommandImpl for Day19 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let re = Regex::new(r"^(wb)?[rugw][rugw]+$").unwrap();

        if let Ok((_, (mut patterns, designs))) = parse_challenge(&blob_string) {
            let smaller_patterns: HashSet<&str> =
                HashSet::from_iter(patterns.iter().filter(|&x| x.len() <= 2).map(|&x| x));
            let pattern_builder = PatternBuilder::new(smaller_patterns);
            patterns.retain(|&x| x.len() <= 2 || !pattern_builder.designable(x));

            println!("there are {:?} patterns", patterns.len());
            patterns.retain(|&x| !re.is_match(x));
            println!("shrunk to {:?} patterns", patterns.len());
            println!("shrunk to {:?}", patterns);
            let pattern_builder = PatternBuilder::new(patterns);
            let mut ndesignable: usize = 0;
            for design in designs.iter() {
                if pattern_builder.designable(design) {
                    ndesignable += 1;
                }
            }
            println!("there are {ndesignable} designs");
        } else {
            println!("unable to read {:?}", &self.input);
        }

        Ok(())
    }
}
