use std::path::PathBuf;

use clap::Parser;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::{
    character::complete::one_of,
    multi::{many0, many1},
};

use nom::character::complete::line_ending;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day25 {
    #[clap(long, short)]
    input: PathBuf,
}

pub type Schematic = Vec<Vec<char>>;
pub type Lock = Schematic;
pub type Key = Schematic;
pub type PinHeights = Vec<usize>;

pub trait Fittable {
    fn fits(&self, other: &Self) -> bool;
}

impl Fittable for PinHeights {
    fn fits(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if (self[i] + other[i]) > 5 {
                return false;
            }
        }
        true
    }
}

pub trait LockHeightable {
    fn lock_heights(&self) -> Option<Vec<usize>>;
}

pub trait KeyHeightable {
    fn key_heights(&self) -> Option<Vec<usize>>;
}

impl LockHeightable for Lock {
    fn lock_heights(&self) -> Option<Vec<usize>> {
        let mut heights: Vec<usize> = Vec::new();
        for (_, row) in self.iter().enumerate() {
            let mut index: usize = 0;
            for (_, val) in row.iter().enumerate() {
                if *val != '#' {
                    break;
                }
                index += 1;
            }
            if index == 0 {
                return None;
            }
            heights.push(index - 1);
        }
        Some(heights)
    }
}

impl KeyHeightable for Key {
    fn key_heights(&self) -> Option<Vec<usize>> {
        let mut heights: Vec<usize> = Vec::new();
        for (_, row) in self.iter().enumerate() {
            let mut index: usize = 0;
            for (_, val) in row.iter().enumerate() {
                if *val == '#' {
                    break;
                }
                index += 1;
            }
            if index == 0 {
                return None;
            }
            heights.push(5 + 1 - index);
        }
        Some(heights)
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

pub trait Lockable {
    fn is_lock(&self) -> bool;
}

pub trait Keyable {
    fn is_key(&self) -> bool;
}

impl Lockable for Schematic {
    fn is_lock(&self) -> bool {
        for row in self.iter() {
            if row[0] != '#' {
                return false;
            }
        }
        true
    }
}

impl Keyable for Schematic {
    fn is_key(&self) -> bool {
        for row in self.iter() {
            if row[0] != '.' {
                return false;
            }
        }
        true
    }
}

fn parse_five_pins(input: &str) -> IResult<&str, Vec<char>> {
    let (input, pins) = many1(one_of("#."))(input)?;
    Ok((input, pins))
}

fn parse_lock(input: &str) -> IResult<&str, Schematic> {
    let (input, locks) = separated_list1(line_ending, parse_five_pins)(input)?;
    Ok((input, transpose(locks)))
}

fn parse_locks(input: &str) -> IResult<&str, Vec<Schematic>> {
    let (input, locks) = separated_list1(many1(line_ending), parse_lock)(input)?;
    //let (input, locks) = newline(input)?;
    Ok((input, locks))
}

impl CommandImpl for Day25 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        if let Ok((_, mut schematics)) = parse_locks(&blob_string) {
            let key_lengths: Vec<Vec<usize>> = schematics
                .iter()
                .filter(|x| x.is_key())
                .map(|x| x.key_heights().unwrap())
                .collect();
            let lock_lengths: Vec<Vec<usize>> = schematics
                .iter()
                .filter(|x| x.is_lock())
                .map(|x| x.lock_heights().unwrap())
                .collect();
            let mut num_fits: usize = 0;
            for key in key_lengths.iter() {
                for lock in lock_lengths.iter() {
                    if key.fits(&lock) {
                        num_fits += 1;
                    }
                }
            }
            println!("n fits: {:?}", num_fits);
            //let locks: Vec<Lock> =
            //schematics.iter().filter(|&x| x.is_lock()).collect::<Vec<Lock>>();
            //println!("locks: {:?}", locks);
        } else {
            println!("unable to read {:?}", &self.input);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn test_first_lock() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....";
        let (_, lock) = parse_lock(input)?;
        let lock: Lock = lock;
        let observed = lock.lock_heights().unwrap();
        let expected = vec![0, 5, 3, 4, 3];
        assert_eq!(observed, expected);
        Ok(())
    }

    #[test]
    fn test_first_key() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = ".....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####";
        let (_, key) = parse_lock(input)?;
        assert!(key.is_key());
        let key: Key = key;
        let observed = key.key_heights().unwrap();
        let expected = vec![5, 0, 2, 1, 3];
        assert_eq!(observed, expected);
        Ok(())
    }

    #[test]
    fn test_second_lock() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "#####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....";

        let (_, lock) = parse_lock(input)?;
        assert!(lock.is_lock());
        let lock: Lock = lock;
        println!("lock: {:?}", lock);
        let observed = lock.lock_heights().unwrap();
        println!("observed: {:?}", observed);
        let expected = vec![1, 2, 0, 5, 3];
        assert_eq!(observed, expected);
        Ok(())
    }
}
