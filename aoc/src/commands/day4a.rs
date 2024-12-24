use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};
use crate::utils::slurp_file;
use std::ops::{Add, Mul};

#[derive(Parser, Debug)]
pub struct Day4a {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum Bits {
    X,
    M,
    A,
    S,
    SA,
    MA,
    XM,
    XMA,
    MAS,
    SAM,
    XMAS,
    SAMX,
    Unknown,
}

impl Bits {
    pub fn new(c: char) -> Self {
        match c {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Unknown,
        }
    }

    pub fn matches(&self) -> bool {
        matches!(self, Bits::XMAS | Bits::SAMX)
    }

    pub fn matches_mas(&self) -> bool {
        matches!(self, Bits::MAS | Bits::SAM)
    }
}

impl Add for Bits {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::X, Self::M) => Self::XM,
            (Self::S, Self::A) => Self::SA,
            (Self::XM, Self::A) => Self::XMA,
            (Self::SA, Self::M) => Self::SAM,
            (Self::SAM, Self::X) => Self::SAMX,
            (Self::SAMX, Self::M) => Self::XM,
            (Self::XMA, Self::S) => Self::XMAS,
            (Self::XMAS, Self::A) => Self::SA,
            (Self::Unknown, _) => other,
            (_, Self::X) => Self::X,
            (_, Self::S) => Self::S,
            (_, _) => Self::Unknown,
        }
    }
}

impl Mul for Bits {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::X, _) => Self::Unknown,
            (Self::Unknown, _) => Self::Unknown,
            (Self::M, Self::A) => Self::MA,
            (Self::S, Self::A) => Self::SA,
            (Self::MA, Self::S) => Self::MAS,
            (Self::SA, Self::M) => Self::SAM,
            (_, _) => Self::Unknown,
        }
    }
}

//fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
//where
//    T: Clone,
//{
//    assert!(!v.is_empty());
//    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
//}

pub struct Letters {
    pub letters: Vec<Vec<Bits>>,
}

impl Letters {
    pub fn new(letters: Vec<Vec<Bits>>) -> Self {
        Self { letters }
    }

    pub fn display(&self) {
        for i in 0..self.letters.len() {
            println!("{:?}", self.letters[i]);
        }
    }

    pub fn count_position(&self, row: usize, col: usize) -> usize {
        let mut n: usize = 0;
        if self.vertical(row, col) {
            n += 1;
        }
        if self.horizontal(row, col) {
            n += 1;
        }
        if self.diagonal(row, col) {
            n += 1;
        }
        if self.cross_diagonal(row, col) {
            n += 1;
        }
        n
    }

    pub fn cumsum(&self) -> usize {
        let mut n: usize = 0;
        for i in 0..self.letters.len() {
            for j in 0..self.letters[0].len() {
                n += self.count_position(i, j);
            }
        }
        n
    }

    pub fn cumsum_mas(&self) -> usize {
        let mut n: usize = 0;
        for row in 1..self.letters.len() - 1 {
            for col in 1..self.letters[0].len() - 1 {
                if self.cross_match(row, col) {
                    n += 1;
                }
            }
        }
        n
    }

    fn vertical(&self, row: usize, col: usize) -> bool {
        if row > self.letters.len() - 4 {
            return false;
        }
        if col > self.letters[0].len() {
            return false;
        }
        let mut x = Bits::Unknown;
        for i in row..(row + 4) {
            x = x + self.letters[i][col];
        }
        x.matches()
    }

    fn cross_match(&self, row: usize, col: usize) -> bool {
        let mut x = self.letters[row - 1][col - 1];
        print!("{row},{col}: one - {:?}", x);
        let z = self.letters[row][col];
        x = x * z;
        print!(", * {:?} = {:?}", z, x);
        let z = self.letters[row + 1][col + 1];
        x = x * z;
        println!(", * {:?} = {:?}", z, x);

        let mut y = self.letters[row + 1][col - 1];
        print!("{row},{col}: one - {:?}", y);
        let z = self.letters[row][col];
        y = y * z;
        print!(", * {:?} = {:?}", z, y);
        let z = self.letters[row - 1][col + 1];
        y = y * z;
        println!(", * {:?} = {:?}", z, y);
        if x.matches_mas() && y.matches_mas() {
            println!("matches {row},{col}");
        }
        x.matches_mas() && y.matches_mas()
    }

    //    fn vertical_mul(&self, row: usize, col: usize) -> bool {
    //        if row > self.letters.len() - 3 {
    //            return false;
    //        }
    //        if col > self.letters[0].len() {
    //            return false;
    //        }
    //        let mut x = Bits::Unknown;
    //        for i in row..(row + 3) {
    //            x = x * self.letters[i][col];
    //        }
    //        x.matches_mas()
    //    }
    //
    fn horizontal(&self, row: usize, col: usize) -> bool {
        if self.letters.is_empty() {
            return false;
        }
        if col + 4 > self.letters[0].len() {
            return false;
        }
        let mut x = Bits::Unknown;
        for i in col..(col + 4) {
            x = x + self.letters[row][i];
        }
        x.matches()
    }

    fn cross_diagonal(&self, row: usize, col: usize) -> bool {
        if self.letters.is_empty() {
            return false;
        }
        if row < 3 {
            return false;
        }
        if col < 3 {
            return false;
        }
        let mut x = Bits::Unknown;
        for i in 0..4 {
            x = x + self.letters[row - i][col - i];
        }
        x.matches()
    }

    fn diagonal(&self, row: usize, col: usize) -> bool {
        if self.letters.is_empty() {
            return false;
        }
        if row + 4 > self.letters.len() {
            return false;
        }
        if col < 3 {
            return false;
        }
        let mut x = Bits::Unknown;
        for i in 0..4 {
            x = x + self.letters[row + i][col - i];
        }
        x.matches()
    }
}

impl CommandImpl for Day4a {
    fn main(&self) -> Result<(), DynError> {
        let string: Vec<String> = slurp_file(&self.input)?;
        let bits: Vec<Vec<Bits>> = string
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .map(|c| c.iter().map(|x| Bits::new(*x)).collect::<Vec<Bits>>())
            .collect();
        let letters: Letters = Letters::new(bits);
        //letters.display();
        let count: usize = letters.cumsum();

        println!("counted XMAS {count} times");
        let count_mas: usize = letters.cumsum_mas();
        println!("counted cross-MAS {count_mas} times");
        //let n: usize = letters.count_position(4, 0);
        //println!("counted XMAS {n} times at (4,0)");
        Ok(())
    }
}
