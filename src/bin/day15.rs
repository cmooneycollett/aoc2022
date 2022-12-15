use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use regex::Regex;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Beacon Exclusion Zone";
const PROBLEM_INPUT_FILE: &str = "./input/day15.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day15_t001.txt";
const PROBLEM_DAY: u64 = 15;

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
    // let target_row = 10;
    let beacons_in_target_row = input.iter().map(|x| x.1).filter(|x| x.get_y() == target_row).collect::<HashSet<Point2D>>();
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
fn solve_part2(_input: &[(Point2D, Point2D)]) -> usize {
    0
}

/// Finds the locations in the specified row that could not contain a beacon.
fn find_beacon_exclusion_locations_in_row(
    loc_sens: &Point2D,
    loc_beac: &Point2D,
    target_row: i64,
) -> HashSet<Point2D> {
    let m_dist = loc_sens.calculate_manhattan_distance(loc_beac);
    let mut visited: HashSet<Point2D> = HashSet::new();
    let mut visit_queue: VecDeque<Point2D> = VecDeque::new();
    let mut output: HashSet<Point2D> = HashSet::new();
    visited.insert(*loc_sens);
    visit_queue.push_back(*loc_sens);
    // Find min and max y
    let y_vals: Vec<i64> = vec![loc_sens.get_y(), loc_beac.get_y()];
    let y_min: i64 = *y_vals.iter().min().unwrap();
    let y_min = {
        if y_min > target_row {
            target_row
        } else {
            y_min
        }
    };
    let y_max: i64 = *y_vals.iter().max().unwrap();
    let y_max = {
        if y_max < target_row {
            target_row
        } else {
            y_max
        }
    };
    while !visit_queue.is_empty() {
        // Get current location to visit
        let loc = visit_queue.pop_front().unwrap();
        if loc.get_y() == target_row {
            output.insert(loc);
            continue;
        }
        // Try next valid locations
        for next_loc in get_next_valid_points(&loc) {
            if !visited.contains(&next_loc)
                && next_loc.calculate_manhattan_distance(loc_sens) <= m_dist
                && next_loc.get_y() >= y_min
                && next_loc.get_y() <= y_max
            {
                visited.insert(next_loc);
                visit_queue.push_back(next_loc);
            }
        }
    }
    output
}

/// Gets the four locations to the left, right, top and bottom of the given location.
fn get_next_valid_points(loc: &Point2D) -> Vec<Point2D> {
    let mut output: Vec<Point2D> = vec![];
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        output.push(loc.check_move_point(dx, dy));
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
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 15 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day15_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
