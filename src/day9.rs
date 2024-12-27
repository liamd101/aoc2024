use std::collections::VecDeque;
use std::fs;
use std::ops::Range;

use tracing::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TripleId(isize, usize, usize);
impl TripleId {
    /// Create a new TripleId
    pub fn new(id: isize, start: usize, length: usize) -> Self {
        Self(id, start, length)
    }

    /// Get the identification value
    pub fn id(&self) -> isize {
        self.0
    }

    /// Get the starting index
    pub fn start(&self) -> usize {
        self.1
    }

    /// Get the length
    pub fn length(&self) -> usize {
        self.2
    }

    /// Compute the ending index (start + length)
    pub fn end(&self) -> usize {
        self.start() + self.length()
    }

    /// Check if a given index is within the range of [start, end)
    pub fn contains(&self, index: usize) -> bool {
        index >= self.start() && index < self.end()
    }

    pub fn range(&self) -> Range<usize> {
        self.start()..self.end()
    }
}

fn parse_into_filesystem(nums: &[isize]) -> String {
    nums.iter()
        .map(|&x| {
            if x == -1 {
                '.'
            } else {
                char::from_digit(x as u32, 10).unwrap()
            }
        })
        .collect()
}

pub fn run(full: bool) {
    let file = crate::utils::get_input(9, full);
    let line: &str = file.lines().collect::<Vec<&str>>()[0];
    part1(line);
    part2(line);
}

fn part1(line: &str) {
    let mut filesystem: Vec<isize> = Vec::new();
    let mut ident = 0;
    // creating the filesystem before altering
    for (i, c) in line.chars().map(|x| x.to_digit(10).unwrap()).enumerate() {
        let added = if i % 2 == 0 {
            ident
        } else {
            ident += 1;
            -1
        };
        for _ in 0..c {
            filesystem.push(added);
        }
    }
    debug!("{:?}", parse_into_filesystem(&filesystem));

    // find all spaces that can be replaced
    let mut open_space = filesystem
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&(_, c)| c == -1)
        .map(|(i, _)| i)
        .collect::<VecDeque<usize>>();
    debug!("{:?}", open_space);

    // swap open spaces with rightmost spaces
    for (i, &c) in filesystem.clone().iter().enumerate().rev() {
        if c == -1 {
            continue;
        }
        let unoccupied = open_space.pop_front();
        match unoccupied {
            Some(idx) => {
                if i > idx {
                    filesystem[idx] = c;
                    filesystem[i] = -1;
                }
            }
            None => break,
        }
        debug!("{:?}", unoccupied);
    }
    debug!("{:?}", parse_into_filesystem(&filesystem));

    // compute checksum
    let checksum = filesystem
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != -1)
        .fold(0, |acc, (i, &c)| acc + (i * c as usize));
    info!("part 1: {}", checksum);
}

fn part2(line: &str) {
    let mut filesystem: Vec<TripleId> = Vec::new();

    // creating the filesystem before altering
    let mut ident = 0;
    let mut index = 0;
    for (i, c) in line.chars().map(|x| x.to_digit(10).unwrap()).enumerate() {
        let added = if i % 2 == 0 {
            ident
        } else {
            ident += 1;
            -1
        };
        let added = TripleId::new(added, index, c as usize);
        filesystem.push(added);
        index = added.end();
    }
    debug!("{:?}", filesystem);

    let open_space = filesystem
        .clone()
        .into_iter()
        .filter(|&triple| triple.id() == -1)
        .collect::<VecDeque<TripleId>>();
    debug!("{:?}", open_space);

    for triple in filesystem.clone().iter().rev() {
        let swap_space = open_space.iter().find(|x| {
            for index in triple.range() {
                if !x.contains(index) {
                    return false;
                }
            }
            x.start() < triple.start()
        });
        match swap_space {
            Some(swappable) => {
                debug!("swappable: {:?}", swappable);
            }
            None => continue,
        }
    }

    let checksum = filesystem
        .iter()
        .enumerate()
        .filter(|(_, &c)| c.id() != -1)
        .fold(0, |acc, (_, &c)| {
            let mut sum = acc;
            for idx in c.range() {
                sum += idx * c.id() as usize;
            }
            sum
        });
    info!("part 2: {}", checksum);
}
