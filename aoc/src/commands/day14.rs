use std::path::PathBuf;

use clap::Parser;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::{
    character::complete::{char, one_of},
    combinator::{map_res, opt, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated},
};
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

use super::{CommandImpl, DynError};
use itertools::Itertools;

#[derive(Parser, Debug)]
pub struct Day14 {
    #[clap(long, short)]
    input: PathBuf,
}

const ROWS: i32 = 103;
const MIDROW: i32 = 51;
const COLUMNS: i32 = 101;
const MIDCOLUMN: i32 = 50;

#[derive(Debug, Clone, Hash)]
pub struct Grid {
    robots: Vec<Robot>,
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Robot {
    row: i32,
    column: i32,
    rowbar: i32,
    colbar: i32,
}

impl Robot {
    pub fn new(row: i32, column: i32, rowbar: i32, colbar: i32) -> Self {
        Self { row, column, rowbar, colbar }
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn column(&self) -> i32 {
        self.column
    }

    pub fn rowbar(&self) -> i32 {
        self.rowbar
    }

    pub fn colbar(&self) -> i32 {
        self.colbar
    }

    pub fn displace(&mut self) {
        let nrows: i32 = ROWS as i32;
        let ncols: i32 = COLUMNS as i32;
        self.row = (self.row + self.rowbar).rem_euclid(nrows);
        self.column = (self.column + self.colbar).rem_euclid(ncols);
    }
}

pub fn add_modulo(augend: usize, addend: usize, modulo: usize) -> usize {
    (augend + addend) % modulo
}

pub fn subtract_modulo(minuend: usize, subtrahend: usize, modulo: usize) -> usize {
    if minuend >= subtrahend {
        (minuend - subtrahend) % modulo
    } else {
        modulo - (minuend.abs_diff(subtrahend) % modulo)
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point {
    row: usize,
    column: usize,
}

fn parse_point(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("p=")(input)?;
    let (input, p) = separated_pair(parse_i32, tag(","), parse_i32)(input)?;
    Ok((input, p))
}

fn parse_velocity(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("v=")(input)?;
    let (input, p) = separated_pair(parse_i32, tag(","), parse_i32)(input)?;
    Ok((input, p))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, p) = parse_point(input)?;
    let (input, _) = space1(input)?;
    let (input, v) = parse_velocity(input)?;
    let r: Robot = Robot::new(p.1, p.0, v.1, v.0);
    Ok((input, r))
}

fn parse_robots(input: &str) -> IResult<&str, Vec<Robot>> {
    let (input, robots) = separated_list1(line_ending, parse_robot)(input)?;
    Ok((input, robots))
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.to_string().parse::<i32>()
    })(input)?;

    Ok((i, number))
}

fn my_digit(input: &str) -> IResult<&str, u64> {
    let (input, digits) = digit1(input)?;
    let x: u64 = digits.parse::<u64>().unwrap();
    Ok((input, x))
}

fn count_quadrant(q: usize, robots: &Vec<Robot>) -> usize {
    let mut n: usize = 0;
    let row_range = match q {
        1 | 3 => 0i32..MIDROW,
        2 | 4 => (MIDROW + 1)..ROWS,
        _ => panic!(),
    };
    let col_range = match q {
        1 | 2 => 0i32..MIDCOLUMN,
        3 | 4 => (MIDCOLUMN + 1)..COLUMNS,
        _ => panic!(),
    };
    for robot in robots.iter() {
        if row_range.contains(&robot.row()) && col_range.contains(&robot.column) {
            n += 1;
        }
    }
    n
}

pub fn show(robots: &Vec<Robot>) {
    for row in 0..ROWS {
        for col in 0..COLUMNS {
            let mut printed: bool = false;
            for robot in robots.iter() {
                if row == robot.row() && col == robot.column() {
                    printed = true;
                }
            }
            if !printed {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!("");
    }
}

impl CommandImpl for Day14 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, mut robots)) = parse_robots(&blob_string) else { todo!() };
        for i in 0..1000 {
            for robot in robots.iter_mut() {
                robot.displace();
            }

            if i > 500 && i < 600 {
                println!("loop {i}");
                show(&robots);
                print!("\x1B[2J");
            }
        }

        let mut nquad: [usize; 4] = [0; 4];
        for i in 0..4 {
            nquad[i] = count_quadrant(i + 1, &robots);
        }
        println!("nquad {:?}", nquad);
        let quadprod: usize = nquad.iter().product();
        println!("product {:?}", quadprod);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn test_parse_point() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "p=0,4";
        let (_, p) = parse_point(input)?;
        assert_eq!(p.0, 0i32);
        assert_eq!(p.1, 4i32);
        Ok(())
    }

    #[test]
    fn test_parse_point_negative() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "p=-22,-44";
        let (_, p) = parse_point(input)?;
        assert_eq!(p.0, -22i32);
        assert_eq!(p.1, -44i32);
        Ok(())
    }

    #[test]
    fn test_parse_robot() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "p=3,0 v=-1,-2";
        let (_, r) = parse_robot(input)?;
        assert_eq!(r.row(), 0usize);
        assert_eq!(r.column(), 3usize);
        assert_eq!(r.rowbar(), -2i32);
        assert_eq!(r.colbar(), -1i32);
        Ok(())
    }

    #[test]
    fn test_parse_robots() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "p=3,0 v=-1,-2\n\
        p=9,3 v=2,3";
        let (_, robots) = parse_robots(input)?;
        println!("{:?}", robots);

        assert_eq!(robots.len(), 2usize);
        assert_eq!(robots[0].row(), 0usize);
        assert_eq!(robots[0].column(), 3usize);
        assert_eq!(robots[0].rowbar(), -2i32);
        assert_eq!(robots[0].colbar(), -1i32);
        Ok(())
    }
}
