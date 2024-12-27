use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(4, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    let mut count = 0;
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let width = lines[0].len() as isize;
    let height = lines.len() as isize;

    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != 'X' {
                continue;
            }
            for (dx, dy) in directions.iter() {
                let mut x = i as isize;
                let mut y = j as isize;
                let mut found = false;
                for num in 1..4 {
                    x += dx;
                    y += dy;
                    if (x < 0) | (y < 0) | (x >= width) | (y >= height) {
                        continue;
                    }
                    let c = lines[y as usize].chars().nth(x as usize).unwrap();
                    match num {
                        1 => {
                            if c != 'M' {
                                break;
                            }
                        }
                        2 => {
                            if c != 'A' {
                                break;
                            }
                        }
                        3 => {
                            if c == 'S' {
                                found = true;
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if found {
                    debug!("({}, {})", j, i);
                    count += 1;
                }
            }
        }
    }
    info!("part 1: {}", count)
}

fn part2(lines: &[&str]) {
    let mut count = 0;
    let ms = b'M' + b'S';

    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }
            let i = i as isize;
            let j = j as isize;

            let (top_left, bottom_right) = ((i - 1, j - 1), (i + 1, j + 1));
            let (top_right, bottom_left) = ((i + 1, j - 1), (i - 1, j + 1));

            if ((get_char_at_index(lines, top_left).unwrap_or(' ') as u8)
                + (get_char_at_index(lines, bottom_right).unwrap_or(' ') as u8)
                == ms)
                & ((get_char_at_index(lines, top_right).unwrap_or(' ') as u8)
                    + (get_char_at_index(lines, bottom_left).unwrap_or(' ') as u8)
                    == ms)
            {
                debug!("found ({}, {})", i, j);
                count += 1;
            }
        }
    }
    info!("part 2: {}", count);
}

pub fn get_char_at_index(lines: &[&str], (x, y): (isize, isize)) -> Option<char> {
    let width = lines[0].len() as isize;
    let height = lines.len() as isize;

    if (x < 0) | (y < 0) | (x >= width) | (y >= height) {
        return None;
    }

    lines.get(y as usize).unwrap().chars().nth(x as usize)
}
