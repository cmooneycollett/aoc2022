use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

use regex::Regex;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Beacon Exclusion Zone";
const PROBLEM_INPUT_FILE: &str = "./input/day15.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day15_t001.txt";
const PROBLEM_DAY: u64 = 15;

const PART2_ROW_LIMIT: i64 = 4000000;

/// Processes the AOC 2022 Day 15 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2022 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2022 Day 15 input file in the format required by the solver functions.
/// Returned value is vector of tuples containing the sensor locations and the location of their
/// closest beacon.
fn process_input_file(filename: &str) -> Vec<(Point2D, Point2D)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut output: Vec<(Point2D, Point2D)> = vec![];
    let regex_line =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    for line in raw_input.lines() {
        // Trim input file line and skip over empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Extract fields from input file line
        let caps = regex_line.captures(line).unwrap();
        let x_sens = caps[1].parse::<i64>().unwrap();
        let y_sens = caps[2].parse::<i64>().unwrap();
        let x_beac = caps[3].parse::<i64>().unwrap();
        let y_beac = caps[4].parse::<i64>().unwrap();
        // Create points for the sensor and beacon locations
        let loc_sens = Point2D::new(x_sens, y_sens);
        let loc_beac = Point2D::new(x_beac, y_beac);
        output.push((loc_sens, loc_beac))
    }
    output
}

/// Solves AOC 2022 Day 15 Part 1 // Determines the number of locations in the row where y=2000000
/// which cannot contain a beacon.
fn solve_part1(input: &[(Point2D, Point2D)]) -> usize {
    let mut target_row_locs: HashSet<Point2D> = HashSet::new();
    let target_row = 2000000;
    let beacons_in_target_row = input
        .iter()
        .map(|x| x.1)
        .filter(|x| x.get_y() == target_row)
        .collect::<HashSet<Point2D>>();
    for (loc_sens, loc_beac) in input {
        let output = find_beacon_exclusion_locations_in_row(loc_sens, loc_beac, target_row);
        target_row_locs.extend(output);
    }
    for beacon in beacons_in_target_row {
        if target_row_locs.contains(&beacon) {
            target_row_locs.remove(&beacon);
        }
    }
    target_row_locs.len()
}

/// Solves AOC 2022 Day 15 Part 2 // ###
fn solve_part2(input: &[(Point2D, Point2D)]) -> i64 {
    for y in 0..=PART2_ROW_LIMIT {
        let mut ranges: Vec<RangeInclusive<i64>> = vec![];
        // Find the exclusion zones in the current row from the sensors
        for (loc_sens, loc_beac) in input {
            let mdist = loc_sens.calculate_manhattan_distance(loc_beac) as i64;
            let delta_y = (loc_sens.get_y() - y).abs();
            if delta_y > mdist {
                continue;
            }
            let min_x = loc_sens.get_x() - mdist + delta_y;
            let max_x = loc_sens.get_x() + mdist - delta_y;
            ranges.push(min_x..=max_x);
        }
        // Sort the ranges based on their start value
        ranges.sort_by(|a, b| {
            if a.start() < b.start() {
                Ordering::Less
            } else if a.start() == b.start() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        // Compare the ranges to find the gap where the distress beacon is located
        let mut left = 0;
        let mut right = 1;
        loop {
            if right >= ranges.len() {
                break;
            }
            if ranges[right].start() - ranges[left].end() == 2 {
                return (ranges[left].end() + 1) * 4000000 + y;
            }
            if ranges[right].end() > ranges[left].end() {
                left = right;
                right = left + 1;
            } else {
                right += 1;
            }
        }
    }
    panic!("Day 15 Part 2 - should not get here!");
}

/// Finds the locations in the specified row that could not contain a beacon.
fn find_beacon_exclusion_locations_in_row(
    loc_sens: &Point2D,
    loc_beac: &Point2D,
    target_row: i64,
) -> HashSet<Point2D> {
    let m_dist = loc_sens.calculate_manhattan_distance(loc_beac) as i64;
    let delta_y = (loc_sens.get_y() - target_row).abs();
    let mut output: HashSet<Point2D> = HashSet::new();
    for x in (loc_sens.get_x() - m_dist + delta_y)..=(loc_sens.get_x() + m_dist - delta_y) {
        output.insert(Point2D::new(x, target_row));
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 15 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day15_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(5394423, solution);
    }

    /// Tests the Day 15 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day15_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(11840879211051, solution);
    }
}
