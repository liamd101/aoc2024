use std::collections::{BTreeSet, VecDeque};

use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(7, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    let mut count = 0;

    for &line in lines.iter() {
        let (answer, items) = line.split_once(": ").unwrap();
        let answer: usize = answer.parse().expect("parsing answer");
        let mut items: VecDeque<usize> = items
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        let mut possible_values: BTreeSet<usize> = BTreeSet::new();
        possible_values.insert(0);
        while let Some(item) = items.pop_front() {
            let mut new_values = BTreeSet::new();

            for value in possible_values.iter() {
                new_values.insert(*value * item);
                new_values.insert(*value + item);
            }

            possible_values = new_values;
        }

        debug!("{}: {:?}", answer, possible_values);
        if possible_values.contains(&answer) {
            count += answer;
        }
    }

    info!("part 1: {}", count);
}

fn part2(lines: &[&str]) {
    let mut count = 0;

    for &line in lines.iter() {
        let (answer, items) = line.split_once(": ").unwrap();
        let answer: usize = answer.parse().expect("parsing answer");
        let mut items: VecDeque<usize> = items
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        let mut possible_values: BTreeSet<usize> = BTreeSet::new();
        possible_values.insert(0);
        while let Some(item) = items.pop_front() {
            let mut new_values = BTreeSet::new();

            for value in possible_values.iter() {
                new_values.insert(*value * item);
                new_values.insert(*value + item);
                let concatenated = format!("{}{}", value, item).parse::<usize>().unwrap();
                new_values.insert(concatenated);
            }

            possible_values = new_values;
        }

        debug!("{}: {:?}", answer, possible_values);
        if possible_values.contains(&answer) {
            count += answer;
        }
    }

    info!("part 1: {}", count);
}
