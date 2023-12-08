use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day4Solver {}
impl Solver for Day4Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day4/input.txt").unwrap();

        let mut total_points = 0;
        for line in file.lines() {
            let (_, all_numbers) = line.split_once(":").unwrap();
            let (winning_numbers, your_numbers) = all_numbers.split_once("|").unwrap();
            let winning_numbers: HashSet<_> = winning_numbers
                .trim()
                .split(" ")
                // Splitting can result in empty strings as entries. Filter them out.
                .filter(|number| !number.is_empty())
                .collect();
            let your_numbers: HashSet<_> = your_numbers
                .trim()
                .split(" ")
                // Splitting can result in empty strings as entries. Filter them out.
                .filter(|number| !number.is_empty())
                .collect();
            let num_match = winning_numbers
                .intersection(&your_numbers)
                .collect::<Vec<_>>()
                .len() as u32;

            if num_match > 0 {
                total_points += 2u32.pow(num_match - 1);
            }
        }

        println!("The total number of points is {total_points}.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day4/input.txt").unwrap();

        let mut wins_per_card = Vec::new();
        for line in file.lines() {
            let (_, all_numbers) = line.split_once(":").unwrap();
            let (winning_numbers, your_numbers) = all_numbers.split_once("|").unwrap();
            let winning_numbers: HashSet<_> = winning_numbers
                .trim()
                .split(" ")
                // Splitting can result in empty strings as entries. Filter them out.
                .filter(|number| !number.is_empty())
                .collect();
            let your_numbers: HashSet<_> = your_numbers
                .trim()
                .split(" ")
                // Splitting can result in empty strings as entries. Filter them out.
                .filter(|number| !number.is_empty())
                .collect();
            let num_match = winning_numbers
                .intersection(&your_numbers)
                .collect::<Vec<_>>()
                .len();

            wins_per_card.push(num_match);
        }

        let mut scratchcards_copies = vec![1; wins_per_card.len()];
        for (i, win_per_card) in wins_per_card.into_iter().enumerate() {
            let scratchard_copies = scratchcards_copies[i];

            for j in (i + 1)..=(i + win_per_card) {
                scratchcards_copies[j] += scratchard_copies;
            }
        }

        println!(
            "The total number of scratchcards is {:?}",
            scratchcards_copies.into_iter().sum::<usize>()
        );
    }
}
