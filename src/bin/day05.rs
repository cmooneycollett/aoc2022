use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Supply Stacks";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

/// Type defintion to simplify function signatures.
type ProblemInput = (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>);

/// Processes the AOC 2022 Day 5 input file and solves both parts of the problem. Solutions are
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
    println!("[*] TOTAL:  {:.2?}", input_parser_duration + p1_duration + p2_duration);
    println!("==================================================");
}

/// Processes the AOC 2022 Day 5 input file in the format required by the solver functions. Returned
/// value is tuple containing the vectors of crate stacks and move instructions.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Initialise the output structures
    let mut stacks: Vec<VecDeque<char>> = vec![];
    for _ in 0..9 {
        stacks.push(VecDeque::<char>::new());
    }
    let mut move_instructions: Vec<(usize, usize, usize)> = vec![];
    // Create the regexes
    let stack_regex = Regex::new(concat!(
        r#"^(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )"#,
        r#"(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )"#,
        r#"(   |\[[A-Z]\])$"#,
    ))
    .unwrap();
    let crate_regex = Regex::new(r"\[([A-Z])\]").unwrap();
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    // Process the lines
    let mut line_count = 0;
    for line in raw_input.lines() {
        line_count += 1;
        // Process crate stack lines
        if (1..=8).contains(&line_count) {
            let captures = stack_regex.captures(line).unwrap();
            for i in 1..=9 {
                if crate_regex.is_match(&captures[i]) {
                    let c = crate_regex.captures(&captures[i]).unwrap()[1]
                        .chars()
                        .next()
                        .unwrap();
                    stacks[i - 1].push_front(c);
                }
            }
        // Process move instruction lines
        } else if line_count >= 11 {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let captures = move_regex.captures(line).unwrap();
            let quantity = captures[1].parse::<usize>().unwrap();
            let from = captures[2].parse::<usize>().unwrap() - 1;
            let to = captures[3].parse::<usize>().unwrap() - 1;
            move_instructions.push((quantity, from, to))
        }
    }
    (stacks, move_instructions)
}

/// Solves AOC 2022 Day 5 Part 1 // Returns the crates at the top of each stack after processing
/// the movement instructions.
fn solve_part1(input: &ProblemInput) -> String {
    let mut stacks = input.0.clone();
    // Move crates
    for (quantity, from, to) in input.1.iter() {
        for _ in 0..*quantity {
            let c = stacks[*from].pop_back().unwrap();
            stacks[*to].push_back(c);
        }
    }
    // Construct the output string
    construct_output_string(&stacks)
}

/// Solves AOC 2022 Day 5 Part 2 // Returns the crates at the top of each stack after processing
/// the movement instructions, with the crane picking up and moving the crates at once rather than
/// one-by-one.
fn solve_part2(input: &ProblemInput) -> String {
    let mut stacks = input.0.clone();
    // Move crates
    for (quantity, from, to) in input.1.iter() {
        // Pick up the stack of crates
        let mut move_queue = VecDeque::<char>::new();
        for _ in 0..*quantity {
            let c = stacks[*from].pop_back().unwrap();
            move_queue.push_front(c);
        }
        // Move the stack of crates to the new location
        for _ in 0..*quantity {
            stacks[*to].push_back(move_queue.pop_front().unwrap());
        }
    }
    // Construct the output string
    construct_output_string(&stacks)
}

/// Creates the output string by joining together the last crate in each of the stacks.
fn construct_output_string(stacks: &[VecDeque<char>]) -> String {
    let mut output = String::new();
    for stack in stacks.iter() {
        output.push(stack[stack.len() - 1]);
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 5 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day05_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("VWLCWGSDQ", solution);
    }

    /// Tests the Day 5 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day05_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("TCGLQSLPW", solution);
    }
}
