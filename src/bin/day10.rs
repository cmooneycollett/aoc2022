use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Cathode-Ray Tube";
const PROBLEM_INPUT_FILE: &str = "./input/day10.txt";
const PROBLEM_DAY: u64 = 10;

/// Represents the different instructions for the display CPU.
enum Instruction {
    Noop,
    Addv {value: i64},
}

/// Processes the AOC 2022 Day 10 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 10 input file in the format required by the solver functions.
/// Returned value is vector of Instruction read from the lines of the input file..
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_noop = Regex::new(r"^noop$").unwrap();
    let regex_addv = Regex::new(r"^addx (-?\d+)$").unwrap();
    let mut output: Vec<Instruction> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Find the instruction type
        if regex_noop.is_match(line) {
            output.push(Instruction::Noop);
        } else if regex_addv.is_match(line) {
            let caps = regex_addv.captures(line).unwrap();
            let value = caps[1].parse::<i64>().unwrap();
            output.push(Instruction::Addv { value });
        }
    }
    output
}

/// Solves AOC 2022 Day 10 Part 1 // Returns the sum of the signal strengths from the 20th, 60th,
/// 100th, 140th, 180th and 220th clock cycle.
fn solve_part1(input: &[Instruction]) -> i64 {
    // Initialise register
    let mut reg_x = 1;
    // Initialise instruction queue
    let mut target_cycles: HashSet<i64> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut output_sum = 0;
    let mut clock_cycle = 1;
    for instruct in input {
        // Break early if all target cycles have been processed.
        if target_cycles.is_empty() {
            break;
        }
        // Process the instruction
        match instruct {
            Instruction::Noop => {
                if target_cycles.contains(&clock_cycle) {
                    output_sum += reg_x * clock_cycle;
                    target_cycles.remove(&clock_cycle);
                }
                clock_cycle += 1;
            },
            Instruction::Addv { value } => {
                for _ in 0..2 {
                    if target_cycles.contains(&clock_cycle) {
                        output_sum += reg_x * clock_cycle;
                        target_cycles.remove(&clock_cycle);
                    }
                    clock_cycle += 1;
                }
                reg_x += value;
            }
        }
    }
    output_sum
}

/// Solves AOC 2022 Day 10 Part 2 // ###
fn solve_part2(_input: &[Instruction]) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 10 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day10_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(16880, solution);
    }

    /// Tests the Day 10 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day10_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
