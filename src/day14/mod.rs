use crate::solver::Solver;

#[derive(Copy, Clone, PartialEq)]
enum Object {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Object {
    fn is_rounded_rock(&self) -> bool {
        *self == Self::RoundedRock
    }

    fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl From<&Object> for char {
    fn from(value: &Object) -> Self {
        match *value {
            Object::RoundedRock => 'O',
            Object::CubeRock => '#',
            Object::Empty => '.',
        }
    }
}

struct Platform {
    platform: Vec<Vec<Object>>,
    height: usize,
    width: usize,
}

impl Platform {
    fn new(platform: Vec<Vec<Object>>) -> Self {
        let height = platform.len();
        let width = platform[0].len();
        Self {
            platform,
            height,
            width,
        }
    }

    fn tilt_north(&mut self) {
        let mut north_roll_per_column = vec![0; self.height];

        for i in 0..self.height {
            for j in 0..self.width {
                let maximum_roll = north_roll_per_column[j];
                let object = self.platform[i][j];
                if object.is_empty() {
                    continue;
                }

                let mut object_distance_from_north = i;
                if object.is_rounded_rock() {
                    if i > maximum_roll {
                        object_distance_from_north = maximum_roll;
                        self.platform[object_distance_from_north][j] = Object::RoundedRock;
                        self.platform[i][j] = Object::Empty;
                    }
                }

                north_roll_per_column[j] = object_distance_from_north + 1;
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut west_roll_per_row = vec![0; self.width];

        for j in 0..self.width {
            for i in 0..self.height {
                let maximum_roll = west_roll_per_row[i];
                let object = self.platform[i][j];
                if object.is_empty() {
                    continue;
                }

                let mut object_distance_from_west = j;
                if object.is_rounded_rock() {
                    if j > maximum_roll {
                        object_distance_from_west = maximum_roll;
                        self.platform[i][object_distance_from_west] = Object::RoundedRock;
                        self.platform[i][j] = Object::Empty;
                    }
                }

                west_roll_per_row[i] = object_distance_from_west + 1;
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut south_roll_per_column = vec![self.height - 1; self.height];

        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let maximum_roll = south_roll_per_column[j];
                let object = self.platform[i][j];
                if object.is_empty() {
                    continue;
                }

                let mut object_distance_from_south = i;
                if object.is_rounded_rock() {
                    if i < maximum_roll {
                        object_distance_from_south = maximum_roll;
                        self.platform[object_distance_from_south][j] = Object::RoundedRock;
                        self.platform[i][j] = Object::Empty;
                    }
                }

                south_roll_per_column[j] = object_distance_from_south - 1;
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut east_roll_per_row = vec![self.width - 1; self.width];

        for j in (0..self.width).rev() {
            for i in 0..self.height {
                let maximum_roll = east_roll_per_row[i];
                let object = self.platform[i][j];
                if object.is_empty() {
                    continue;
                }

                let mut object_distance_from_east = j;
                if object.is_rounded_rock() {
                    if j < maximum_roll {
                        object_distance_from_east = maximum_roll;
                        self.platform[i][object_distance_from_east] = Object::RoundedRock;
                        self.platform[i][j] = Object::Empty;
                    }
                }

                east_roll_per_row[i] = object_distance_from_east - 1;
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn calculate_total_load(&self) -> u64 {
        let mut total_load = 0;

        for i in 0..self.height {
            for j in 0..self.width {
                let object = self.platform[i][j];

                if object.is_rounded_rock() {
                    total_load += (self.height - i) as u64;
                }
            }
        }

        total_load
    }

    fn print(&self) {
        for row in &self.platform {
            println!("{}", row.iter().map(char::from).collect::<String>());
        }
        println!();
    }
}

pub struct Day14Solver {}

impl Solver for Day14Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day14/input.txt").unwrap();

        let mut platform = Platform::new(
            file.lines()
                .map(|line| line.chars().map(Object::from).collect())
                .collect(),
        );

        platform.tilt_north();

        println!(
            "The total load on the platform after tilting it North is {}",
            platform.calculate_total_load()
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day14/input.txt").unwrap();

        let mut platform = Platform::new(
            file.lines()
                .map(|line| line.chars().map(Object::from).collect())
                .collect(),
        );

        println!("Initial load: {}", platform.calculate_total_load());
        for i in 1..=1_000_000_000 {
            platform.spin_cycle();
            if i % 100_000 == 0 {
                println!("Load after {i} cycle: {}", platform.calculate_total_load());
            }
        }

        println!(
            "The total load on the platform after going through 1B cycles is {}",
            platform.calculate_total_load()
        );
    }
}
