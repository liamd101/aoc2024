use std::fs;
use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(7, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
}

fn part1(lines: &[&str]) {
    let mut count = 0;

    for &line in lines.iter() {
        let (answer, items) = line.split_once(": ").unwrap();
        let answer: usize = answer.parse().expect("parsing answer");
        let items: Vec<usize> = items
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        debug!("{}: {:?}", answer, items);
    }

    info!("part 1: {}", count);
}
