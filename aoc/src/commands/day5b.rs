use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day5b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day5b {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
