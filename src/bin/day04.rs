use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Camp Cleanup";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

/// Processes the AOC 2022 Day 4 input file and solves both parts of the problem. Solutions are
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
    println!("==================================================");
}

/// Processes the AOC 2022 Day 4 input file in the format required by the solver functions.
/// Returned value is vector of four-tuples containing the lower and upper limits of the ranges
/// specified in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<(u64, u64, u64, u64)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_line = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut ranges: Vec<(u64, u64, u64, u64)> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let captures = regex_line.captures(line).unwrap();
        let first_left = captures[1].parse::<u64>().unwrap();
        let first_right = captures[2].parse::<u64>().unwrap();
        let second_left = captures[3].parse::<u64>().unwrap();
        let second_right = captures[4].parse::<u64>().unwrap();
        ranges.push((first_left, first_right, second_left, second_right));
    }
    ranges
}

/// Solves AOC 2022 Day 4 Part 1 // Returns the number of range pairs where one range fully contains
/// the other range.
fn solve_part1(input: &[(u64, u64, u64, u64)]) -> u64 {
    let mut count = 0;
    for range_pair in input {
        if check_for_whole_overlap(range_pair) {
            count += 1;
        }
    }
    count
}

/// Solves AOC 2022 Day 4 Part 2 // Returns the number of range pairs where the two ranges overlap
/// in whole or in part
fn solve_part2(input: &[(u64, u64, u64, u64)]) -> u64 {
    let mut count = 0;
    for range_pair in input {
        if check_for_whole_overlap(range_pair) || check_for_partial_overlap(range_pair) {
            count += 1;
        }
    }
    count
}

/// Checks if one of the ranges in the pair fully contains the other.
fn check_for_whole_overlap(range_pair: &(u64, u64, u64, u64)) -> bool {
    let (first_left, first_right, second_left, second_right) = range_pair;
    // Check if first range fully contains the second range
    if first_left <= second_left && first_right >= second_right {
        return true;
    }
    // Check if second range fully contains the first range
    if second_left <= first_left && second_right >= first_right {
        return true;
    }
    false
}

/// Checks if there is a partial overlap between the ranges.
fn check_for_partial_overlap(range_pair: &(u64, u64, u64, u64)) -> bool {
    // Calculate minimum length of range of the two ranges
    let (first_left, first_right, second_left, second_right) = range_pair;
    let first_len = first_right - first_left + 1;
    let second_len = second_right - second_left + 1;
    let min_len = {
        if first_len < second_len {
            first_len
        } else {
            second_len
        }
    };
    // First range is to left
    if first_left < second_right && first_right >= second_left {
        let overlap_len = first_right - second_left + 1;
        if overlap_len < min_len {
            return true;
        }
    }
    // Second range is to left
    if second_left < first_right && second_right >= first_left {
        let overlap_len = second_right - first_left + 1;
        if overlap_len < min_len {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 4 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day04_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(462, solution);
    }

    /// Tests the Day 4 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day04_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(835, solution);
    }
}
