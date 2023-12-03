use crate::solver::Solver;

const NUM_RED_CUBES: u32 = 12;
const NUM_GREEN_CUBES: u32 = 13;
const NUM_BLUE_CUBES: u32 = 14;

pub struct Day2Solver {}
impl Solver for Day2Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day2/input.txt").unwrap();

        let mut total_valid_game_ids = 0;
        'game: for line in file.lines() {
            let (game_prefix, plays) = line.split_once(':').unwrap();
            let (_, game_id) = game_prefix.split_once(' ').unwrap();
            let game_id: u32 = game_id.parse().unwrap();

            for play in plays.split(';') {
                for draw in play.split(',') {
                    let (num_drawn, colour) = draw.trim().split_once(' ').unwrap();
                    let num_drawn: u32 = num_drawn.parse().unwrap();

                    let max_drawn = match colour {
                        "red" => NUM_RED_CUBES,
                        "green" => NUM_GREEN_CUBES,
                        "blue" => NUM_BLUE_CUBES,
                        _ => unreachable!(),
                    };

                    if num_drawn > max_drawn {
                        continue 'game;
                    }
                }
            }

            total_valid_game_ids += game_id;
        }

        println!("The sum of valid game IDs is: {total_valid_game_ids}.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day2/input.txt").unwrap();

        let mut sum_game_power_sets = 0;
        for line in file.lines() {
            let (_, plays) = line.split_once(':').unwrap();

            let mut min_red_cubes = 0;
            let mut min_green_cubes = 0;
            let mut min_blue_clubes = 0;
            for play in plays.split(';') {
                for draw in play.split(',') {
                    let (num_drawn, colour) = draw.trim().split_once(' ').unwrap();
                    let num_drawn: u32 = num_drawn.parse().unwrap();

                    let min_cubes = match colour {
                        "red" => &mut min_red_cubes,
                        "green" => &mut min_green_cubes,
                        "blue" => &mut min_blue_clubes,
                        _ => unreachable!(),
                    };

                    *min_cubes = (*min_cubes).max(num_drawn);
                }
            }

            // Definition of a game power set is the multiplication of all colours.
            sum_game_power_sets += min_red_cubes * min_green_cubes * min_blue_clubes;
        }

        println!("The sum of power sets is: {sum_game_power_sets}.");
    }
}
