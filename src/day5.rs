use std::{
    collections::{HashMap, HashSet},
    fs,
};
use tracing::{debug, info};

pub fn run(full: bool) {
    let file = crate::utils::get_input(5, full);
    part1(&file);
    part2(&file);
}

fn part1(input: &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut count: usize = 0;
    let mut g: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in rules.lines() {
        let (l, r) = line.split_once('|').unwrap();
        let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
        match g.get_mut(&r) {
            Some(nodes) => {
                nodes.insert(l);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(l);
                g.insert(r, set);
            }
        }
    }

    for update in updates.lines() {
        let update_order: Vec<usize> = update
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .collect();
        if is_valid(&update_order, &g).is_none() {
            debug!(
                "middle value: {}",
                update_order.get(update_order.len() / 2).unwrap()
            );
            count += update_order.get(update_order.len() / 2).unwrap();
        }
    }

    info!("part 1: {}", count);
}

fn is_valid(update: &[usize], g: &HashMap<usize, HashSet<usize>>) -> Option<(usize, usize)> {
    let mut valid_update = None;

    for (i, &x) in update.iter().enumerate() {
        for (j, &y) in update.iter().enumerate() {
            if (i < j) & (g.get(&x).unwrap_or(&HashSet::new()).contains(&y)) {
                valid_update = Some((i, j));
                break;
            }
        }
    }

    valid_update
}

fn part2(input: &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut count: usize = 0;
    let mut g: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in rules.lines() {
        let (l, r) = line.split_once('|').unwrap();
        let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
        match g.get_mut(&r) {
            Some(nodes) => {
                nodes.insert(l);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(l);
                g.insert(r, set);
            }
        }
    }

    for update in updates.lines() {
        let mut update_order: Vec<usize> = update
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .collect();

        if is_valid(&update_order, &g).is_none() {
            continue;
        }
        while let Some((i, j)) = is_valid(&update_order, &g) {
            update_order.swap(i, j);
        }

        debug!(
            "middle value: {}",
            update_order.get(update_order.len() / 2).unwrap()
        );
        count += update_order.get(update_order.len() / 2).unwrap();
    }

    info!("part 2: {}", count);
}
