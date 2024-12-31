use std::path::PathBuf;

use clap::Parser;
use std::collections::VecDeque;

use env_logger;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day21 {
    #[clap(long, short)]
    input: PathBuf,
}

pub trait Positional {
    type Position;
    fn position(&self) -> Self::Position;
}

impl DirectionalKey {
    fn move_steps(&self) -> (i32, i32) {
        match self {
            DirectionalKey::Activate => (0i32, 0i32),
            DirectionalKey::Right => (0i32, 1i32),
            DirectionalKey::Down => (-1i32, 0i32),
            DirectionalKey::Left => (0i32, -1i32),
            DirectionalKey::Up => (1i32, 0i32),
            DirectionalKey::Gap => panic!(),
        }
    }

    fn move_arm(&mut self, other: &DirectionalKey) -> DirectionalKey {
        let steps = other.move_steps();
        let position = self.position();
        (position.0 + steps.0, position.1 + steps.1).into()
    }
}

impl From<KeyLocation> for DirectionalKey {
    fn from(p: KeyLocation) -> DirectionalKey {
        match p {
            (0i32, 0i32) => DirectionalKey::Left,
            (0i32, 1i32) => DirectionalKey::Down,
            (0i32, 2i32) => DirectionalKey::Right,
            (1i32, 2i32) => DirectionalKey::Activate,
            (1i32, 1i32) => DirectionalKey::Up,
            _ => panic!(),
        }
    }
}

pub trait Distanceable {
    fn relative_distance(&self, other: &Self) -> (Direction, Direction);
    fn vertical(&self, other: &Self) -> Direction;
    fn horizontal(&self, other: &Self) -> Direction;
}

pub trait Directional {
    fn direction(&self) -> (Direction, Direction);
}

pub trait Traversable {
    // define a safe traversal between nodes
    fn traverse(&self, other: &Self) -> (Direction, Direction);
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Right(i32),
    Down(i32),
    Left(i32),
    Up(i32),
}

impl Distanceable for DirectionalKey {
    fn relative_distance(&self, other: &Self) -> (Direction, Direction) {
        (self.vertical(other), self.horizontal(other))
    }

    fn vertical(&self, other: &Self) -> Direction {
        let this = self.position();
        let that = other.position();
        if this.0 > that.0 {
            Direction::Down(this.0.abs_diff(that.0).try_into().unwrap())
        } else {
            Direction::Up(this.0.abs_diff(that.0).try_into().unwrap())
        }
    }

    fn horizontal(&self, other: &Self) -> Direction {
        let this = self.position();
        let that = other.position();
        if this.1 > that.1 {
            Direction::Left(this.1.abs_diff(that.1).try_into().unwrap())
        } else {
            Direction::Right(this.1.abs_diff(that.1).try_into().unwrap())
        }
    }
}

impl Traversable for DirectionalKey {
    fn traverse(&self, other: &Self) -> (Direction, Direction) {
        let distance = self.relative_distance(other);
        let curr = self.position();
        let gap = DirectionalKey::Gap.position();

        if curr.0 == gap.0 {
            (distance.1, distance.0)
        } else {
            distance
        }
    }
}

impl Traversable for NumericalKey {
    fn traverse(&self, other: &Self) -> (Direction, Direction) {
        let distance = self.relative_distance(other);
        let curr = self.position();
        let gap = DirectionalKey::Gap.position();

        if curr.0 == gap.0 {
            (distance.1, distance.0)
        } else {
            distance
        }
    }
}

fn translate(d: Direction) -> String {
    let (c, n): (char, i32) = match d {
        Direction::Right(n) => ('>', n),
        Direction::Down(n) => ('v', n),
        Direction::Left(n) => ('<', n),
        Direction::Up(n) => ('^', n),
    };
    (0..n).map(|_| c).collect::<String>()
}

impl Distanceable for NumericalKey {
    fn relative_distance(&self, other: &Self) -> (Direction, Direction) {
        (self.vertical(other), self.horizontal(other))
    }

    fn vertical(&self, other: &Self) -> Direction {
        let this = self.position();
        let that = other.position();
        if this.0 > that.0 {
            Direction::Down(this.0.abs_diff(that.0).try_into().unwrap())
        } else {
            Direction::Up(this.0.abs_diff(that.0).try_into().unwrap())
        }
    }

