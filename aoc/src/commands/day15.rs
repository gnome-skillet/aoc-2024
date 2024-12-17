use std::path::PathBuf;

use clap::Parser;

use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use nom::{
    character::complete::one_of,
    multi::{many0, many1},
};
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day15 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub enum Object {
    Robot,
    BigBoxLeft,
    BigBoxRight,
    Box,
    #[default]
    Wall,
    Space,
}

impl Object {
    pub fn new(tick: char) -> Self {
        match tick {
            '.' => Object::Space,
            '[' => Object::BigBoxLeft,
            ']' => Object::BigBoxRight,
            'O' => Object::Box,
            '@' => Object::Robot,
            '#' => Object::Wall,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Object>>,
    robot_position: (usize, usize),
}

#[derive(Debug)]
pub struct Direction {
    row: i32,
    column: i32,
}

pub fn find_robot(grid: &Vec<Vec<Object>>) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Object::Robot {
                return Some((i, j));
            }
        }
    }
    None
}

impl Direction {
    pub fn new(tick: char) -> Self {
        match tick {
            '^' => Direction { row: -1i32, column: 0i32 },
            'v' => Direction { row: 1i32, column: 0i32 },
            '>' => Direction { row: 0i32, column: 1i32 },
            '<' => Direction { row: 0i32, column: -1i32 },
            _ => panic!(),
        }
    }
}

impl Direction {
    fn next_position(&self, p: (usize, usize)) -> (usize, usize) {
        let row: i32 = p.0 as i32 + self.row;
        let column: i32 = p.1 as i32 + self.column;
        (row as usize, column as usize)
    }
}

impl Grid {
    pub fn new(grid: Vec<Vec<Object>>) -> Self {
        if let Some(robot_position) = find_robot(&grid) {
            Grid { grid, robot_position }
        } else {
            Grid { grid, robot_position: (usize::MAX, usize::MAX) }
        }
    }
}

impl Grid {
    pub fn move_robot(&mut self, d: Direction) {
        let (robot_row, robot_column) = self.robot_position;
        let (next_row, next_column) = d.next_position(self.robot_position);
        if self.grid[next_row][next_column] == Object::Wall {
            return;
        }
        if self.grid[next_row][next_column] == Object::Space {
            self.grid[robot_row][robot_column] = Object::Space;
            self.grid[next_row][next_column] = Object::Robot;
            self.robot_position = (next_row, next_column);
            return;
        }
        let (mut last_next_row, mut last_next_column) = (next_row, next_column);
        loop {
            let (next_next_row, next_next_column) =
                d.next_position((last_next_row, last_next_column));
            if self.grid[next_next_row][next_next_column] == Object::Wall {
                return;
            }
            if self.grid[next_next_row][next_next_column] == Object::Space {
                self.grid[robot_row][robot_column] = Object::Space;
                self.grid[next_row][next_column] = Object::Robot;
                self.robot_position = (next_row, next_column);
                self.grid[next_next_row][next_next_column] = Object::Box;
                return;
            }
            last_next_row = next_next_row;
            last_next_column = next_next_column;
        }
    }

    pub fn sum_boxes(&self) -> usize {
        let mut sumboxes: usize = 0;
        for row in 0..self.grid.len() {
            for column in 0..self.grid.len() {
                if self.grid[row][column] == Object::Box {
                    sumboxes += row * 100 + column;
                }
            }
        }
        sumboxes
    }
}

use regex::Regex;

fn parse_row(input: &str) -> IResult<&str, Vec<Object>> {
    let (input, row) = many1(one_of("#.@O"))(input)?;
    let s: String = row.into_iter().collect();
    let re = Regex::new(r"(?<space>[\.\#])").unwrap();
    let after = re.replace_all(&s, "$space$space");
    let re = Regex::new(r"(?<box>O)").unwrap();
    let after = re.replace_all(&after, "[]");
    let re = Regex::new(r"(?<robot>@)").unwrap();
    let after = re.replace_all(&after, "@.");
    let after_chars: Vec<char> = after.chars().collect();
    let objects: Vec<Object> = after_chars.iter().map(|x| Object::new(*x)).collect::<Vec<Object>>();
    Ok((input, objects))
}

fn parse_move_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, row) = many1(one_of("<>^v"))(input)?;
    Ok((input, row))
}

fn parse_rows(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, rows) = separated_list1(line_ending, parse_row)(input)?;
    Ok((input, rows))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, commands) = separated_list1(line_ending, parse_move_line)(input)?;
    let line =
        commands.into_iter().flatten().map(|c| Direction::new(c)).collect::<Vec<Direction>>();
    Ok((input, line))
}

fn parse_challenge(input: &str) -> IResult<&str, (Vec<Vec<Object>>, Vec<Direction>)> {
    let (input, objects) = parse_rows(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, moves) = parse_moves(input)?;

    Ok((input, (objects, moves)))
}

pub fn show(grid: &Grid) {
    println!("robot: {:?}, ", grid.robot_position);
    for row in 0..grid.grid.len() {
        for col in 0..grid.grid[0].len() {
            print!("{:?}, ", grid.grid[row][col]);
        }
        println!("");
    }
}

impl CommandImpl for Day15 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, (rows, moves))) = parse_challenge(&blob_string) else { todo!() };
        for row in rows.iter() {
            println!("row: {:?}", row);
        }
        let mut grid: Grid = Grid::new(rows);
        for m in moves.into_iter() {
            grid.move_robot(m);
        }
        println!("sum boxes = {:?}", grid.sum_boxes());

        Ok(())
    }
}
