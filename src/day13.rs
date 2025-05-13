use tracing::{debug, info};

use good_lp::{constraint, scip, variable, ProblemVariables, Solution, SolverModel};

use regex::Regex;

pub fn run(full: bool) {
    let input = crate::utils::get_input(13, full);
    let input: Vec<&str> = input.split("\n\n").collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &[&str]) {
    let mut tokens = 0;

    for machine in input.iter() {
        let parts: Vec<&str> = machine.splitn(3, '\n').collect();
        let (button_a, button_b, prize) = (parts[0], parts[1], parts[2]);

        tokens += solve_soe(button_a, button_b, prize);
    }

    info!("part 1: {}", tokens);
}

fn parse_movements(line: &str) -> Result<(f64, f64), &'static str> {
    let re = Regex::new(r"X(?:\+|=)(\d+),\s*Y(?:\+|=)(\d+)").unwrap();
    if let Some(caps) = re.captures(line) {
        let x: f64 = caps[1].parse().unwrap();
        let y: f64 = caps[2].parse().unwrap();
        Ok((x, y))
    } else {
        Err("invalid line")
    }
}

/// dest_x = (n_a * a_x) + (n_b * b_x)
/// dest_y = (n_a * a_y) + (n_b * b_y)
/// cost   = 3 * n_a     + n_b
///
/// f(n_a, n_b) = 3 * n_a + n_b
/// g(n_a, n_b) = n_a * a_x + n_b * b_x - dest_x = 0
/// h(n_a, n_b) = n_a * a_y + n_b * b_y - dest_y = 0
///
/// constrained optimization problem. can solve using lagrange multipliers lol

fn solve_soe(button_a: &str, button_b: &str, prize: &str) -> usize {
    let (a_x, a_y) = parse_movements(button_a).unwrap();
    let (b_x, b_y) = parse_movements(button_b).unwrap();
    let (prize_x, prize_y) = parse_movements(prize).unwrap();

    debug!("Button A: X+{}, Y+{}", a_x, a_y);
    debug!("Button B: X+{}, Y+{}", b_x, b_y);
    debug!("Prize: X={}, Y={}", prize_x, prize_y);

    let mut problem = ProblemVariables::new();
    let n_a = problem.add(variable().integer().min(0));
    let n_b = problem.add(variable().integer().min(0));

    let solution = problem
        .minimise(3 * n_a + n_b)
        .using(scip)
        .with(constraint!(
            n_a * a_x + n_b * b_x == prize_x
        ))
        .with(constraint!(
            n_a * a_y + n_b * b_y == prize_y
        ))
        .solve();

    match solution {
        Ok(solution) => {
            debug!("n_b: {}", solution.value(n_b));
            debug!("n_a: {}", solution.value(n_a));
            solution.eval(3 * n_a + n_b) as usize
        }
        Err(e) => {
            debug!("Error: {}", e);
            0
        }
    }
}

fn part2(input: &[&str]) {}
