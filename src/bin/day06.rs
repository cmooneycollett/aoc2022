use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Tuning Trouble";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

/// Processes the AOC 2022 Day 6 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 6 input file in the format required by the solver functions.
/// Returned value is vector of characters given in the input file.
fn process_input_file(filename: &str) -> Vec<char> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    return raw_input.trim().chars().collect::<Vec<char>>();
}

/// Solves AOC 2022 Day 6 Part 1 // Returns the number of characters that need to be processed
/// before the first start-of-packet marker (four consecutive characters that are different) is
/// observed.
fn solve_part1(input: &[char]) -> usize {
    for cursor in 0..(input.len() - 3) {
        let mut window_set: HashSet<char> = HashSet::new();
        for i in 0..4 {
            window_set.insert(input[cursor + i]);
        }
        if window_set.len() == 4 {
            return cursor + 4;
        }
    }
    panic!("Day 6 Part 1 - did not first the start-of-packet marker!");
}

/// Solves AOC 2022 Day 6 Part 2 // Returns the number of characters that need to be processed
/// before the first start-of-message marker (13 consecutive characters that are different) is
/// observed.
fn solve_part2(input: &[char]) -> usize {
    for cursor in 0..(input.len() - 13) {
        let mut window_set: HashSet<char> = HashSet::new();
        for i in 0..14 {
            window_set.insert(input[cursor + i]);
        }
        if window_set.len() == 14 {
            return cursor + 14;
        }
    }
    panic!("Day 6 Part 2 - did not first the start-of-message marker!");
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 6 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1109, solution);
    }

    /// Tests the Day 6 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(3965, solution);
    }
}
