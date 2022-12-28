use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Full of Hot Air";
const PROBLEM_INPUT_FILE: &str = "./input/day25.txt";
const PROBLEM_DAY: u64 = 25;

lazy_static! {
    static ref SNAFU_DIGITS: Vec<char> = vec!['0', '1', '2', '=', '-'];
}

/// Processes the AOC 2022 Day 25 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 25 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
    // let mut output: Vec<Vec<i64>> = vec![];
    // for line in raw_input.lines() {
    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue;
    //     }
    //     let mut digits: Vec<i64> = vec![];
    //     for c in line.chars().rev() {
    //         match c {
    //             '0' => digits.push(0),
    //             '1' => digits.push(1),
    //             '2' => digits.push(2),
    //             '-' => digits.push(-1),
    //             '=' => digits.push(-2),
    //             _ => panic!("Bad character in input file line!"),
    //         }
    //     }
    // }
    // output
}

/// Solves AOC 2022 Day 25 Part 1 // Determines the SNAFU number that needs to be supplied to Bob's
/// console.
fn solve_part1(snafu_numbers: &[String]) -> String {
    let mut snafu_sum = 0;
    for snafu_num in snafu_numbers {
        snafu_sum += convert_snafu_number_to_decimal(snafu_num);
    }
    convert_decimal_to_snafu(snafu_sum)
}

/// Solves AOC 2022 Day 25 Part 2 // ###
fn solve_part2(_input: &[String]) -> bool {
    false
}

/// Converts the given decimal value into the equivalent SNAFU string representation.
fn convert_decimal_to_snafu(value: i64) -> String {
    let mut holder = value;
    let mut output: Vec<char> = vec![];
    loop {
        let i = (holder % 5) as usize;
        output.push(SNAFU_DIGITS[i]);
        holder = (holder + 2) / 5;
        if holder == 0 {
            break;
        }
    }
    output.iter().rev().collect::<String>()
}

/// Converts the string representation of a SNAFU number into the equivalent decimal representation.
fn convert_snafu_number_to_decimal(snafu_number: &str) -> i64 {
    let mut snafu_decimal = 0;
    let mut place = 1;
    for c in snafu_number.chars().rev() {
        match c {
            '0' => (),
            '1' => snafu_decimal += place,
            '2' => snafu_decimal += place * 2,
            '-' => snafu_decimal += place * -1,
            '=' => snafu_decimal += place * -2,
            _ => panic!("Bad character in input file line!"),
        }
        place *= 5
    }
    snafu_decimal
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 25 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day25_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(String::from("2=01-0-2-0=-0==-1=01"), solution);
    }

    /// Tests the Day 25 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day25_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 25 Part 1 solver method against example input 001.
    #[test]
    fn test_day25_part1_t001() {
        let input = process_input_file("./input/test/day25_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(String::from("2=-1=0"), solution);
    }
}
