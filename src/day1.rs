use std::{collections::HashMap, fs};

use tracing::info;

pub fn run(full: bool) {
    let file = crate::utils::get_input(1, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut left_list = Vec::<usize>::new();
    let mut right_list = Vec::<usize>::new();

    for line in lines.iter() {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse::<usize>().expect("unable to parse line"));
        right_list.push(right.parse::<usize>().expect("unable to parse line"));
    }

    left_list.sort();
    right_list.sort();

    let mut distance = 0;
    for (&left, &right) in left_list.iter().zip(right_list.iter()) {
        distance += right.abs_diff(left);
    }
    info!("part 1: {}", distance);
}

fn part2(lines: &Vec<&str>) {
    let mut left_list = Vec::<usize>::new();
    let mut right_side = HashMap::<usize, usize>::new();

    for line in lines.iter() {
        let (left, right) = line.split_once("   ").unwrap();
        let (left, right) = (
            left.parse::<usize>().expect("unable to parse line"),
            right.parse::<usize>().expect("unable to parse line"),
        );
        left_list.push(left);
        let count = right_side.get(&right).unwrap_or(&0);
        right_side.insert(right, count + 1);
    }
    let mut similarity_score = 0;
    for value in left_list.iter() {
        similarity_score += right_side.get(&value).unwrap_or(&0) * value;
    }
    info!("part 2: {}", similarity_score);
}
