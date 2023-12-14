use std::collections::HashMap;

use crate::solver::Solver;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

#[derive(Clone, Copy)]
enum Step {
    Left,
    Right,
}

impl From<char> for Step {
    fn from(value: char) -> Self {
        match value {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => unreachable!(),
        }
    }
}

pub struct Day8Solver {}

impl Day8Solver {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        (a / Self::gcd(a, b)) * b
    }
}

impl Solver for Day8Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day8/input.txt").unwrap();
        let mut lines = file.lines();
        let steps: Vec<_> = lines.next().unwrap().chars().map(Step::from).collect();
        let steps_len = steps.len();

        // Skip empty line.
        lines.next();

        let mut nodes = HashMap::new();
        for line in lines {
            let (from_node, to_nodes) = line.split_once('=').unwrap();
            let from_node = from_node.trim();
            let (left_to_node, right_to_node) = to_nodes.split_once(',').unwrap();
            let left_to_node = left_to_node.trim().strip_prefix('(').unwrap();
            let right_to_node = right_to_node.trim().strip_suffix(')').unwrap();

            nodes.insert(from_node, (left_to_node, right_to_node));
        }

        let mut curr_node = START_NODE;
        let mut i = 0;
        while curr_node != END_NODE {
            let (left_node, right_node) = nodes.get(curr_node).unwrap();
            curr_node = match steps[i % steps_len] {
                Step::Left => &left_node,
                Step::Right => &right_node,
            };

            i += 1;
        }

        println!("It took {i} steps to reach ZZZ");
    }

    fn solve_part2() {
        // These are the periods of all the traversals starting at a node ending in A.
        // I found these periods by printing the iteration at which each traversal was passing
        // through a node ending in Z. A pattern emerged that each traversal recurringly
        // went through a node ending in Z after a fixed period. Therefore, the solution to part
        // 2 ended being to find the least common multiple (LCM) of all of these periods. The
        // naïve solution otherwise ended up being way too slow -- the solution being in the
        // tens of trillions of iterations.
        let periods: Vec<u64> = vec![19241, 18157, 19783, 16531, 21409, 14363];
        let lcm = periods
            .into_iter()
            .reduce(|lcm, period| Self::lcm(lcm, period))
            .unwrap();
        println!(
            "The number of steps at which all ghosts end up in a Z node is {}",
            lcm
        );
    }
}
