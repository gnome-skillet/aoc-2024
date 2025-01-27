use log::{debug, info};
use std::collections::HashSet;
use std::collections::VecDeque;
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
    BoxLeft,
    BoxRight,
    Box,
    #[default]
    Wall,
    Space,
}

impl Object {
    pub fn new(tick: char) -> Self {
        match tick {
            '.' => Object::Space,
            '[' => Object::BoxLeft,
            ']' => Object::BoxRight,
            'O' => Object::Box,
            '@' => Object::Robot,
            '#' => Object::Wall,
            _ => panic!(),
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Object::Space => '.',
            Object::BoxLeft => '[',
            Object::BoxRight => ']',
            Object::Box => 'O',
            Object::Robot => '@',
            Object::Wall => '#',
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Object>>,
    robot_position: (usize, usize),
}

#[derive(Debug, Copy, Clone)]
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

pub type Point = (usize, usize);

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
    fn next_position(&self, p: Point) -> Point {
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
    fn show_robot_position(&self) {
        println!("robot position: {:?}", self.robot_position);
    }

    fn show_rows(&self) {
        for row in self.grid.iter() {
            for x in row.iter() {
                print!("{}", x.to_char());
            }
            println!();
        }
    }

    fn vertical_move(&mut self, d: Direction, i: usize) {
        let mut curr_p: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut steps: Vec<(usize, usize)> = Vec::new();
        curr_p.push_back(self.robot_position);
        while let Some((row, column)) = curr_p.pop_front() {
            if !visited.contains(&(row, column)) {
                steps.push((row, column));
                visited.insert((row, column));
            }
            let (next_row, next_column) = d.next_position((row, column));
            match self.grid[next_row][next_column] {
                Object::Wall => {
                    return;
                }
                Object::Box => {
                    curr_p.push_back((next_row, next_column));
                }
                Object::BoxLeft => {
                    curr_p.push_back((next_row, next_column));
                    if self.grid[row][column] != self.grid[next_row][next_column] {
                        curr_p.push_back((next_row, next_column + 1));
                    }
                }
                Object::BoxRight => {
                    curr_p.push_back((next_row, next_column));
                    if self.grid[row][column] != self.grid[next_row][next_column] {
                        curr_p.push_back((next_row, next_column - 1));
                    }
                }
                Object::Space => {}
                _ => (),
            }
        }
        while let Some(curr) = steps.pop() {
            let (next_row, next_column) = d.next_position((curr.0, curr.1));
            self.grid[next_row][next_column] = self.grid[curr.0][curr.1];
            self.grid[curr.0][curr.1] = Object::Space;
        }
        self.robot_position = d.next_position(self.robot_position);
    }

    fn horizontal_move(&mut self, d: Direction) {
        let mut curr_p: VecDeque<(usize, usize)> = VecDeque::new();
        let mut steps: Vec<(usize, usize)> = Vec::new();
        curr_p.push_back(self.robot_position);
        while let Some((row, column)) = curr_p.pop_front() {
            steps.push((row, column));
            let (next_row, next_column) = d.next_position((row, column));
            match self.grid[next_row][next_column] {
                Object::Wall => {
                    return;
                }
                Object::BoxLeft | Object::BoxRight | Object::Box => {
                    curr_p.push_back((next_row, next_column));
                }
                Object::Space => {
                    break;
                }
                _ => panic!(),
            }
        }
        while let Some(curr) = steps.pop() {
            let (next_row, next_column) = d.next_position((curr.0, curr.1));
            self.grid[next_row][next_column] = self.grid[curr.0][curr.1];
            self.grid[curr.0][curr.1] = Object::Space;
        }
        self.robot_position = d.next_position(self.robot_position);
    }

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
        for row in 1..self.grid.len() {
            for column in 1..self.grid[row].len() {
                if self.grid[row][column] == Object::Box
                    || self.grid[row][column] == Object::BoxLeft
                {
                    let curr = row * 100 + column;
                    sumboxes += curr;
                }
            }
        }
        sumboxes
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<Object>> {
    let (input, row) = many1(one_of("#.@O"))(input)?;
    let s: String = row.into_iter().collect();
    let after_chars: Vec<char> = s.chars().collect();
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

fn double_up(objects: &Vec<Object>) -> Vec<Object> {
    objects.iter().flat_map(|v| vec![*v, *v]).map(|x| x).collect()
}

impl CommandImpl for Day15 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, (rows, moves))) = parse_challenge(&blob_string) else { todo!() };
        let mut grid: Grid = Grid::new(rows);
        for m in moves.iter() {
            grid.move_robot(*m);
        }
        println!("sum boxes = {:?}", grid.sum_boxes());

        let Ok((_, (rows, _))) = parse_challenge(&blob_string) else { todo!() };
        let mut rows: Vec<Vec<Object>> = rows.iter().map(|r| double_up(r)).collect();
        for row in rows.iter_mut() {
            let mut box_conversion: bool = false;
            let mut second_robot: bool = false;
            for v in row.iter_mut() {
                if *v == Object::Robot {
                    if second_robot {
                        *v = Object::Space;
                        second_robot = false;
                    } else {
                        second_robot = true;
                    }
                }
                if *v == Object::Box {
                    if box_conversion {
                        *v = Object::BoxRight;
                        box_conversion = false;
                    } else {
                        *v = Object::BoxLeft;
                        box_conversion = true;
                    }
                }
            }
        }
        let mut grid: Grid = Grid::new(rows);
        for (i, m) in moves.iter().enumerate() {
            if m.column == 0 {
                grid.vertical_move(*m, i);
            } else {
                grid.horizontal_move(*m);
            }
        }
        println!("sum boxes = {:?}", grid.sum_boxes());

        Ok(())
    }
}
