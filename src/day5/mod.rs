use crate::solver::Solver;
use std::ops::Range;
use std::str::Lines;
use std::sync::mpsc::channel;

#[derive(Clone, Debug)]
struct ConversionMap {
    // Start bound is inclusive.
    start_bound: i64,
    // End bound is exclusive.
    end_bound: i64,

    // If added to a source value within the range above, this will convert the source value
    // to the correct destination value.
    conversion: i64,
}

impl ConversionMap {
    fn convert_if_inbound(&self, key: i64) -> Option<i64> {
        if key >= self.start_bound && key < self.end_bound {
            Some(key + self.conversion)
        } else {
            None
        }
    }
}

pub struct Day5Solver {}

impl Day5Solver {
    fn parse_seeds_list_part1(lines: &mut Lines) -> Vec<i64> {
        let seeds_line = lines.next().unwrap();
        let (_, seeds) = seeds_line.split_once(": ").unwrap();
        let parsed_seeds = seeds.split(' ').flat_map(str::parse::<i64>).collect();

        // Skip the following empty line.
        lines.next();

        parsed_seeds
    }

    fn parse_seeds_list_part2(lines: &mut Lines) -> Vec<Range<i64>> {
        let seeds_line = lines.next().unwrap();
        let (_, seeds) = seeds_line.split_once(": ").unwrap();
        let parsed_seeds_and_ranges: Vec<_> =
            seeds.split(' ').flat_map(str::parse::<i64>).collect();

        // Process seed ranges in pairs. Even entries are a seed, and the subsequent
        // odd entry is the range the preceding seed applies to.
        let mut i = 0;
        let parsed_seeds_and_ranges_len = parsed_seeds_and_ranges.len();
        let mut parsed_seed_ranges = Vec::new();
        while (i + 1) < parsed_seeds_and_ranges_len {
            let parsed_seed_start = parsed_seeds_and_ranges[i];
            let range = parsed_seeds_and_ranges[i + 1];
            let parsed_seed_end = parsed_seed_start + range;
            parsed_seed_ranges.push(parsed_seed_start..parsed_seed_end);

            i += 2;
        }

        // Skip the following empty line.
        lines.next();

        parsed_seed_ranges
    }

    fn parse_category_conversion_maps(lines: &mut Lines) -> Vec<ConversionMap> {
        let mut category_conversion_maps = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let (destination_start, source_start_and_range) = line.split_once(' ').unwrap();
            let (source_start, range) = source_start_and_range.split_once(' ').unwrap();

            let destination_start: i64 = destination_start.parse().unwrap();
            let source_start: i64 = source_start.parse().unwrap();
            let range: i64 = range.parse().unwrap();

            let start_bound = source_start;
            let end_bound = start_bound + range;
            let conversion = destination_start - source_start;

            category_conversion_maps.push(ConversionMap {
                start_bound,
                end_bound,
                conversion,
            });
        }

        category_conversion_maps.sort_by(|a, b| a.start_bound.cmp(&b.start_bound));
        category_conversion_maps
    }

    fn find_seed_location_number(
        seed: i64,
        categories_conversion_maps: &Vec<Vec<ConversionMap>>,
    ) -> i64 {
        let mut source = seed;
        // For each category map, we apply binary search to find the matching conversation map.
        // If there isn't any, then the mapping is the identity function (i.e. X => X), therefore
        // we do nothing.
        for category_conversion_map in categories_conversion_maps {
            let mut low = 0;
            let mut high = category_conversion_map.len() - 1;

            while low <= high {
                let mid = low + ((high - low) / 2);
                let mid_conversion_map = &category_conversion_map[mid];

                if mid_conversion_map.start_bound > source {
                    // Special case where `high` would otherwise underflow.
                    // This happens if the entry to find is smaller than the first entry in
                    // the list.
                    if mid == 0 {
                        break;
                    }

                    high = mid - 1;
                } else if let Some(destination) = mid_conversion_map.convert_if_inbound(source) {
                    source = destination;
                    break;
                } else {
                    low = mid + 1;
                }
            }
        }

        source
    }
}

impl Solver for Day5Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day5/input.txt").unwrap();
        let mut lines = file.lines();
        let parsed_seeds = Self::parse_seeds_list_part1(&mut lines);

        let mut categories_conversion_maps = Vec::new();
        while lines.next().is_some() {
            categories_conversion_maps.push(Self::parse_category_conversion_maps(&mut lines));
        }

        println!(
            "The lowest location number is {}",
            parsed_seeds
                .iter()
                .map(|seed| Self::find_seed_location_number(*seed, &categories_conversion_maps))
                .min()
                .unwrap()
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day5/input.txt").unwrap();
        let mut lines = file.lines();
        let parsed_seed_ranges = Self::parse_seeds_list_part2(&mut lines);
        let num_parsed_seed_ranges = parsed_seed_ranges.len();

        let mut categories_conversion_maps = Vec::new();
        while lines.next().is_some() {
            categories_conversion_maps.push(Self::parse_category_conversion_maps(&mut lines));
        }

        let (tx, rx) = channel();
        for parsed_seed_range in parsed_seed_ranges {
            let categories_conversion_maps = categories_conversion_maps.clone();
            let tx = tx.clone();
            std::thread::spawn(move || {
                let lowest_location_number_for_parsed_seed_range = parsed_seed_range
                    .into_iter()
                    .map(|seed| Self::find_seed_location_number(seed, &categories_conversion_maps))
                    .min()
                    .unwrap();
                tx.send(lowest_location_number_for_parsed_seed_range)
                    .unwrap();
            });
        }

        let mut global_lowest_location_number = i64::MAX;
        for _ in 0..num_parsed_seed_ranges {
            global_lowest_location_number = global_lowest_location_number.min(rx.recv().unwrap());
        }

        println!(
            "The lowest location number is {}",
            global_lowest_location_number
        );
    }
}
