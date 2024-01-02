use std::{ops::RangeInclusive, str::Lines};

use crate::solver::Solver;

const MIN_X: f64 = 200_000_000_000_000.0;
const MIN_Y: f64 = 200_000_000_000_000.0;
const MAX_X: f64 = 400_000_000_000_000.0;
const MAX_Y: f64 = 400_000_000_000_000.0;

#[derive(Debug)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hail {
    fn intersect_at_xy(&self, b: &Self) -> Option<(f64, f64)> {
        // The following equations were derived on a whiteboard. There are two variables
        // to solve, thus two equations are needed. With substitution, we end up finding
        // the time at which both hail stones intersect, which we can then use to find
        // their positions.
        let a = self;

        let ta_divisor = a.vx;
        if ta_divisor == 0.0 {
            return None;
        }

        let tb_divisor = b.vy - (a.vy * b.vx / a.vx);
        if tb_divisor == 0.0 {
            return None;
        }

        let tb_dividend = a.y - b.y + a.vy * (b.x - a.x) / a.vx;
        let tb = tb_dividend / tb_divisor;

        let ta_dividend = b.x - a.x + b.vx * tb;
        let ta = ta_dividend / ta_divisor;

        // We only care about future times.
        if ta < 0.0 || tb < 0.0 {
            return None;
        }

        let intersect_x = a.x + a.vx * ta;
        let intersect_y = a.y + a.vy * ta;

        Some((intersect_x, intersect_y))
    }
}

impl From<&str> for Hail {
    fn from(value: &str) -> Self {
        let (coordinates, velocities) = value.split_once(" @ ").unwrap();
        let coordinates: Vec<_> = coordinates.split(", ").collect();
        let velocities: Vec<_> = velocities.split(", ").collect();

        let x = coordinates[0].parse().unwrap();
        let y = coordinates[1].parse().unwrap();
        let z = coordinates[2].parse().unwrap();
        let vx = velocities[0].parse().unwrap();
        let vy = velocities[1].parse().unwrap();
        let vz = velocities[2].parse().unwrap();

        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

#[derive(Debug)]
struct HailCollider {
    hails: Vec<Hail>,
}

impl HailCollider {
    fn new(lines: Lines) -> Self {
        let hails = lines.map(Hail::from).collect();

        Self { hails }
    }

    fn count_xy_trajectories_intersections(
        &self,
        intersection_conditions: IntersectionConditions,
    ) -> u32 {
        let mut count = 0;
        let num_hails = self.hails.len();

        for hail_a_index in 0..num_hails {
            let hail_a = &self.hails[hail_a_index];

            for hail_b_index in (hail_a_index + 1)..num_hails {
                let hail_b = &self.hails[hail_b_index];

                match hail_a.intersect_at_xy(hail_b) {
                    Some((x, y))
                        if intersection_conditions.x.contains(&x)
                            && intersection_conditions.y.contains(&y) =>
                    {
                        count += 1
                    }
                    _ => {}
                }
            }
        }

        count
    }

    fn find_magic_rock_coordinates_sum(&self) -> i64 {
        // Just like for part 1, most of the work was done on a white board. The main goal was to
        // derive a system of linear equations, which was especially tricky because the equations
        // are themselves non-linear. I had to apply vectorial and linear algebra
        // (e.g. cross products and subtracting equations) to remove the non-linear terms.
        // The end result are these "magic" A and b matrices below, which we can solve for
        // using the nalgebra crate.
        //
        // Essentially, what we want is to find the intial position of the rock (in x,y,z)
        // and its velocity (in x,y,z) such that at it will collide with all of the hail stones.
        // These collisions can happen at different times for different stones, but contrarily
        // to part 1, the path intersection between the magic rock and a given hail stone must
        // happen at the same time. There are 6 variables to solve (the position and velocity
        // of the rock), but position equations include time, which add more variables as we
        // include more hail stones in our equation system. Fortunately, with some vectorial
        // manipulation we can get rid of the time from the system, which makes the whole system linear.
        // Though, that requires substracting the position equations from hail stone B and from hail stone A
        // to actually make it linear. As we need to solve 6 variables, we need 6 equations. For each
        // hail stone we have 3 equations (one for each dimension), but because we do some elimination
        // the equations from the 2nd stone are merged with the equations from the 1st stone to help us make
        // our system linear, so we actually need the data from 3 stones: the 2nd one is merged in the
        // 1st, and the 3rd is merged into the 1st. Thus, we end up with 6 equations. One requirement is that
        // each selected stone must be linearly independent, which the first 3 in our dataset are.
        let a = &self.hails[0];
        let b = &self.hails[1];
        let c = &self.hails[2];

        #[rustfmt::skip]
        let a_matrix = nalgebra::Matrix6::new(
          0.0, a.vz - b.vz, b.vy-a.vy, 0.0, b.z-a.z, a.y-b.y,
          b.vz-a.vz, 0.0, a.vx-b.vx,a.z-b.z,0.0,b.x-a.x,
          a.vy-b.vy,b.vx-a.vx,0.0,b.y-a.y,a.x-b.x,0.0,
          0.0,a.vz-c.vz,c.vy-a.vy,0.0,c.z-a.z,a.y-c.y,
          c.vz-a.vz,0.0,a.vx-c.vx,a.z-c.z,0.0,c.x-a.x,
          a.vy-c.vy,c.vx-a.vx,0.0,c.y-a.y,a.x-c.x,0.0,
        );
        let b_vector = nalgebra::Vector6::new(
            b.vy * b.z - b.vz * b.y - a.vy * a.z + a.vz * a.y,
            b.vz * b.x - b.vx * b.z - a.vz * a.x + a.vx * a.z,
            b.vx * b.y - b.vy * b.x - a.vx * a.y + a.vy * a.x,
            c.vy * c.z - c.vz * c.y - a.vy * a.z + a.vz * a.y,
            c.vz * c.x - c.vx * c.z - a.vz * a.x + a.vx * a.z,
            c.vx * c.y - c.vy * c.x - a.vx * a.y + a.vy * a.x,
        );

        // X is a 6-long vector. It contains, in order, the position of the rock (x, y, and z),
        // and its velocity (x, y, and z).
        let x = a_matrix
            .lu()
            .solve(&b_vector)
            .expect("Linear resolution failed");

        (x[0] + x[1] + x[2]).round() as i64
    }
}

struct IntersectionConditions {
    x: RangeInclusive<f64>,
    y: RangeInclusive<f64>,
}

pub struct Day24Solver {}

impl Solver for Day24Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day24/input.txt").unwrap();
        let hail_collider = HailCollider::new(file.lines());
        let num_intersections =
            hail_collider.count_xy_trajectories_intersections(IntersectionConditions {
                x: MIN_X..=MAX_X,
                y: MIN_Y..=MAX_Y,
            });

        println!("There were {num_intersections} intersections for the given input and the given time/space constraints.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day24/input.txt").unwrap();
        let hail_collider = HailCollider::new(file.lines());
        println!(
            "The sum of the coordinates of the magic rock hitting all hail stones is {}",
            hail_collider.find_magic_rock_coordinates_sum()
        );
    }
}
