use clap::Parser;
use std::path::PathBuf;

use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use nom::{character::complete::one_of, multi::many1};

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day20 {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_row(input: &str) -> IResult<&str, Vec<Object>> {
    let (input, row) = many1(one_of("#.SE"))(input)?;
    let s: String = row.into_iter().collect();
    let after_chars: Vec<char> = s.chars().collect();
    let objects: Vec<Object> = after_chars.iter().map(|x| Object::new(*x)).collect::<Vec<Object>>();
    Ok((input, objects))
}

fn parse_rows(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, maze) = separated_list1(line_ending, parse_row)(input)?;
    Ok((input, maze))
}

pub type Point = (usize, usize);

#[derive(Debug, EnumIter)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

pub trait Distanceable {
    fn distance(&self, other: &Self) -> usize;
}

impl Distanceable for Point {
    fn distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

pub trait Neighborable {
    type Item;
    fn neighborhood(&self, nrows: usize, ncols: usize) -> impl Iterator;
}

pub trait Cheatable {
    type Item;
    fn cheats(&self, nrows: usize, ncols: usize, cheat_duration: usize) -> impl Iterator;
}

impl Neighborable for Point {
    type Item = Point;

    fn neighborhood(&self, nrows: usize, ncols: usize) -> impl Iterator<Item = Point> {
        let minrow: usize = if self.row() == 0 { self.row() } else { self.row() - 1 };
        let mincol: usize = if self.column() == 0 { self.column() } else { self.column() - 1 };
        let maxrow: usize = if self.row() == nrows - 1 { self.row() } else { self.row() + 1 };
        let maxcol: usize =
            if self.column() == ncols - 1 { self.column() } else { self.column() + 1 };
        (minrow..=maxrow)
            .cartesian_product(mincol..=maxcol)
            .filter(|&p| p != *self && self.distance(&p) == 1)
    }
}

impl Cheatable for Point {
    type Item = Point;

    fn cheats(
        &self,
        nrows: usize,
        ncols: usize,
        cheat_duration: usize,
    ) -> impl Iterator<Item = Point> {
        let minrow: usize =
            if self.row() < cheat_duration { 0usize } else { self.row() - cheat_duration };
        let mincol: usize =
            if self.column() < cheat_duration { 0usize } else { self.column() - cheat_duration };
        let maxrow: usize = if nrows - self.row() - 1 < cheat_duration {
            nrows - 1
        } else {
            self.row() + cheat_duration
        };
        let maxcol: usize = if ncols - self.column() - 1 < cheat_duration {
            ncols - 1
        } else {
            self.column() + cheat_duration
        };

        (minrow..=maxrow).cartesian_product(mincol..=maxcol)
    }
}

pub fn find_start(maze: &[Vec<Object>]) -> Option<Point> {
    for (r, row) in maze.iter().enumerate() {
        for (c, object) in row.iter().enumerate() {
            if *object == Object::Start {
                return Some((r, c));
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

trait Indexable {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
}

impl Indexable for Point {
    fn row(&self) -> usize {
        self.0
    }

    fn column(&self) -> usize {
        self.1
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
    start: Point,
    end: Point,
}

impl Maze {
    pub fn new(blueprint: Vec<Vec<Object>>, start: Point, end: Point) -> Self {
        Maze { blueprint, start, end }
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

    pub fn goal_reached(&self, p: &Point) -> bool {
        self.blueprint[p.row()][p.column()] == Object::End
    }
}

#[derive(Debug)]
pub struct ShortestPath {
    maze: Maze,
}

impl ShortestPath {
    pub fn new(maze: Maze) -> Self {
        Self { maze }
    }
}

impl ShortestPath {
    pub fn search(&mut self, current_score: usize) -> usize {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
        let mut best_score: usize = usize::MAX;
        let nrows: usize = self.maze.nrows();
        let ncols: usize = self.maze.ncols();
        queue.push_back((self.maze.start, current_score));

        while !queue.is_empty() {
            if let Some((mut top, score)) = queue.pop_front() {
                if self.maze.goal_reached(&top) && score < best_score {
                    best_score = score;
                } else {
                    for neighbor in top.neighborhood(nrows, ncols) {
                        if self.maze.reachable(neighbor) && !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back((neighbor, score + 1));
                        };
                    }
                }
            }
        }
        best_score
    }

    fn distance_matrix(&self) -> Vec<Vec<usize>> {
        let mut dm: Vec<Vec<usize>> = self
            .maze
            .blueprint
            .iter()
            .map(|x| x.iter().map(|x| usize::MAX).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        let mut visited: HashSet<Point> = HashSet::new();
        let nrows: usize = self.maze.nrows();
        let ncols: usize = self.maze.ncols();
        let goal: Point = self.maze.end;
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
        dm[goal.0][goal.1] = 0usize;
        visited.insert(goal);
        queue.push_back((goal, 0usize));
        while let Some((point, score)) = queue.pop_front() {
            for neighbor in point.neighborhood(nrows, ncols) {
                if !visited.contains(&neighbor) && self.maze.reachable(neighbor) {
                    dm[neighbor.0][neighbor.1] = score + 1;
                    visited.insert(neighbor);
                    queue.push_back((neighbor, score + 1));
                }
            }
        }
        let score: usize = dm[self.maze.start.0][self.maze.start.1];
        dm
    }

    fn tally_cheats(&self, dm: Vec<Vec<usize>>) -> HashMap<usize, usize> {
        let mut queue: VecDeque<Point> = VecDeque::new();
        let nrows: usize = self.maze.nrows();
        let ncols: usize = self.maze.ncols();
        let mut improvements: HashMap<usize, usize> = HashMap::new();
        let mut ncount: usize = 0usize;
        queue.push_back(self.maze.start);
        while let Some(point) = queue.pop_front() {
            let nsteps: usize = dm[point.0][point.1];
            if nsteps == 0 {
                break;
            }
            for cheat in point.cheats(nrows, ncols, 20) {
                let child_steps: usize = dm[cheat.0][cheat.1];
                if child_steps < nsteps && nsteps - child_steps >= 2 {
                    let diff: usize = nsteps - child_steps - point.distance(&cheat);
                    *improvements.entry(diff).or_insert(0usize) += 1;
                }
            }
            for neighbor in point.neighborhood(nrows, ncols) {
                if self.maze.reachable(neighbor) && dm[neighbor.0][neighbor.1] == nsteps - 1 {
                    queue.push_back(neighbor);
                }
            }
        }
        improvements
    }
}

impl CommandImpl for Day20 {
    fn main(&self) -> Result<(), DynError> {
        let blob_string = fs::read_to_string(&self.input)?;
        let Ok((_, rows)) = parse_rows(&blob_string) else { todo!() };
        if let (Some(start), Some(end)) = (find_start(&rows), find_end(&rows)) {
            let maze: Maze = Maze::new(rows, start, end);
            let mut shortest_path: ShortestPath = ShortestPath::new(maze);
            let best_score = shortest_path.search(0usize);
            println!("shortest path: {best_score}");
            let dm: Vec<Vec<usize>> = shortest_path.distance_matrix();
            let cheatmap: HashMap<usize, usize> = shortest_path.tally_cheats(dm);
            let mut n: usize = 0usize;
            println!("cheats {:?}", cheatmap);
            for (k, v) in cheatmap.into_iter() {
                if k >= 100 {
                    n += v;
                }
            }
            println!("there are {n} cheats that will save at least 100 picoseconds");
        }

        Ok(())
    }
}
//
//#[cfg(test)]
//mod test {
//    use super::*;
//    use rstest::*;
//}
