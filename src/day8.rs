use crate::day4::get_char_at_index;
use std::collections::{HashMap, HashSet};

use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(8, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            match antennas.get_mut(&c) {
                Some(coordinates) => coordinates.push((x as isize, y as isize)),
                None => {
                    let coordinates = vec![(x as isize, y as isize)];
                    antennas.insert(c, coordinates);
                }
            }
        }
    }

    for (_, coordinates) in antennas.iter() {
        for (i, (x1, y1)) in coordinates.iter().enumerate() {
            for (j, (x2, y2)) in coordinates.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dx = x1 - x2;
                let dy = y1 - y2;
                if get_char_at_index(lines, (x1 + dx, y1 + dy)).is_some() {
                    antinodes.insert((x1 + dx, y1 + dy));
                }
                if get_char_at_index(lines, (x2 - dx, y2 - dy)).is_some() {
                    antinodes.insert((x2 - dx, y2 - dy));
                }
            }
        }
    }

    let mut new_lines: Vec<String> = lines.iter().map(|&s| String::from(s).to_owned()).collect();
    for (x, y) in antinodes.clone().into_iter() {
        let tmp = new_lines.get_mut(y as usize).unwrap();
        tmp.replace_range((x as usize)..(x as usize) + 1, "#");
        new_lines[y as usize] = tmp.clone();
    }
    debug!("\n{}", new_lines.join("\n"));

    info!("part 1: {}", antinodes.len());
}

fn part2(lines: &Vec<&str>) {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            match antennas.get_mut(&c) {
                Some(coordinates) => coordinates.push((x as isize, y as isize)),
                None => {
                    let coordinates = vec![(x as isize, y as isize)];
                    antennas.insert(c, coordinates);
                }
            }
        }
    }

    for (_, coordinates) in antennas.iter() {
        for (i, &(x1, y1)) in coordinates.iter().enumerate() {
            for (j, &(x2, y2)) in coordinates.iter().enumerate() {
                if i == j {
                    continue;
                }
                antinodes.insert((x1, y1));
                antinodes.insert((x2, y2));
                let dx = x1 - x2;
                let dy = y1 - y2;
                let mut tmp = (x1 + dx, y1 + dy);
                while get_char_at_index(lines, tmp).is_some() {
                    antinodes.insert(tmp);
                    tmp.0 += dx;
                    tmp.1 += dy;
                }
                let mut tmp = (x2 - dx, y2 - dy);
                while get_char_at_index(lines, tmp).is_some() {
                    antinodes.insert(tmp);
                    tmp.0 -= dx;
                    tmp.1 -= dy;
                }
            }
        }
    }

    let mut new_lines: Vec<String> = lines.iter().map(|&s| String::from(s).to_owned()).collect();
    for &(x, y) in antinodes.iter() {
        let tmp = new_lines.get_mut(y as usize).unwrap();
        if tmp.chars().nth(x as usize).unwrap() != '.' {
            continue;
        }
        tmp.replace_range((x as usize)..(x as usize) + 1, "#");
        new_lines[y as usize] = tmp.clone();
    }
    debug!("\n{}", new_lines.join("\n"));

    info!("part 2: {}", antinodes.len());
}
