use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2a {
    #[clap(long, short)]
    input: PathBuf,
}

pub fn differences(vec: &[i32]) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn valid(vec: &[i32]) -> bool {
    let diffs: Vec<i32> = differences(vec);
    let all_negative: bool = diffs.iter().all(|x| *x < 0);
    let all_positive: bool = diffs.iter().all(|x| *x > 0);
    let all_in_range: bool = diffs.iter().all(|x| x.abs() < 4);
    all_in_range && (all_negative || all_positive)
}

pub fn is_valid(vec: &[i32]) -> bool {
    if valid(vec) {
        return true;
    }
    for i in 0..vec.len() {
        let left_slice: &[i32] = &vec[0..i];
        let right_slice: &[i32] = if i < vec.len() - 1 { &vec[(i + 1)..] } else { &[] };
        let new_slice = [left_slice, right_slice].concat();
        if valid(&new_slice) {
            return true;
        }
    }
    false
}

impl CommandImpl for Day2a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let reports: Vec<Vec<i32>> =
            lines.iter().map(|s| s.split(" ").flat_map(|x| x.parse::<i32>()).collect()).collect();
        //println!("reports: {:?}", reports);
        let n_valid: usize = reports.iter().map(|x| is_valid(x)).filter(|&x| x).count();

        println!("there are {:?} valid reports", n_valid);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slices() {
        assert!(is_valid(&[80, 81, 83, 84, 87]));
    }
}
