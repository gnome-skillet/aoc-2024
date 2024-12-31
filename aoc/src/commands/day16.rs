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

trait Indexable {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
}

pub type Point = (usize, usize);
impl Indexable for Point {
    fn row(&self) -> usize {
        self.0
    }

    fn column(&self) -> usize {
        self.1
    }
}

impl Indexable for DirectedParticle {
    fn row(&self) -> usize {
        match *self {
            DirectedParticle::East(r, _) => r,
            DirectedParticle::South(r, _) => r,
            DirectedParticle::West(r, _) => r,
            DirectedParticle::North(r, _) => r,
        }
    }

    fn column(&self) -> usize {
        match *self {
            DirectedParticle::East(_, c) => c,
            DirectedParticle::South(_, c) => c,
            DirectedParticle::West(_, c) => c,
            DirectedParticle::North(_, c) => c,
        }
    }
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

#[derive(Debug)]
pub struct Maze {
    blueprint: Vec<Vec<Object>>,
    start: DirectedParticle,
    end: Point,
    vertices: HashMap<Vertex, Weight>,
}

impl Maze {
    pub fn new(blueprint: Vec<Vec<Object>>, start: DirectedParticle, end: Point) -> Self {
        let vertices: HashMap<Vertex, Weight> = HashMap::new();
        Maze { blueprint, start, end, vertices }
    }

    pub fn add_vertex<T: Into<Point>>(&mut self, lhs: T, rhs: T, w: Weight) {
        self.vertices.insert((lhs.into(), rhs.into()), w);
    }

    pub fn reachable(&self, p: Point) -> bool {
        self.blueprint[p.row()][p.column()] != Object::Wall
    }

    pub fn nrows(&self) -> usize {
        self.blueprint.len()
    }

    pub fn ncols(&self) -> usize {
        self.blueprint[0].len()
    }

    pub fn goal_reached(&self, p: &DirectedParticle) -> bool {
        self.blueprint[p.row()][p.column()] == Object::End
    }
}

type Vertex = (Point, Point);
type Weight = usize;

#[derive(Debug)]
pub struct ShortestPath {
    maze: Maze,
    visited: HashMap<DirectedParticle, usize>,
    queue: VecDeque<(DirectedParticle, usize, usize)>,
    best_score: usize,
    nsquares: usize,
}

impl ShortestPath {
    pub fn new(maze: Maze) -> Self {
        let visited: HashMap<DirectedParticle, usize> = HashMap::new();
        let queue: VecDeque<(DirectedParticle, usize, usize)> = VecDeque::new();
        Self { maze, visited, queue, best_score: usize::MAX, nsquares: 0_usize }
    }
}

impl ShortestPath {
    pub fn beats_score(&mut self, p: &DirectedParticle, score: usize) -> bool {
        if !self.visited.contains_key(p) {
            return true;
        }
        let curr_score = self.visited.get(p).cloned().unwrap_or(usize::MAX);
        score <= curr_score
    }

    pub fn beats_or_equals_best_score(&self, score: usize) -> bool {
        score <= self.best_score
    }

    pub fn beats_best_score(&self, score: usize) -> bool {
        score < self.best_score
    }

    pub fn best_score(&self) -> usize {
        self.best_score
    }

    pub fn nsquares(&self) -> usize {
        self.nsquares
    }

    pub fn update_score(&mut self, p: &DirectedParticle, score: usize, nsquares: usize) {
        let curr_score = self.visited.get(p).cloned().unwrap_or(usize::MAX);
        if score < curr_score {
            self.visited.insert(*p, score);
        }
        let curr: Point = p.into();
        if curr == self.maze.end && score < self.best_score {
            self.best_score = score;
            self.nsquares = nsquares;
        }
    }

    pub fn initialize_queue(&mut self) {
        let s: DirectedParticle = self.maze.start;
        self.enqueue(s, 0_usize, 1_usize);
    }

    pub fn enqueue(&mut self, p: DirectedParticle, score: usize, nsquares: usize) {
        self.queue.push_back((p, score, nsquares));
        self.update_score(&p, score, nsquares);
    }

