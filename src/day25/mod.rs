/// Solution heavily inspired by
/// https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/kfoynua/.
use hashbrown::{HashMap, HashSet};
use std::{collections::VecDeque, str::Lines};

use crate::solver::Solver;

#[derive(Debug)]
struct Components {
    components: Vec<Vec<usize>>,
}

impl Components {
    fn new(lines: Lines) -> Self {
        let mut name_to_id_mapping = HashMap::new();
        let mut components = Vec::new();

        for line in lines {
            let (src, dsts) = line.split_once(": ").unwrap();
            let src_id = *name_to_id_mapping.entry(src).or_insert_with(|| {
                let id = components.len();
                components.push(Vec::new());
                id
            });

            dsts.split_whitespace().for_each(|dst| {
                let dst_id = *name_to_id_mapping.entry(dst).or_insert_with(|| {
                    let id = components.len();
                    components.push(Vec::new());
                    id
                });

                components[src_id].push(dst_id);
                components[dst_id].push(src_id);
            });
        }

        Self { components }
    }

    fn find_two_partition_lengths_in_3_cuts(&self) -> (usize, usize) {
        let arbitrary_node = 0;
        let node_a = self.find_furthest_node(arbitrary_node);
        let node_b = self.find_furthest_node(node_a);

        // Apply 3 cuts. All 3 cuts should not partition the graph.
        let mut edges_in_visited_paths = HashSet::new();
        for _ in 0..3 {
            assert!(self
                .find_path_or_return_partition_size(node_a, node_b, &mut edges_in_visited_paths)
                .is_none());
        }

        // The 4th and last cut should partition the graph.
        let partition_1_size = self
            .find_path_or_return_partition_size(node_a, node_b, &mut edges_in_visited_paths)
            .expect("The 3 previous paths did not partition the graph");

        (partition_1_size, self.components.len() - partition_1_size)
    }

    fn find_furthest_node(&self, from: usize) -> usize {
        let mut visited = vec![false; self.components.len()];
        let mut to_visits = VecDeque::from([from]);
        let mut last = from;

        while let Some(to_visit) = to_visits.pop_front() {
            if visited[to_visit] {
                continue;
            }

            last = to_visit;
            visited[last] = true;

            for neighbour in &self.components[to_visit] {
                if visited[*neighbour] {
                    continue;
                }

                to_visits.push_back(*neighbour);
            }
        }

        last
    }

    fn find_path_or_return_partition_size(
        &self,
        node_a: usize,
        node_b: usize,
        edges_in_visited_paths: &mut HashSet<(usize, usize)>,
    ) -> Option<usize> {
        let mut visited_nodes = vec![false; self.components.len()];
        // Pairs of next node with the index of the edge that linked to them. The latter will be
        // important when we walk the path back. We start with MAX as a sentinel value.
        let mut to_visits = VecDeque::from([(node_a, usize::MAX)]);
        // Pairs of edges and the index of the previous edge that linked to it.
        let mut prevs = Vec::new();
        // Count of the connected nodes from `node_a`.
        let mut count = 0;

        while let Some((to_visit, mut prev)) = to_visits.pop_front() {
            if visited_nodes[to_visit] {
                continue;
            }

            count += 1;
            visited_nodes[to_visit] = true;

            if to_visit == node_b {
                // Walk the path back.
                while prev != usize::MAX {
                    let (edge, prev_prev) = prevs[prev];
                    edges_in_visited_paths.insert(edge);
                    prev = prev_prev;
                }

                return None;
            }

            for neighbour in &self.components[to_visit] {
                // We order the edge to have a consistent representation of an edge (this is an undirected graph).
                let edge = if to_visit < *neighbour {
                    (to_visit, *neighbour)
                } else {
                    (*neighbour, to_visit)
                };

                if visited_nodes[*neighbour] || edges_in_visited_paths.contains(&edge) {
                    continue;
                }

                to_visits.push_back((*neighbour, prevs.len()));
                prevs.push((edge, prev));
            }
        }

        Some(count)
    }
}

pub struct Day25Solver {}

impl Solver for Day25Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day25/input.txt").unwrap();
        let components = Components::new(file.lines());
        let (partition1_len, partition2_len) = components.find_two_partition_lengths_in_3_cuts();
        println!(
            "The sizes of the two partitions are {} x {} = {}",
            partition1_len,
            partition2_len,
            partition1_len * partition2_len
        );
    }

    fn solve_part2() {
        // There is no problem 2.
    }
}
