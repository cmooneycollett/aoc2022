use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Distress Signal";
const PROBLEM_INPUT_FILE: &str = "./input/day13.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day13_t001.txt";
const PROBLEM_DAY: u64 = 13;

/// Processes the AOC 2022 Day 13 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 13 input file in the format required by the solver functions.
/// Returned value is vector of string pairs given in the input file.
fn process_input_file(filename: &str) -> Vec<(String, String)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut output: Vec<(String, String)> = vec![];
    for pair in raw_input.trim().split("\n\n") {
        let pair_strings = pair.lines().map(|line| line).collect::<Vec<&str>>();
        output.push((pair_strings[0].to_string(), pair_strings[1].to_string()));
    }
    output
}

/// Solves AOC 2022 Day 13 Part 1 // ###
fn solve_part1(input: &[(String, String)]) -> usize {
    let mut index_sum = 0;
    let regex_line = Regex::new(r"(\[|\]|\d+)").unwrap();
    for (i, (left, right)) in input.iter().enumerate() {
        let left_elems = regex_line
            .find_iter(left)
            .map(|elem| elem.as_str().to_string())
            .collect::<Vec<String>>();
        let right_elems = regex_line
            .find_iter(right)
            .map(|elem| elem.as_str().to_string())
            .collect::<Vec<String>>();
        if compare_left_and_right_string(&left_elems, &right_elems) {
            index_sum += i + 1;
        }
    }
    index_sum
}

/// Solves AOC 2022 Day 13 Part 2 // ###
fn solve_part2(_input: &[(String, String)]) -> usize {
    0
}

fn compare_left_and_right_string(left_elems: &Vec<String>, right_elems: &Vec<String>) -> bool {
    // Initialise cursors
    let mut c_left: usize = 0;
    let mut c_right: usize = 0;
    let mut left = left_elems.clone();
    let mut right = right_elems.clone();
    loop {
        // Move out of end of list element
        while c_left < left.len() && left[c_left] == "]" {
            c_left += 1;
        }
        while c_right < right.len() && right[c_right] == "]" {
            c_right += 1;
        }
        // Check bounds
        if c_left >= left.len() {
            // left runs out of elements first
            return true;
        } else if c_right >= right.len() {
            // right runs out of elements first
            return false;
        }
        // Check if both current elements are integers
        let left_num = left[c_left].parse::<u64>();
        let right_num = right[c_right].parse::<u64>();
        if left_num.is_ok() && right_num.is_ok() {
            let left_num = left_num.unwrap();
            let right_num = right_num.unwrap();
            if left_num < right_num {
                return true;
            } else if left_num > right_num {
                return false;
            }
        } else if left_num.is_err() && right_num.is_err() {
            // both items are lists
            ()
        } else if right_num.is_ok() {
            right.insert(c_right, String::from("["));
            right.insert(c_right + 2, String::from("]"));
        } else {
            left.insert(c_left, String::from("["));
            left.insert(c_left + 2, String::from("]"));
        }
        c_left += 1;
        c_right += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 13 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day13_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 13 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day13_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
