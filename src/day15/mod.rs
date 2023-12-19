use std::collections::VecDeque;

use crate::solver::Solver;

const NUM_BOXES: usize = 256;

fn hash(string: &[u8]) -> usize {
    let mut output: usize = 0;
    for byte in string {
        output += *byte as usize;
        output *= 17;
        output %= 256;
    }

    output
}

#[derive(Clone, Debug, Default)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

#[derive(Debug)]
enum Op<'a> {
    Equal(Lens<'a>),
    Min(&'a [u8]),
}

impl<'a> Op<'a> {
    fn new(instruction: &'a str) -> Self {
        if instruction.ends_with('-') {
            Self::Min(instruction.strip_suffix('-').unwrap().as_bytes())
        } else {
            let (label, focal_length) = instruction.split_once('=').unwrap();
            let label = label.as_bytes();
            let focal_length = focal_length.parse().unwrap();
            Self::Equal(Lens {
                label,
                focal_length,
            })
        }
    }

    fn get_box_index(&self) -> usize {
        let label = match self {
            Self::Equal(Lens { label, .. }) => label,
            Self::Min(label) => label,
        };

        hash(label)
    }
}

#[derive(Clone, Debug, Default)]
struct Box<'a> {
    lenses: VecDeque<Lens<'a>>,
}

impl<'a> Box<'a> {
    fn apply_eq(&mut self, label: &'a [u8], focal_length: u8) {
        if let Some(lens_index) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses[lens_index].focal_length = focal_length;
        } else {
            self.lenses.push_back(Lens {
                label,
                focal_length,
            });
        }
    }

    fn apply_min(&mut self, label: &[u8]) {
        if let Some(lens_index) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses.remove(lens_index);
        }
    }
}

type Boxes<'a> = [Box<'a>; NUM_BOXES];

pub struct Day15Solver {}

impl Day15Solver {
    fn solve_part2_with_file(file: &str) -> usize {
        let mut boxes: Boxes = std::array::from_fn(|_| Box::default());

        for op in file.split(',').map(Op::new) {
            let box_index = op.get_box_index();
            let selected_box = &mut boxes[box_index];

            match op {
                Op::Equal(Lens {
                    label,
                    focal_length,
                }) => {
                    selected_box.apply_eq(label, focal_length);
                }
                Op::Min(label) => {
                    selected_box.apply_min(label);
                }
            }
        }

        let mut focusing_power = 0;
        for (box_index, curr_box) in boxes.into_iter().enumerate() {
            for (lens_index, lens) in curr_box.lenses.into_iter().enumerate() {
                focusing_power += (box_index + 1) * (lens_index + 1) * lens.focal_length as usize;
            }
        }

        focusing_power
    }
}

impl Solver for Day15Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day15/input.txt").unwrap();
        let sum: usize = file
            .split(',')
            .map(|instruction| instruction.as_bytes())
            .map(hash)
            .sum();
        println!("{}", sum);
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day15/input.txt").unwrap();

        let focusing_power = Self::solve_part2_with_file(&file);
        println!("The focusing power of all these boxes is {focusing_power}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day15/input.txt").unwrap();

        b.iter(|| Day15Solver::solve_part2_with_file(&file));
    }
}
