use std::collections::VecDeque;
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

    pub fn as_vec(&self) -> Vec<isize> {
        self.range().map(|_| self.id()).collect()
    }

    // need a function to "put" one triple into another
    // this should only be allowed if the size of the triple being put into is larger than the one
    // being put in
    // this should also only be allowed if the triple being put in is not already occupied (id !=
    // -1)
    // it should place the triple in the leftmost position
    pub fn put(&self, other: TripleId) -> std::io::Result<(Option<TripleId>, TripleId)> {
        if self.length() < other.length() || self.id() != -1 || self.start() > other.start() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid input",
            ));
        }
        let new_start = self.start() + other.length();
        let new_length = self.length() - other.length();
        let condensed_triple = if new_length == 0 {
            None
        } else {
            Some(TripleId::new(self.id(), new_start, new_length))
        };
        let moved_triple = TripleId::new(other.id(), self.start(), other.length());

        Ok((condensed_triple, moved_triple))
    }
}

fn parse_into_filesystem(nums: &[isize]) -> String {
    nums.iter()
        .map(|&x| {
            if x == -1 {
                '.'
            } else {
                char::from_digit(x as u32, 10).unwrap_or(' ')
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

    #[allow(unused_mut)]
    let mut open_space = filesystem
        .clone()
        .into_iter()
        .filter(|&triple| triple.id() == -1)
        .collect::<VecDeque<TripleId>>();
    debug!("{:?}", open_space);

    let mut new_filesystem: Vec<TripleId> = vec![];

    for triple in filesystem.clone().iter().rev() {
        // iterate through all triples in reverse
        // find the leftmost open space that can be swapped with the current triple
        // if there is no open space that can fit the current triple, continue
        // if there is an open space that can fit the current triple, put the current triple in the
        // open space and continue

        if triple.id() == -1 {
            continue;
        }

        let swap_space = open_space.iter().enumerate().find(|&(_, space)| {
            space.length() >= triple.length() && triple.start() > space.start()
        });

        match swap_space {
            Some((idx, space)) => {
                let (condensed, moved) = space.put(*triple).unwrap();
                if let Some(condensed) = condensed {
                    open_space[idx] = condensed;
                } else {
                    // delete the open space if it is fully occupied
                    open_space.remove(idx);
                }
                new_filesystem.push(moved);
            }
            None => new_filesystem.push(*triple),
        }
    }

    new_filesystem.append(&mut open_space.into_iter().collect::<Vec<TripleId>>());
    new_filesystem.sort_by_key(|x| x.start());
    debug!("{:?}", new_filesystem);
    let parsed = parse_into_filesystem(
        &new_filesystem
            .iter()
            .flat_map(|x| x.as_vec())
            .collect::<Vec<isize>>(),
    );
    debug!("{:?}", parsed);

    let checksum = new_filesystem
        .iter()
        .enumerate()
        .filter(|(_, &c)| c.id() != -1)
        .fold(0, |acc, (_, &c)| {
            let mut sum = 0;
            for idx in c.range() {
                sum += idx * c.id() as usize;
            }
            debug!("{:?} -> {}", c, sum);
            sum + acc
        });
    info!("part 2: {}", checksum);
}
