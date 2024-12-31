use std::path::PathBuf;

use clap::Parser;

use nom::lib::std::cmp::Ordering;
use std::cmp::min;
use std::fs;
use std::ops::Range;

use super::{CommandImpl, DynError};

//use nom::sequence::preceded;

#[derive(Parser, Debug)]
pub struct Day9a {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Space {
    id: Option<usize>,
    range: Range<usize>,
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.range.start == other.range.start
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

impl Space {
    pub fn new(id: Option<usize>, start: usize, nblocks: usize) -> Self {
        Self { id, range: Range { start, end: (start + nblocks) } }
    }

    pub fn free_block(&self) -> bool {
        self.id.is_none()
    }

    pub fn nblocks(&self) -> usize {
        self.range.end - self.range.start
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.nblocks() >= other.nblocks()
    }

    pub fn value(&self) -> Option<usize> {
        if let Some(id) = self.id {
            let mut v: usize = 0;
            for i in self.range.start..self.range.end {
                v += i * id;
            }
            Some(v)
        } else {
            None
        }
    }

    pub fn fill(&self, other: &Self) -> Self {
        Space::new(other.id, self.range.start, min(self.nblocks(), other.nblocks()))
    }

    pub fn remainder(&self, other: &Self) -> Option<Self> {
        if self.nblocks() > other.nblocks() {
            Some(Space::new(
                self.id,
                self.range.start + other.nblocks(),
                self.nblocks() - other.nblocks(),
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct FileBlocks {
    free_blocks: Vec<Space>,
    file_blocks: Vec<Space>,
}

impl FileBlocks {
    pub fn new(mut free_blocks: Vec<Space>, mut file_blocks: Vec<Space>) -> Self {
        file_blocks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        free_blocks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self { free_blocks, file_blocks }
    }

    pub fn free_space(&self) -> bool {
        self.free_blocks.is_empty()
    }

    pub fn compress(&mut self) {
        let mut stack: Vec<Space> = Vec::new();
        while let Some(top) = self.file_blocks.pop() {
            let mut pushed: bool = false;
            for free_block_index in 0..self.free_blocks.len() {
                if self.free_blocks[free_block_index].contains(&top) {
                    let free_block = self.free_blocks.remove(free_block_index);
                    stack.push(free_block.fill(&top));
                    if let Some(fblock) = free_block.remainder(&top) {
                        self.free_blocks.push(fblock);
                    }
                    self.free_blocks.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                stack.push(top);
            }
        }
        self.file_blocks = stack
    }
}

impl CommandImpl for Day9a {
    fn main(&self) -> Result<(), DynError> {
        let string = fs::read_to_string(&self.input)?;
        let disk_map: Vec<usize> = string
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        let mut block_offset: usize = 0;
        let file_blocks = disk_map
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let id: Option<usize> = if i % 2 == 0 { Some(i / 2) } else { None };
                let space: Space = Space::new(id, block_offset, *v);
                block_offset += *v;
                space
            })
            .collect::<Vec<Space>>();
        println!("file blocks {:?}", file_blocks);

        let file_size: usize = disk_map.iter().sum();
        let mut block: Vec<usize> = vec![usize::MAX; file_size + 1];
        let mut block_index: usize = 0;
        for (i, v) in disk_map.iter().enumerate() {
            let block_id: usize = if i % 2 == 0 { i / 2 } else { usize::MAX };
            for j in 0..(*v) {
                block[block_index + j] = block_id;
            }
            block_index += *v;
        }
        for front in 0..block.len() {
            if block[front] == usize::MAX {
                for back in (front..block.len()).rev() {
                    if block[back] != usize::MAX {
                        block.swap(front, back);
                        break;
                    }
                }
            }
        }
        let mut mysum: usize = 0;
        let index = block.iter().position(|&x| x == usize::MAX).unwrap();
        for (i, v) in block[..index].iter().enumerate() {
            mysum += i * v;
        }
        println!("the block sum is {mysum}");

        Ok(())
    }
}
