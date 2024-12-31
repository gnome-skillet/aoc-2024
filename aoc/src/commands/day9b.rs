use std::path::PathBuf;

use clap::Parser;

use env_logger;
use log::debug;
use std::collections::VecDeque;
use std::fs;

use super::{CommandImpl, DynError};

//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day9b {
    #[clap(long, short)]
    input: PathBuf,
}

pub trait Identifiable {
    fn id(&self) -> usize;
}

pub trait Blockable {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0usize
    }
}

pub trait Subtractable {
    fn sub(&self, other: &Self) -> Self;
}

pub trait AbsoluteDifferencable {
    fn abs_diff(&self, other: &Self) -> usize;
}

pub trait Relocatable {
    fn relocate(&self, start: usize) -> Self;
}

pub trait CheckSummable {
    fn checksum(&self) -> usize;
}

pub type Block = (usize, usize, usize);

impl CheckSummable for Block {
    fn checksum(&self) -> usize {
        (self.start()..self.end()).map(|i| i * self.id()).sum()
    }
}

impl Relocatable for Block {
    fn relocate(&self, start: usize) -> Self {
        (start, start + self.len(), self.id())
    }
}

impl AbsoluteDifferencable for Block {
    fn abs_diff(&self, other: &Self) -> usize {
        other.start() - self.end()
    }
}

impl Subtractable for Block {
    fn sub(&self, other: &Self) -> Block {
        (self.end(), other.start(), usize::MAX)
    }
}

impl Blockable for Block {
    fn start(&self) -> usize {
        self.0
    }

    fn end(&self) -> usize {
        self.1
    }

    fn len(&self) -> usize {
        self.1 - self.0
    }
}

impl Identifiable for Block {
    fn id(&self) -> usize {
        self.2
    }
}

impl CommandImpl for Day9b {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        let string = fs::read_to_string(&self.input)?;
        let disk_map: Vec<usize> = string
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        let mut block_offset: usize = 0;
        let mut file_blocks: Vec<Block> = disk_map
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let block: Option<Block> =
                    if i % 2 == 0 { Some((block_offset, block_offset + *v, i / 2)) } else { None };
                block_offset += *v;
                block
            })
            .collect::<Vec<Block>>();

        let mut final_queue: VecDeque<Block> = VecDeque::new();
        debug!("file blocks = {:?}", file_blocks);
        while let Some(top) = file_blocks.pop() {
            let mut swapped: bool = false;
            for i in 1..file_blocks.len() {
                //let abs_diff: usize = file_blocks[i - 1].abs_diff(&file_blocks[i]);
                if file_blocks[i - 1].abs_diff(&file_blocks[i]) >= top.len() {
                    file_blocks.insert(i, top.relocate(file_blocks[i - 1].end()));
                    swapped = true;
                    break;
                }
            }
            if file_blocks.is_empty() {
                final_queue.push_front(top);
            } else if !swapped {
                let last = file_blocks.len() - 1;
                // corner case: space before last
                if file_blocks[last].abs_diff(&top) >= top.len() {
                    final_queue.push_front(top.relocate(file_blocks[last].end()));
                } else {
                    final_queue.push_front(top);
                }
            }
        }
        let checksum: usize = final_queue.iter().map(|x| x.checksum()).sum();
        debug!("final_queue = {:?}", final_queue);
        println!("checksum = {checksum}");
        //6286183402063 is too high
        //6286182965311

        Ok(())
    }
}
