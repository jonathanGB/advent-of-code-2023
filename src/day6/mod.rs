use crate::solver::Solver;

pub struct Day6Solver {}

impl Day6Solver {
    fn find_number_of_ways_to_win((time, distance): (f64, f64)) -> i64 {
        let sqrt = f64::sqrt(time.powi(2) - 4.0 * distance);
        let first_solution = ((time - sqrt) / 2.0).ceil() as i64;
        let last_solution = ((time + sqrt) / 2.0).floor() as i64;

        // The count should include the first and last solutions, thus the +1.
        last_solution - first_solution + 1
    }
}

impl Solver for Day6Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day6/input.txt").unwrap();
        let mut lines = file.lines();
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();

        let (_, times) = time_line.split_once(':').unwrap();
        let (_, distances) = distance_line.split_once(':').unwrap();
        let times = times.trim();
        let distances = distances.trim();

        let product: i64 = std::iter::zip(times.split_whitespace(), distances.split_whitespace())
            .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
            .map(Self::find_number_of_ways_to_win)
            .product();

        println!("The product of all ways to win this game is {product}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day6/input.txt").unwrap();
        let mut lines = file.lines();
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();

        let (_, times) = time_line.split_once(':').unwrap();
        let (_, distances) = distance_line.split_once(':').unwrap();

        let merged_times: String = times.split_whitespace().collect();
        let merged_distances: String = distances.split_whitespace().collect();
        let merged_times: f64 = merged_times.parse().unwrap();
        let merged_distances: f64 = merged_distances.parse().unwrap();

        let num_of_ways_to_win = Self::find_number_of_ways_to_win((merged_times, merged_distances));
        println!("The number of ways to win this merged race is {num_of_ways_to_win}");
    }
}
