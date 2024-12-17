use std::path::PathBuf;

use clap::Parser;

use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use nom::{character::complete::one_of, multi::many1};
use std::collections::HashMap;
use std::fmt;
use std::fs;

use std::collections::VecDeque;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day16 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Default, Copy, Clone, Hash, PartialEq)]
pub enum Object {
    #[default]
    Space,
    Start,
    End,
    Wall,
}

impl Object {
    pub fn new(tick: char) -> Self {
        match tick {
            '.' => Object::Space,
            'S' => Object::Start,
            'E' => Object::End,
            '#' => Object::Wall,
            _ => panic!(),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Wall => write!(f, "Wall "),
            Object::End => write!(f, "End  "),
            Object::Start => write!(f, "Start"),
            Object::Space => write!(f, "Space"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<Object>>,
}

#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub enum Direction {
    East(usize, usize),
    South(usize, usize),
    West(usize, usize),
    North(usize, usize),
}

const ROTATION_PENALTY: usize = 1000;
impl Direction {
    pub fn rotate(&self) -> Self {
        match self {
            Direction::East(r, c) => Direction::South(*r, *c),
            Direction::South(r, c) => Direction::West(*r, *c),
            Direction::West(r, c) => Direction::North(*r, *c),
            Direction::North(r, c) => Direction::East(*r, *c),
        }
    }

    pub fn move_one(&self) -> Self {
        match self {
            Direction::East(r, c) => Direction::East(*r, *c + 1usize),
            Direction::South(r, c) => Direction::South(*r + 1usize, *c),
            Direction::West(r, c) => Direction::West(*r, *c - 1usize),
            Direction::North(r, c) => Direction::North(*r - 1usize, *c),
        }
    }
}

impl Maze {
    pub fn new(maze: Vec<Vec<Object>>) -> Self {
        Maze { maze }
    }
}

#[derive(Debug)]
pub struct SearchMaze {
    visited: HashMap<Direction, usize>,
    search_queue: VecDeque<(Direction, usize)>,
    maze: Maze,
}

impl SearchMaze {
    pub fn new(maze: Maze) -> Self {
        let visited: HashMap<Direction, usize> = HashMap::new();
        let search_queue: VecDeque<(Direction, usize)> = VecDeque::new();
        Self { visited, search_queue, maze }
    }
}

impl SearchMaze {
    pub fn prime_search(&mut self) {
        if let Some(curr) = self.maze.find_start() {
            self.search_queue.push_back((curr, 0usize));
            self.visited.insert(curr, 0usize);
        } else {
            todo!()
        }
    }

    pub fn find_shortest_path(&mut self, _maze: &Maze) {
        while self.visited.is_empty() {
            if let Some((curr, score)) = self.search_queue.pop_front() {
                let curr: Direction = curr.move_one();
                let curr: Direction = curr.rotate();
                println!("move {:?}", curr.move_one());
            } else {
                todo!()
            }
            // if !visited
            //     add to queue
            //     3 * rotate add to queue
            // if score < old score
            //
        }
    }
}

impl Maze {
    pub fn find_start(&self) -> Option<Direction> {
        for i in 0..self.maze.len() {
            for j in 0..self.maze[i].len() {
                if self.maze[i][j] == Object::Start {
                    return Some(Direction::East(i, j));
                }
            }
        }
        None
    }

    pub fn sum_boxes(&self) -> usize {
        let mut sumboxes: usize = 0;
        for row in 0..self.maze.len() {
            for column in 0..self.maze.len() {
                if self.maze[row][column] == Object::End {
                    sumboxes += row * 100 + column;
                }
            }
        }
        sumboxes
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<Object>> {
    let (input, row) = many1(one_of("#.SE"))(input)?;
    let s: String = row.into_iter().collect();
    let after_chars: Vec<char> = s.chars().collect();
    let objects: Vec<Object> = after_chars.iter().map(|x| Object::new(*x)).collect::<Vec<Object>>();
    Ok((input, objects))
}

fn parse_rows(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, rows) = separated_list1(line_ending, parse_row)(input)?;
    Ok((input, rows))
}

fn parse_challenge(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, objects) = parse_rows(input)?;
    Ok((input, objects))
}

pub fn show(maze: &Maze) {
    for row in 0..maze.maze.len() {
        for col in 0..maze.maze[0].len() {
            print!("{:?}, ", maze.maze[row][col]);
        }
        println!("");
    }
}

impl CommandImpl for Day16 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, rows)) = parse_challenge(&blob_string) else { todo!() };
        for row in rows.iter() {
            println!("row: {:?}", row);
        }
        let maze: Maze = Maze::new(rows);
        let search_maze: SearchMaze = SearchMaze::new(maze);
        println!("search maze: {:?}", search_maze);
        //for m in moves.into_iter() {
        //    maze.move_robot(m);
        //}
        //println!("sum boxes = {:?}", maze.sum_boxes());

        Ok(())
    }
}
