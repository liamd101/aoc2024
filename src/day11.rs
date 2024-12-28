use std::collections::HashMap;

use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(11, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(lines[0]);
    part2(lines[0]);
}

fn part1(line: &str) {
    let mut stones = line
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .map(|x| (x, 1))
        .collect::<HashMap<isize, isize>>();

    let mut stone_mapping: HashMap<isize, (isize, isize)> = HashMap::new();

    debug!("{:?}", stones);

    for _ in 0..25 {
        stones = blink_optimized(stones, &mut stone_mapping);
        debug!("stones: {:?}", stones);
        debug!("stone mapping: {:?}", stone_mapping);
    }

    let count = stones.iter().fold(0, |acc, (_, v)| acc + v);

    info!("part 1: {}", count);
}

/// takes the following:
///   - map of stone -> what it produces
///   - map of stone -> number of those stones
///
/// returns the following:
///   - updated map of stone -> what it produces (if it was not in the input)
///     - this can just be a mutable ref
///   - map of stone -> number of those stones (updated)
///     - this should probably just be a new one?
fn blink_optimized(
    current_stones: HashMap<isize, isize>,
    stone_mapping: &mut HashMap<isize, (isize, isize)>,
) -> HashMap<isize, isize> {
    let mut new_stones: HashMap<isize, isize> = HashMap::new();

    for (&stone, &count) in current_stones.iter() {
        let (stone1, stone2) = match stone_mapping.get(&stone) {
            Some(&stones) => stones,
            None => {
                let stones = blink(stone);
                stone_mapping.insert(stone, stones);
                stones
            }
        };

        let new_stone1_count = new_stones.get(&stone1).unwrap_or(&0);
        new_stones.insert(stone1, new_stone1_count + count);

        if stone2 != -1 {
            let new_stone2_count = new_stones.get(&stone2).unwrap_or(&0);
            new_stones.insert(stone2, new_stone2_count + count);
        }
    }

    new_stones
}

fn blink(stone: isize) -> (isize, isize) {
    if stone == 0 {
        (1, -1)
    } else if stone == 1 {
        (2024, -1)
    } else if let Some(stones) = split_stone(stone) {
        stones
    } else {
        (stone * 2024, -1)
    }
}

fn split_stone(stone: isize) -> Option<(isize, isize)> {
    let stone = format!("{}", stone);
    if stone.len() % 2 == 0 {
        let (stone1, stone2) = stone.split_at(stone.len() / 2);
        let stone1 = stone1.parse::<isize>().unwrap();
        let stone2 = stone2.parse::<isize>().unwrap();
        Some((stone1, stone2))
    } else {
        None
    }
}

fn part2(line: &str) {
    let mut stones = line
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .map(|x| (x, 1))
        .collect::<HashMap<isize, isize>>();

    let mut stone_mapping: HashMap<isize, (isize, isize)> = HashMap::new();

    debug!("{:?}", stones);

    for _ in 0..75 {
        stones = blink_optimized(stones, &mut stone_mapping);
        debug!("stones: {:?}", stones);
        debug!("stone mapping: {:?}", stone_mapping);
    }

    let count = stones.iter().fold(0, |acc, (_, v)| acc + v);

    info!("part 1: {}", count);
}
