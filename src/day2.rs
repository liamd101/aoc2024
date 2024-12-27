use tracing::info;

pub fn run(full: bool) {
    let file = crate::utils::get_input(2, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let safe_lines: isize = lines
        .clone()
        .iter_mut()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts
                .iter()
                .map(|part| part.parse::<isize>().expect("unable to parse line"))
                .collect::<Vec<isize>>()
        })
        .filter(|line| is_safe(line))
        .count()
        .try_into()
        .unwrap();
    info!("part 1: {}", safe_lines);
}

fn is_safe(line: &[isize]) -> bool {
    let mut line = line.to_owned();
    let mut prev = line.remove(0);
    let mut increasing = false;
    let mut decreasing = false;

    for &part in line.iter() {
        match prev - part {
            1..=3 => {
                increasing = true;
            }
            -3..=-1 => {
                decreasing = true;
            }
            _ => return false,
        }
        prev = part;
    }

    !(increasing & decreasing)
}

fn part2(lines: &Vec<&str>) {
    let safe_lines: isize = lines
        .clone()
        .iter_mut()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts
                .iter()
                .map(|part| part.parse::<isize>().expect("unable to parse line"))
                .collect::<Vec<isize>>()
        })
        .filter(|line| {
            (0..(line.len())).any(|skipped| {
                let mut tmp = line.clone();
                tmp.remove(skipped);
                is_safe(&tmp)
            })
        })
        .count()
        .try_into()
        .unwrap();
    info!("part 2: {}", safe_lines);
}
