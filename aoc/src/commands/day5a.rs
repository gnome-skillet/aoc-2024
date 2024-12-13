use std::path::PathBuf;

use clap::Parser;

use std::fs;

use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::newline;
use nom::combinator::{map_res, recognize};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day5a {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_rule(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, p) = separated_pair(digit1, tag("|"), digit1)(input)?;
    let integers = (p.0.to_string().parse::<u8>().unwrap(), p.1.to_string().parse::<u8>().unwrap());
    Ok((input, integers))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
    let (input, paired_rules) = separated_list1(newline, parse_rule)(input)?;
    Ok((input, paired_rules))
}

fn create_rule_map(paired_rules: &Vec<(u8, u8)>) -> HashMap<u8, HashSet<u8>> {
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

fn parse_problem(input: &str) -> IResult<&str, (Vec<(u8, u8)>, Vec<Vec<u8>>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, updates) = parse_updates(input)?;
    Ok((input, (rules, updates)))
}

fn parse_set_adapter(hashmap: &HashMap<u8, HashSet<u8>>) -> HashMap<u8, u128> {
    hashmap.into_iter().map(|(i, v)| (*i, hashset2bitmap(v))).collect::<HashMap<u8, u128>>()
}

fn hashset2bitmap(hashset: &HashSet<u8>) -> u128 {
    let mut bitmap: u128 = 0;
    for x in hashset.iter() {
        bitmap |= 1u128 << x;
    }
    bitmap
}

pub struct SafetyManual {
    rules: Vec<(u8, u8)>,
}

impl SafetyManual {
    pub fn new(rules: Vec<(u8, u8)>) -> Self {
        Self { rules }
    }

    fn flag_sorted(&self, updates: &Vec<Vec<u8>>) -> Vec<bool> {
        let rule_map = create_rule_map(&self.rules);
        updates.iter().map(|u| Self::is_update_ordered(&rule_map, u)).collect::<Vec<bool>>()
    }

    fn is_update_ordered(rules: &HashMap<u8, HashSet<u8>>, updates: &Vec<u8>) -> bool {
        let mut printed = HashSet::new();
        for page in updates {
            if let Some(descendent) = rules.get(&page) {
                let ncommon = descendent.intersection(&printed).count();
                if ncommon > 0 {
                    return false;
                }
            }
            printed.insert(*page);
        }
        return true;
    }

    pub fn reorder_updates(&mut self, updates: &mut Vec<Vec<u8>>) {
        for u in updates.iter_mut() {
            self.pair_sort(u);
        }
    }

    fn pair_sort(&mut self, updates: &mut Vec<u8>) {
        loop {
            let mut swapped = false;

            for &(first, second) in self.rules.iter() {
                for i in 0..updates.len() {
                    if updates[i] == second {
                        for j in i..updates.len() {
                            if updates[j] == first {
                                updates.swap(i, j);
                                swapped = true;
                                continue;
                            }
                        }
                    }
                }
            }

            if !swapped {
                break;
            }
        }
    }

    fn hash_sort(&mut self, updates: &mut Vec<u8>) {
        loop {
            let mut swapped = false;

            for &(first, second) in self.rules.iter() {
                for i in 0..updates.len() {
                    if updates[i] == second {
                        for j in i..updates.len() {
                            if updates[j] == first {
                                updates.swap(i, j);
                                swapped = true;
                                continue;
                            }
                        }
                    }
                }
            }

            if !swapped {
                break;
            }
        }
    }

    pub fn middle_values(&self, updates: &Vec<Vec<u8>>) -> Vec<u64> {
        updates.iter().map(|x| x[(x.len() - 1) / 2usize] as u64).collect::<Vec<u64>>()
    }

    pub fn nrules(&self) -> usize {
        self.rules.len()
    }

    pub fn middle_value_sum(&self, updates: &Vec<Vec<u8>>) -> u64 {
        self.middle_values(updates).iter().enumerate().map(|(_, x)| *x).sum()
    }

    pub fn filter_ordered(&self, updates: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let ordered = self.flag_sorted(updates);
        updates.iter().enumerate().filter(|(i, _)| ordered[*i]).map(|(_, x)| x.clone()).collect()
    }

    pub fn filter_unordered(&self, updates: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let ordered = self.flag_sorted(updates);
        updates.iter().enumerate().filter(|(i, _)| !ordered[*i]).map(|(_, x)| x.clone()).collect()
    }
}

impl CommandImpl for Day5a {
    fn main(&self) -> Result<(), DynError> {
        let file = fs::read_to_string(&self.input)?;
        if let Ok((_, (rules, mut updates))) = parse_problem(&file) {
            let mut safety_manual = SafetyManual::new(rules);
            let mut updates = safety_manual.filter_unordered(&mut updates);
            let _flag_ordered_updates = safety_manual.flag_sorted(&updates);
            safety_manual.reorder_updates(&mut updates);
            let sum_middle_values = safety_manual.middle_value_sum(&updates);
            println!("sum of middle values: {sum_middle_values}");
        } else {
            println!("error");
        }
        Ok(())
    }
}
