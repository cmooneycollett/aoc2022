use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Rucksack Reorganization";
const PROBLEM_INPUT_FILE: &str = "./input/day03.txt";
const PROBLEM_DAY: u64 = 3;

/// Processes the AOC 2022 Day 3 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 3 input file in the format required by the solver functions.
/// Returned value is vector or strings extracted from the lines of the input file.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    return raw_input
        .trim()
        .lines()
        .map(|line| String::from(line.trim()))
        .collect::<Vec<String>>();
}

/// Solves AOC 2022 Day 3 Part 1 // Returns the total prioritisation of the items in both
/// compartments for all knapsacks.
fn solve_part1(input: &[String]) -> u64 {
    let mut total = 0;
    for items in input {
        let first = items[0..items.len() / 2].chars().collect::<HashSet<char>>();
        let last = items[items.len() / 2..].chars().collect::<HashSet<char>>();
        let common_item = *first.intersection(&last).next().unwrap();
        if common_item.is_lowercase() {
            total += 1 + (common_item as u64 - 'a' as u64);
        } else {
            total += 27 + (common_item as u64 - 'A' as u64);
        }
    }
    total
}

/// Solves AOC 2022 Day 3 Part 2 // Returns the total prioritisation of the common item between each
/// three elf group.
fn solve_part2(input: &[String]) -> u64 {
    let mut total = 0;
    for i in (0..input.len()).step_by(3) {
        // Find intersection of
        let first = input[i].chars().collect::<HashSet<char>>();
        let second = input[i + 1].chars().collect::<HashSet<char>>();
        let third = input[i + 2].chars().collect::<HashSet<char>>();
        // Intersection of first two sets
        let first_second = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>();
        // Intersection of third set with first two sets
        let common_item = *first_second.intersection(&third).next().unwrap();
        if common_item.is_lowercase() {
            total += 1 + (common_item as u64 - 'a' as u64);
        } else {
            total += 27 + (common_item as u64 - 'A' as u64);
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 3 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day03_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(8240, solution);
    }

    /// Tests the Day 3 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day03_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2587, solution);
    }
}
