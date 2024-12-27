use regex::Regex;
use std::fs;
use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(3, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = vec![];
    for line in lines {
        let mut result = 0;
        for cap in re.captures_iter(line) {
            let captures = cap.extract::<2>();
            let a = captures.1[0].parse::<isize>().unwrap();
            let b = captures.1[1].parse::<isize>().unwrap();
            debug!("{} * {} = {}", a, b, a * b);
            result += a * b;
        }
        results.push(result);
    }
    info!(
        "part 1: {:?}",
        results.iter().fold(0, |acc, &x| acc + x)
    )
}

fn part2(lines: &Vec<&str>) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut results = vec![];
    let mut disabled = false;
    for line in lines {
        let mut result = 0;
        for cap in re.captures_iter(line) {
            if cap.get(0).unwrap().as_str() == "don't()" {
                debug!("disabled");
                disabled = true;
            } else if cap.get(0).unwrap().as_str() == "do()" {
                debug!("enabled");
                disabled = false;
            } else {
                if disabled {
                    continue;
                }
                let a = cap.get(1).unwrap().as_str().parse::<isize>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<isize>().unwrap();
                debug!("{} * {} = {}", a, b, a * b);
                result += a * b;
            }
        }
        results.push(result);
    }
    info!(
        "part 2: {:?}",
        results.iter().fold(0, |acc, &x| acc + x)
    )
}
