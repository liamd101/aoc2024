use std::collections::HashSet;

use tracing::{debug, info};

use crate::day4::get_char_at_index;
use crate::day6::Direction;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn run(full: bool) {
    let file = crate::utils::get_input(12, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
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
    current_plot: &mut HashSet<(isize, isize)>,
) -> usize {
    let mut perimeter = 0;
    if current_plot.contains(&position) {
        return perimeter;
    }
    current_plot.insert(position);

    let c = get_char_at_index(lines, position).unwrap();
    let (x, y) = position;

    let adjacent = DIRECTIONS
        .into_iter()
        .filter(|&(dx, dy)| get_char_at_index(lines, (x + dx, y + dy)).unwrap_or('\0') == c)
        .collect::<Vec<(isize, isize)>>();

    perimeter += adjacent
        .iter()
        .map(|&(dx, dy)| compute_plot(lines, (x + dx, y + dy), current_plot))
        .sum::<usize>();

    perimeter + (4 - adjacent.len())
}

fn part2(lines: &[&str]) {
    let mut price = 0;
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    let all_positions: Vec<(isize, isize)> = (0..lines[0].len())
        .flat_map(|x| (0..lines.len()).map(move |y| (x as isize, y as isize)))
        .collect();

    for &(x, y) in all_positions.iter() {
        if seen.contains(&(x, y)) {
            continue;
        }
        let c = get_char_at_index(lines, (x, y)).unwrap_or('-');
        if c != 'M' {
            continue;
        }

        let mut this_plot: HashSet<(isize, isize)> = HashSet::default();
        compute_plot(lines, (x, y), &mut this_plot);
        let sides = num_sides(&this_plot);
        price += this_plot.len() * sides;

        debug!(
            "{} -> {} * {} = {}",
            c,
            this_plot.len(),
            sides,
            this_plot.len() * sides
        );

        seen.extend(this_plot);
    }

    info!("part 2: {}", price);
}

/// Computes the number of sides that a generic "plot" of land has
/// This is computed by first filtering all of the spaces in the plot that are on the edge
/// Then, we choose an arbitrary direction and start traversing along the edge
/// Every time we change direction (indicating a corner), we increment counter by 1
/// We then return the number of times that we changed direction, i.e. the number of sides
fn num_sides(plot: &HashSet<(isize, isize)>) -> usize {
    let edge_spaces: HashSet<(isize, isize)> = plot
        .clone()
        .into_iter()
        .filter(|&(x, y)| {
            !DIRECTIONS
                .iter()
                .all(|&(dx, dy)| plot.contains(&(x + dx, y + dy)))
        })
        .collect();

    debug!("edges: {:?}", edge_spaces);

    let mut num_sides = 0;

    // choose arbitrary direction to start. we need to keep track of this to know when the cycle
    // ends (i.e. when have we reached the same spot in the same direction)
    let start_position = edge_spaces.iter().next().unwrap().clone();
    let start_direction = DIRECTIONS
        .clone()
        .into_iter()
        .find(|&(dx, dy)| edge_spaces.contains(&(start_position.0 + dx, start_position.1 + dy)))
        .unwrap();
    let start_direction = Direction::from_tuple(start_direction).expect("receive valid direction");

    debug!("start position: {:?}", start_position);
    debug!("start direction: {:?}", start_direction);

    let mut direction = start_direction;
    let mut position = start_position;

    // hmmm
    loop {
        if num_sides > 15 {
            break;
        }
        if num_sides > 0 && (direction == start_direction) && (position == start_position) {
            break;
        }

        let mut next_direction = start_direction;
        let mut next_position = next_direction.move_direction(position);

        while !plot.contains(&next_position) {
            next_direction = next_direction.rotate();
            next_position = next_direction.move_direction(position);
        }

        if next_direction != direction {
            num_sides += 1;
            debug!("corner at {:?}", next_position);
            direction = next_direction;
        } else {
            debug!("straight at {:?} moving {:?}", position, direction);
        }
        position = next_position;
    }

    num_sides
}
