use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

const PROBLEM_NAME: &str = "Distress Signal";
const PROBLEM_INPUT_FILE: &str = "./input/day13.txt";
const PROBLEM_DAY: u64 = 13;

lazy_static! {
    static ref REGEX_LINE: Regex = Regex::new(r"(\[|\]|\d+)").unwrap();
}

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

/// Solves AOC 2022 Day 13 Part 1 // Returns the sum of the pair indices for the pairs that are in
/// the correct order.
fn solve_part1(input: &[(String, String)]) -> usize {
    let mut index_sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        if compare_left_and_right_packets(left, right) {
            index_sum += i + 1;
        }
    }
    index_sum
}

/// Solves AOC 2022 Day 13 Part 2 // ###
fn solve_part2(input: &[(String, String)]) -> usize {
    // Add all packets into the vector
    let mut packets: Vec<String> = vec![];
    for (left, right) in input {
        packets.push(left.to_string());
        packets.push(right.to_string());
    }
    // Add the divider packets
    packets.push(String::from("[[2]]"));
    packets.push(String::from("[[6]]"));
    // Sort the packets
    packets.sort_by(|left, right| {
        if compare_left_and_right_packets(left, right) {
            Ordering::Less
        } else if left == right {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // Get the indices of the divider packets
    let mut index2: Option<usize> = None;
    let mut index6: Option<usize> = None;
    for (i, packet) in packets.iter().enumerate() {
        if packet == "[[2]]" {
            index2 = Some(i + 1);
        } else if packet == "[[6]]" {
            index6 = Some(i + 1);
        }
        if index2.is_some() && index6.is_some() {
            break;
        }
    }
    if index2.is_none() || index6.is_none() {
        panic!("Day 13 Part 2 - could not find both divider packets in sorted vector!");
    }
    index2.unwrap() * index6.unwrap()
}

/// Compares the left and right packets, represented by vector of their tokens. Returns true if the
/// packets are in the right order. Otherwise, returns false.
fn compare_left_and_right_packets(left_packet: &String, right_packet: &String) -> bool {
    // Initialise cursors
    let mut c_left: usize = 0;
    let mut c_right: usize = 0;
    // Tokenise packets
    let mut left = REGEX_LINE
        .find_iter(left_packet)
        .map(|elem| elem.as_str().to_string())
        .collect::<Vec<String>>();
    let mut right = REGEX_LINE
        .find_iter(right_packet)
        .map(|elem| elem.as_str().to_string())
        .collect::<Vec<String>>();
    loop {
        // Move out of end of list element
        loop {
            // Check bounds
            if c_left >= left.len() {
                // left runs out of elements first
                return true;
            } else if c_right >= right.len() {
                // right runs out of elements first
                return false;
            }
            // Check that both left and right are not at the end of a list
            if left[c_left] != "]" && right[c_right] != "]" {
                break;
            }
            if left[c_left] == "]" && right[c_right] != "]" {
                // left has reached end of a list before right
                return true;
            } else if left[c_left] != "]" && right[c_right] == "]" {
                // right has reached end of a list before left
                return false;
            } else {
                // Both left and right at end of list with same number of elements
                c_left += 1;
                c_right += 1;
            }
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
        let solution = solve_part1(&input);
        assert_eq!(6076, solution);
    }

    /// Tests the Day 13 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day13_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(24805, solution);
    }
}