    pub fn dequeue(&mut self) -> Option<(DirectedParticle, usize, usize)> {
        self.queue.pop_front()
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn nvisited(&self) -> usize {
        self.visited.len()
    }

    pub fn search(&mut self) -> usize {
        self.initialize_queue();
        let mut npaths: usize = 0;
        let mut n_squares_total: usize = 0;

        while !self.is_queue_empty() {
            if let Some((mut top, score, nsquares)) = self.dequeue() {
                if self.maze.goal_reached(&top) && self.beats_or_equals_best_score(score) {
                    if self.beats_best_score(score) {
                        npaths = 1;
                        n_squares_total = nsquares;
                        self.update_score(&top, score, nsquares);
                    } else {
                        npaths += 1;
                        n_squares_total += nsquares;
                    }
                } else {
                    for i in 0..4 {
                        let penalty: usize = match i {
                            1 | 3 => 1,
                            2 => 2,
                            _ => 0,
                        };
                        if let Some(s) = top.move_one() {
                            let new_score: usize = score + penalty * ROTATION_PENALTY + 1;
                            if self.beats_score(&s, new_score)
                                && self.maze.reachable((&s).into())
                                && self.beats_best_score(new_score)
                            {
                                self.enqueue(s, new_score, nsquares + 1);
                                self.maze.add_vertex(top, s, new_score - score);
                            }
                        };
                        top = top.rotate();
                    }
                }
            }
        }
        println!("n paths {npaths}");
        println!("n squares with recounts {n_squares_total}");
        self.best_score
    }

    pub fn backtrack(&mut self) {
        self.visited.retain(|_, v| *v <= self.best_score);
    }
}

#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub enum DirectedParticle {
    East(usize, usize),
    South(usize, usize),
    West(usize, usize),
    North(usize, usize),
}

impl DirectedParticle {
    pub fn rotate(&self) -> Self {
        match *self {
            DirectedParticle::East(r, c) => DirectedParticle::South(r, c),
            DirectedParticle::South(r, c) => DirectedParticle::West(r, c),
            DirectedParticle::West(r, c) => DirectedParticle::North(r, c),
            DirectedParticle::North(r, c) => DirectedParticle::East(r, c),
        }
    }

    pub fn move_one(&self) -> Option<Self> {
        match *self {
            DirectedParticle::East(r, c) => Some(DirectedParticle::East(r, c + 1usize)),
            DirectedParticle::South(r, c) => Some(DirectedParticle::South(r + 1usize, c)),
            DirectedParticle::West(r, c) => Some(DirectedParticle::West(r, c - 1usize)),
            DirectedParticle::North(r, c) => Some(DirectedParticle::North(r - 1usize, c)),
        }
    }
}

impl From<DirectedParticle> for Point {
    fn from(p: DirectedParticle) -> Self {
        (p.row(), p.column())
    }
}

impl From<&DirectedParticle> for Point {
    fn from(p: &DirectedParticle) -> Self {
        (p.row(), p.column())
    }
}

pub fn find_start(maze: &[Vec<Object>]) -> Option<DirectedParticle> {
    for (r, row) in maze.iter().enumerate() {
        for (c, object) in row.iter().enumerate() {
            if *object == Object::Start {
                return Some(DirectedParticle::East(r, c));
            }
        }
    }
    None
}

pub fn find_end(maze: &[Vec<Object>]) -> Option<Point> {
    for (r, row) in maze.iter().enumerate() {
        for (c, object) in row.iter().enumerate() {
            if *object == Object::End {
                return Some((r, c));
            }
        }
    }
    None
}

const ROTATION_PENALTY: usize = 1000;

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

impl CommandImpl for Day16 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, rows)) = parse_challenge(&blob_string) else { todo!() };
        if let (Some(start), Some(end)) = (find_start(&rows), find_end(&rows)) {
            let maze: Maze = Maze::new(rows, start, end);
            println!("start: {:?}", maze.start);
            println!("end: {:?}", maze.end);
            let mut shortest_path: ShortestPath = ShortestPath::new(maze);
            shortest_path.search();
            println!("visited: {:?}", shortest_path.nvisited());
            shortest_path.backtrack();
            println!("visited (after backtrack): {:?}", shortest_path.nvisited());
            println!("(shortest path: {:?}", shortest_path.best_score());
            println!("(n squares: {:?}", shortest_path.nsquares());
        }

        Ok(())
    }
}
