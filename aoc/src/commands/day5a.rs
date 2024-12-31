use std::path::PathBuf;

use clap::Parser;

use std::fs;

use std::collections::{HashMap, HashSet};

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::newline;
use nom::combinator::{map_res, recognize};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use super::{CommandImpl, DynError};

pub type Rule = (u8, u8);
#[derive(Parser, Debug)]
pub struct Day5a {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, p) = separated_pair(digit1, tag("|"), digit1)(input)?;
    let integers = (p.0.to_string().parse::<u8>().unwrap(), p.1.to_string().parse::<u8>().unwrap());
    Ok((input, integers))
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u8, HashSet<u8>>> {
    let (input, paired_rules) = separated_list1(newline, parse_rule)(input)?;
    let rule_map = create_rule_map(&paired_rules);
    Ok((input, rule_map))
}

fn create_rule_map(paired_rules: &[Rule]) -> HashMap<u8, HashSet<u8>> {
    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    for rule in paired_rules.iter() {
        rules.entry(rule.0).or_default().insert(rule.1);
    }
    rules
}

fn my_u8(input: &str) -> IResult<&str, u8> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, update) = separated_list1(tag(","), my_u8)(input)?;
    Ok((input, update))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (input, updates) = separated_list1(line_ending, parse_update)(input)?;
    Ok((input, updates))
}

fn parse_problem(input: &str) -> IResult<&str, (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, updates) = parse_updates(input)?;
    Ok((input, (rules, updates)))
}

pub struct SafetyManual {
    rules: HashMap<u8, HashSet<u8>>,
}

impl SafetyManual {
    pub fn new(rules: HashMap<u8, HashSet<u8>>) -> Self {
        Self { rules }
    }

    fn is_update_ordered(&self, updates: &Vec<u8>) -> bool {
        let mut printed = HashSet::new();
        for page in updates {
            if let Some(descendent) = self.rules.get(page) {
                let ncommon = descendent.intersection(&printed).count();
                if ncommon > 0 {
                    return false;
                }
            }
            printed.insert(*page);
        }
        true
    }

    fn pair_sort(&self, updates: &mut [u8]) {
        loop {
            let mut swapped = false;

            for i in 0..(updates.len() - 1) {
                for j in i..updates.len() {
                    if let Some(x) = self.rules.get(&updates[j]) {
                        if x.contains(&updates[i]) {
                            updates.swap(i, j);
                            swapped = true;
                            continue;
                        }
                    }
                }
            }

            if !swapped {
                break;
            }
        }
    }

    pub fn middle_values(&self, updates: &[Vec<u8>]) -> Vec<u64> {
        updates.iter().map(|x| x[(x.len() - 1) / 2usize] as u64).collect::<Vec<u64>>()
    }

    pub fn nrules(&self) -> usize {
        self.rules.len()
    }

    pub fn middle_value_sum(&self, updates: &[Vec<u8>]) -> u64 {
        self.middle_values(updates).iter().sum()
    }

    pub fn filter_unordered(&self, updates: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        updates.retain(|x| self.is_update_ordered(x));
        updates.to_vec()
    }

    pub fn filter_ordered(&self, updates: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        updates.retain(|x| !self.is_update_ordered(x));
        updates.to_vec()
    }
}

impl CommandImpl for Day5a {
    fn main(&self) -> Result<(), DynError> {
        let file = fs::read_to_string(&self.input)?;
        if let Ok((_, (rules, mut updates))) = parse_problem(&file) {
            let safety_manual = SafetyManual::new(rules);
            let mut updates = safety_manual.filter_ordered(&mut updates);
            updates.iter_mut().for_each(|x| safety_manual.pair_sort(x));
            let sum_middle_values = safety_manual.middle_value_sum(&updates);
            println!("sum of middle values: {sum_middle_values}");
        } else {
            println!("error");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_point() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
        let (_, rules) = parse_rules(input)?;
        let updates: Vec<u8> = vec![75, 47, 61, 53, 29];
        let safety_manual = SafetyManual::new(rules);
        assert!(safety_manual.is_update_ordered(&updates));
        let updates: Vec<u8> = vec![97, 61, 53, 29, 13];
        assert!(safety_manual.is_update_ordered(&updates));
        let updates: Vec<u8> = vec![75, 29, 13];
        assert!(safety_manual.is_update_ordered(&updates));
        let updates: Vec<u8> = vec![75, 97, 47, 61, 53];
        assert!(!safety_manual.is_update_ordered(&updates));
        let updates: Vec<u8> = vec![61, 13, 29];
        assert!(!safety_manual.is_update_ordered(&updates));
        let updates: Vec<u8> = vec![97, 13, 75, 29, 47];
        assert!(!safety_manual.is_update_ordered(&updates));
        let mut updates: Vec<u8> = vec![75, 97, 47, 61, 53];
        safety_manual.pair_sort(&mut updates);
        let expected: Vec<u8> = vec![97, 75, 47, 61, 53];
        assert_eq!(updates, expected);
        Ok(())
    }
}
