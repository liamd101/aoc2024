use std::collections::HashSet;
use tracing::{debug, info};

use crate::day4::get_char_at_index;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn run(full: bool) {
    let file = crate::utils::get_input(10, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    let trail_heads: Vec<(isize, isize)> = (0..=lines[0].len())
        .flat_map(|x| (0..=lines.len()).map(move |y| (x as isize, y as isize)))
        .filter(|&position| get_char_at_index(lines, position) == Some('0'))
        .collect();

    debug!("trail_heads: {:?}", trail_heads);

    let count = trail_heads
        .iter()
        .map(|&trail_head| run_dfs::<HashSet<(isize, isize)>>(lines, trail_head))
        .fold(0, |acc, found| acc + found.len());

    info!("part 1: {}", count);
}

fn part2(lines: &[&str]) {
    let trail_heads: Vec<(isize, isize)> = (0..=lines[0].len())
        .flat_map(|x| (0..=lines.len()).map(move |y| (x as isize, y as isize)))
        .filter(|&position| get_char_at_index(lines, position) == Some('0'))
        .collect();

    debug!("trail_heads: {:?}", trail_heads);

    let count = trail_heads
        .iter()
        .map(|&trail_head| run_dfs::<Vec<(isize, isize)>>(lines, trail_head))
        .fold(0, |acc, found| acc + found.len());

    info!("part 1: {}", count);
}

/// runs DFS on the lines from a starting position
/// This will check all adjacent points to see if they are increasing by +1, and if so it will
/// recursively call DFS on that position.
/// Returns a HashSet of all indices of a 9 that are reachable from the starting position
fn run_dfs<C>(lines: &[&str], position: (isize, isize)) -> C
where
    C: Default + std::fmt::Debug + Extend<(isize, isize)> + IntoIterator<Item = (isize, isize)>,
{
    let height = (get_char_at_index(lines, position).unwrap_or('0') as u8) - b'0';
    if height == 9 {
        let mut container = C::default();
        container.extend([position]);
        return container;
    }
    let (x, y) = position;

    let mut container = C::default();
    for (dx, dy) in DIRECTIONS {
        let nx = x + dx;
        let ny = y + dy;
        let neighbor_height = (get_char_at_index(lines, (nx, ny)).unwrap_or('0') as u8) - b'0';

        if neighbor_height == height + 1 {
            // Recursively collect from the neighbor, then extend `container`
            container.extend(run_dfs::<C>(lines, (nx, ny)));
        }
    }

    if height == 0 {
        debug!("{:?} -> {:?}", position, container);
    }

    container
}
