use std::ops;
use std::path::PathBuf;

use clap::Parser;

use env_logger;
use log::debug;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day13 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct Button {
    row: usize,
    column: usize,
}

impl Button {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

type Prize = (usize, usize);

impl ops::Add<Button> for Button {
    type Output = Button;

    fn add(self, rhs: Button) -> Button {
        Self::new(self.row + rhs.row, self.column + rhs.column)
    }
}

impl ops::Mul<usize> for Button {
    type Output = Button;

    fn mul(self, rhs: usize) -> Button {
        Self::new(self.row * rhs, self.column * rhs)
    }
}

pub trait Indexable {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
}

impl Indexable for Button {
    fn row(&self) -> usize {
        self.row
    }

    fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct MachineConfiguration {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl MachineConfiguration {
    pub fn new(button_a: Button, button_b: Button, prize: Prize) -> Self {
        Self { button_a, button_b, prize }
    }
}

fn my_digit(input: &str) -> IResult<&str, usize> {
    let (input, digits) = digit1(input)?;
    let x: usize = digits.parse::<usize>().unwrap();
    Ok((input, x))
}

fn parse_x(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("X+")(input)?;
    let (input, x) = my_digit(input)?;
    Ok((input, x))
}

fn parse_y(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Y+")(input)?;
    let (input, x) = my_digit(input)?;
    Ok((input, x))
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    let (input, _) = tag("Button")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = terminated(one_of("AB"), tag(":"))(input)?;
    let (input, _) = space1(input)?;
    let (input, button) = separated_pair(parse_x, tag(", "), parse_y)(input)?;
    Ok((input, Button::new(button.0, button.1)))
}

fn parse_buttons(input: &str) -> IResult<&str, (Button, Button)> {
    let (input, buttons) = separated_pair(parse_button, line_ending, parse_button)(input)?;
    Ok((input, buttons))
}

fn parse_prize(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("Prize:")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("X=")(input)?;
    let (input, x) = my_digit(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, _) = tag("Y=")(input)?;
    let (input, y) = my_digit(input)?;
    Ok((input, (x, y)))
}

#[derive(Debug, Copy, Clone)]
pub struct Equation {
    eqn: (usize, usize, usize),
}

impl Equation {
    pub fn new(eqn: (usize, usize, usize)) -> Self {
        Self { eqn }
    }

    fn lcm(&self, other: &Equation) -> usize {
        lcm(self.eqn.0, other.eqn.0)
    }

    fn part2(&self, value: usize) -> Self {
        let eqn: (usize, usize, usize) = (self.eqn.0, self.eqn.1, self.eqn.2 + value);
        Self { eqn }
    }
}

impl ops::Add<Equation> for Equation {
    type Output = Equation;

    fn add(self, rhs: Self) -> Self {
        Self::new((self.eqn.0 + rhs.eqn.0, self.eqn.1 + rhs.eqn.1, self.eqn.2 + rhs.eqn.2))
    }
}

impl ops::Sub<Equation> for Equation {
    type Output = Option<Equation>;

    fn sub(self, rhs: Self) -> Option<Self> {
        if self.eqn.0 < rhs.eqn.0 || self.eqn.1 < rhs.eqn.1 || self.eqn.2 < rhs.eqn.2 {
            return None;
        }
        Some(Self::new((self.eqn.0 - rhs.eqn.0, self.eqn.1 - rhs.eqn.1, self.eqn.2 - rhs.eqn.2)))
    }
}

impl ops::Mul<usize> for Equation {
    type Output = Equation;

    fn mul(self, rhs: usize) -> Equation {
        Self::new((self.eqn.0 * rhs, self.eqn.1 * rhs, self.eqn.2 * rhs))
    }
}

#[derive(Debug)]
pub struct EquationSolver {
    eqn1: Equation,
    eqn2: Equation,
}

impl EquationSolver {
    pub fn new(eqn1: Equation, eqn2: Equation) -> Self {
        Self { eqn1, eqn2 }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.eqn1, &mut self.eqn2);
    }

    pub fn lcm(&self) -> usize {
        self.eqn1.lcm(&self.eqn2)
    }

    pub fn solve(&mut self) -> Option<(usize, usize)> {
        let lcm: usize = self.lcm();
        let mut eqn1: Equation = self.eqn1;
        let mut eqn2: Equation = self.eqn2;

        let fix: usize = lcm / eqn1.eqn.0;
        eqn1 = eqn1 * fix;
        let fix: usize = lcm / eqn2.eqn.0;
        eqn2 = eqn2 * fix;
        if eqn2.eqn.1 > eqn1.eqn.1 {
            std::mem::swap(&mut eqn1, &mut eqn2);
        }
        if let Some(eqn) = eqn1 - eqn2 {
            eqn1 = eqn;
        }
        let b: usize = eqn1.eqn.2 / eqn1.eqn.1;
        if eqn2.eqn.2 < b * eqn2.eqn.1 {
            return None;
        }
        let a: usize = (eqn2.eqn.2 - b * eqn2.eqn.1) / eqn2.eqn.0;
        if (self.eqn1.eqn.0 * a + self.eqn1.eqn.1 * b == self.eqn1.eqn.2)
            && (self.eqn2.eqn.0 * a + self.eqn2.eqn.1 * b == self.eqn2.eqn.2)
        {
            println!("a: {a}, b: {b}");
            return Some((a, b));
        }

        None
    }

    fn part2(&mut self) {
        self.eqn1 = self.eqn1.part2(10000000000000usize);
        self.eqn2 = self.eqn2.part2(10000000000000usize);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut x: usize = a;
    let mut y: usize = b;
    while y != 0 {
        let t: usize = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

fn parse_command(input: &str) -> IResult<&str, EquationSolver> {
    let (input, buttons) = parse_buttons(input)?;
    let (input, _) = line_ending(input)?;
    let (input, prize) = parse_prize(input)?;

    let eqn1: (usize, usize, usize) = (buttons.0.row, buttons.1.row, prize.0);
    let eqn2: (usize, usize, usize) = (buttons.0.column, buttons.1.column, prize.1);
    let eqn1: Equation = Equation::new(eqn1);
    let eqn2: Equation = Equation::new(eqn2);
    Ok((input, EquationSolver::new(eqn1, eqn2)))
}

fn parse_challenge(input: &str) -> IResult<&str, Vec<EquationSolver>> {
    let (input, commands) = separated_list1(many1(line_ending), parse_command)(input)?;
    Ok((input, commands))
}

impl CommandImpl for Day13 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        let blob_string = fs::read_to_string(&self.input)?;
        let mut tokens: usize = 0;
        match parse_challenge(&blob_string) {
            Ok((_, mut equations)) => {
                for eqn in equations.iter_mut() {
                    eqn.part2();
                    if let Some(soln) = eqn.solve() {
                        tokens += 3 * soln.0 + soln.1;
                        debug!("solution for {:?} = {:?}", eqn, soln);
                    }
                }
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
        println!("tokens: {tokens}");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_x() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "X+94";
        let (_, number) = parse_x(input)?;
        assert_eq!(number, 94usize);
        Ok(())
    }

    #[test]
    fn test_parse_y() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Y+89";
        let (_, number) = parse_y(input)?;
        assert_eq!(number, 89usize);
        Ok(())
    }

    #[test]
    fn test_parse_button() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Button A: X+94, Y+34";
        let (_, button) = parse_button(input)?;
        let expected: Button = Button::new(94usize, 34usize);
        assert_eq!(button, expected);
        Ok(())
    }

    #[test]
    fn test_parse_buttons() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Button A: X+94, Y+34\nButton B: X+89, Y+23";
        let (_, buttons) = parse_buttons(input)?;
        let expected_a: Button = Button::new(94usize, 34usize);
        let expected_b: Button = Button::new(89usize, 23usize);
        assert_eq!(buttons.0, expected_a);
        assert_eq!(buttons.1, expected_b);
        Ok(())
    }

    #[test]
    fn test_prize() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Prize: X=8400, Y=5400";
        let (_, prize) = parse_prize(input)?;
        assert_eq!(prize.0, 8400usize);
        assert_eq!(prize.1, 5400usize);
        Ok(())
    }
}
