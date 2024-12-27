use std::collections::HashSet;
use std::result::Result;

use crate::day4::get_char_at_index;
use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(6, full);
    let lines: Vec<&str> = file.lines().collect();
    part1(&lines, (-1, -1));
    part2(&lines);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn from_char(c: char) -> Result<Direction, &'static str> {
        match c {
            'v' => Ok(Direction::Down),
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            '^' => Ok(Direction::Up),
            _ => Err("invalid char"),
        }
    }

    fn move_direction(&self, (x, y): (isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.vector();
        (x + dx, y + dy)
    }
}

fn part1(lines: &[&str], placed_obstacle: (isize, isize)) -> bool {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut agent_location: Option<(isize, isize)> = None;
    let mut agent_direction: Direction = Direction::Up;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'v' | '^' | '>' | '<' => {
                    agent_location = Some((x as isize, y as isize));
                    agent_direction = Direction::from_char(c).expect("invalid char");
                }
                _ => continue,
            };
        }
    }

    let size = lines.len() * lines[0].len();
    let mut len_path = 0;
    let mut hit_direction: Vec<Direction> = vec![];

    while let Some((x, y)) = agent_location {
        if len_path + 1 > size * 2 {
            return true;
        }

        let new_location = agent_direction.move_direction((x, y));

        // if we have bumbed into the obstacle and we are moving in the same direction as any hit,
        // return true (this must be a cycle)
        if (new_location == placed_obstacle) & (hit_direction.contains(&agent_direction)) {
            return true;
        } else if new_location == placed_obstacle {
            // add the direction we hit the obstacle from
            hit_direction.push(agent_direction);
            agent_direction = agent_direction.rotate();
            continue;
        }

        match get_char_at_index(lines, new_location) {
            Some('#') => agent_direction = agent_direction.rotate(),
            Some(_) => {
                visited.insert((x, y));
                agent_location = Some(new_location);
            }
            None => {
                visited.insert((x, y));
                agent_location = None
            }
        }
        len_path += 1;
    }

    info!("part 1: {}", visited.len());

    false
}

fn part2(lines: &[&str]) {
    let all_positions: Vec<(isize, isize)> = (0..=lines[0].len())
        .flat_map(|x| (0..=lines.len()).map(move |y| (x as isize, y as isize)))
        .collect();

    let placed_obstacles = all_positions
        .into_iter()
        .filter(|&(x, y)| part1(lines, (x, y)))
        .collect::<HashSet<(isize, isize)>>();

    debug!("{:?}", placed_obstacles);

    info!("part 2: {}", placed_obstacles.len());
}