    fn horizontal(&self, other: &Self) -> Direction {
        let this = self.position();
        let that = other.position();
        if this.1 > that.1 {
            Direction::Left(this.1.abs_diff(that.1).try_into().unwrap())
        } else {
            Direction::Right(this.1.abs_diff(that.1).try_into().unwrap())
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum DirectionalKey {
    #[default]
    Activate,
    Up,
    Gap,
    Right,
    Down,
    Left,
}

impl From<char> for DirectionalKey {
    fn from(c: char) -> Self {
        match c {
            'A' => DirectionalKey::Activate,
            '^' => DirectionalKey::Up,
            '>' => DirectionalKey::Right,
            'v' => DirectionalKey::Down,
            '<' => DirectionalKey::Left,
            _ => panic!(),
        }
    }
}

impl From<char> for NumericalKey {
    fn from(c: char) -> Self {
        match c {
            'A' => NumericalKey::Activate,
            '0' => NumericalKey::Zero,
            '1' => NumericalKey::One,
            '2' => NumericalKey::Two,
            '3' => NumericalKey::Three,
            '4' => NumericalKey::Four,
            '5' => NumericalKey::Five,
            '6' => NumericalKey::Six,
            '7' => NumericalKey::Seven,
            '8' => NumericalKey::Eight,
            '9' => NumericalKey::Nine,
            _ => panic!(),
        }
    }
}

pub trait Avoidable {
    type Position;
    fn avoided_position(&self) -> Self::Position;
}

impl Avoidable for DirectionalKey {
    type Position = (i32, i32);
    fn avoided_position(&self) -> Self::Position {
        DirectionalKey::Gap.position()
    }
}

impl Avoidable for NumericalKey {
    type Position = (i32, i32);
    fn avoided_position(&self) -> Self::Position {
        NumericalKey::Gap.position()
    }
}

impl Positional for DirectionalKey {
    type Position = (i32, i32);

    fn position(&self) -> Self::Position {
        match *self {
            DirectionalKey::Activate => (1i32, 2i32),
            DirectionalKey::Up => (1i32, 1i32),
            DirectionalKey::Gap => (1i32, 0i32),
            DirectionalKey::Right => (0i32, 2i32),
            DirectionalKey::Down => (0i32, 1i32),
            DirectionalKey::Left => (0i32, 0i32),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum NumericalKey {
    Gap,
    Zero,
    #[default]
    Activate,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

type KeyLocation = (i32, i32);
impl Positional for NumericalKey {
    type Position = KeyLocation;
    fn position(&self) -> Self::Position {
        match *self {
            NumericalKey::Gap => (0i32, 0i32),
            NumericalKey::Zero => (0i32, 1i32),
            NumericalKey::Activate => (0i32, 2i32),
            NumericalKey::One => (1i32, 0i32),
            NumericalKey::Two => (1i32, 1i32),
            NumericalKey::Three => (1i32, 2i32),
            NumericalKey::Four => (2i32, 0i32),
            NumericalKey::Five => (2i32, 1i32),
            NumericalKey::Six => (2i32, 2i32),
            NumericalKey::Seven => (3i32, 0i32),
            NumericalKey::Eight => (3i32, 1i32),
            NumericalKey::Nine => (3i32, 2i32),
        }
    }
}

impl From<KeyLocation> for NumericalKey {
    fn from(p: KeyLocation) -> NumericalKey {
        match p {
            (0, 0) => NumericalKey::Gap,
            (0, 1) => NumericalKey::Zero,
            (0, 2) => NumericalKey::Activate,
            (1, 0) => NumericalKey::One,
            (1, 1) => NumericalKey::Two,
            (1, 2) => NumericalKey::Three,
            (2, 0) => NumericalKey::Four,
            (2, 1) => NumericalKey::Five,
            (2, 2) => NumericalKey::Six,
            (3, 0) => NumericalKey::Seven,
            (3, 1) => NumericalKey::Eight,
            (3, 2) => NumericalKey::Nine,
            _ => panic!(),
        }
    }
}

type NumericalCode = Vec<NumericalKey>;

impl NumericalKey {
    fn numeric(code: &NumericalCode) -> Option<i32> {
        let numbers: Vec<i32> = code.iter().filter_map(|&c| c.try_into().ok()).collect();
        numbers
            .iter()
            .scan(0i32, |acc, &x| {
                *acc = 10 * *acc + x;
                Some(*acc)
            })
            .last()
    }
}

impl TryFrom<NumericalKey> for i32 {
    type Error = ();

    fn try_from(key: NumericalKey) -> Result<Self, Self::Error> {
        match key {
            NumericalKey::Zero => Ok(0),
            NumericalKey::One => Ok(1),
            NumericalKey::Two => Ok(2),
            NumericalKey::Three => Ok(3),
            NumericalKey::Four => Ok(4),
            NumericalKey::Five => Ok(5),
            NumericalKey::Six => Ok(6),
            NumericalKey::Seven => Ok(7),
            NumericalKey::Eight => Ok(8),
            NumericalKey::Nine => Ok(9),
            _ => Err(()),
        }
    }
}

fn parse_command(input: &str) -> IResult<&str, Vec<NumericalKey>> {
    let (input, code) = alphanumeric1(input)?;
    let mut code: VecDeque<NumericalKey> = code.chars().map(|c| c.into()).collect();
    code.push_front(NumericalKey::Activate);
    let vec: Vec<NumericalKey> = code.into();

    Ok((input, vec))
}

fn parse_challenge(input: &str) -> IResult<&str, Vec<Vec<NumericalKey>>> {
    let (input, codes) = separated_list1(many1(line_ending), parse_command)(input)?;
    Ok((input, codes))
}

fn decode<D: Traversable>(code: &[D]) -> String {
    let mut s: String = "A".to_string();
    for commands in code.windows(2) {
        let d: (Direction, Direction) = commands[0].traverse(&commands[1]);
        s.push_str(&translate(d.1).to_owned());
        s.push_str(&translate(d.0).to_owned());
        s.push_str("A");
    }
    s
}
//impl NumericalKey {
//    fn encode(&self, code: String) -> String {
//        let characters: Vec<char> = numerical_instructions.chars().collect();
//        let mut s: String = "".to_string();
//        // initialize arm to default position
//        let mut curr = D::default();
//
//        for commands in code.chars() {
//            let d: (Direction, Direction) = curr.traverse(&commands);
//            s.push_str(&translate(d.1).to_owned());
//            s.push_str(&translate(d.0).to_owned());
//            s.push_str(&"A".to_owned());
//        }
//        s
//    }
//}

impl CommandImpl for Day21 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        let blob_string = fs::read_to_string(&self.input)?;
        let mut part1_answer: i32 = 0;
        match parse_challenge(&blob_string) {
            Ok((_, codes)) => {
                for code in codes.iter() {
                    let numerical_instructions: String = decode(code);
                    let ch: Vec<char> = numerical_instructions.chars().collect();
                    let directions: Vec<DirectionalKey> =
                        ch.iter().map(|&c| DirectionalKey::from(c)).collect();
                    let robot_2: String = decode(&directions);
                    let ch: Vec<char> = robot_2.chars().collect();
                    let directions: Vec<DirectionalKey> =
                        ch.iter().map(|&c| DirectionalKey::from(c)).collect();
                    let robot_1: String = decode(&directions);
                    if let Some(numeric_value) = NumericalKey::numeric(code) {
                        println!(
                            "complexity = {:?} * {:?}",
                            (robot_1.len() as i32 - 1_i32),
                            numeric_value
                        );
                        part1_answer += numeric_value * (robot_1.len() as i32 - 1_i32);
                    }
                    println!("human1 to robot 1: {robot_1}");
                    println!("robot1 to robot 2: {robot_2}");
                    println!("robot2 to robot 3: {numerical_instructions}");
                    println!("code: {:?}", code);
                }
                println!("part 1 answer: {part1_answer}");
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numerical_distance() -> Result<(), Box<dyn std::error::Error>> {
        let lhs: NumericalKey = NumericalKey::Zero;
        let rhs: NumericalKey = NumericalKey::Eight;
        let (row, column): (Direction, Direction) = lhs.traverse(&rhs);
        assert_eq!(row, Direction::Up(3i32));
        assert_eq!(column, Direction::Right(0i32));
        let rhs: NumericalKey = NumericalKey::One;
        let (row, column): (Direction, Direction) = lhs.traverse(&rhs);
        assert_eq!(row, Direction::Up(1i32));
        assert_eq!(column, Direction::Left(1i32));
        Ok(())
    }

    #[test]
    fn test_direction_distance() -> Result<(), Box<dyn std::error::Error>> {
        let lhs: DirectionalKey = DirectionalKey::Activate;
        let rhs: DirectionalKey = DirectionalKey::Left;
        let (row, column): (Direction, Direction) = lhs.traverse(&rhs);
        assert_eq!(row, Direction::Down(1i32));
        assert_eq!(column, Direction::Left(2i32));
        let rhs: DirectionalKey = DirectionalKey::Up;
        let (row, column): (Direction, Direction) = lhs.traverse(&rhs);
        assert_eq!(row, Direction::Up(0i32));
        assert_eq!(column, Direction::Left(1i32));
        Ok(())
    }
}
