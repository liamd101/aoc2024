use std::collections::HashSet;

use tracing::{debug, info};

use crate::day4::get_char_at_index;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn run(full: bool) {
    let file = crate::utils::get_input(12, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
}

fn part1(lines: &[&str]) {
    let mut price = 0;
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    let all_positions: Vec<(isize, isize)> = (0..lines[0].len())
        .flat_map(|x| (0..lines.len()).map(move |y| (x as isize, y as isize)))
        .collect();

    for &(x, y) in all_positions.iter() {
        if seen.contains(&(x, y)) {
            continue;
        }

        let mut this_plot = HashSet::default();
        let perimeter = compute_plot(lines, (x, y), &mut this_plot);
        price += this_plot.len() * perimeter;

        let c = get_char_at_index(lines, (x, y)).unwrap_or('-');
        debug!("{} -> {:?}", c, this_plot);
        debug!(
            "{} * {} = {}",
            this_plot.len(),
            perimeter,
            this_plot.len() * perimeter
        );

        seen.extend(this_plot);
    }

    info!("part 1: {}", price);
}

fn compute_plot(
    lines: &[&str],
    position: (isize, isize),
    mut current_plot: &mut HashSet<(isize, isize)>,
) -> usize {
    let mut perimeter = 0;
    if current_plot.contains(&position) {
        return perimeter;
    }
    current_plot.insert(position);

    let c = get_char_at_index(lines, position).unwrap();
    let (x, y) = position;

    let adjacent = DIRECTIONS
        .clone()
        .into_iter()
        .filter(|&(dx, dy)| get_char_at_index(lines, (x + dx, y + dy)).unwrap_or('\0') == c)
        .collect::<Vec<(isize, isize)>>();

    perimeter += adjacent
        .iter()
        .map(|&(dx, dy)| compute_plot(lines, (x + dx, y + dy), &mut current_plot))
        .fold(0, |acc, p| acc + p);

    perimeter + (4 - adjacent.len())
}
