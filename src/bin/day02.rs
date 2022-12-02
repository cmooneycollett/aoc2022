use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Rock Paper Scissors";
const PROBLEM_INPUT_FILE: &str = "./input/day02.txt";
const PROBLEM_DAY: u64 = 2;

/// Processes the AOC 2022 Day 2 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 2 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> Vec<(char, char)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut rounds: Vec<(char, char)> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let plays = line.split(" ").map(|x| x.chars().next().unwrap()).collect::<Vec<char>>();
        rounds.push((plays[0], plays[1]));
    }
    return rounds;
}

/// Solves AOC 2022 Day 2 Part 1 // Returns the total score from playing the "rock paper scissors"
/// game, assuming everything goes exactly according to the strategy guide.
fn solve_part1(rounds: &Vec<(char, char)>) -> u64 {
    let mut total_score = 0;
    for round in rounds {
        total_score += match (round.0, round.1) {
            ('A', 'X') => 4, // tie (3, 1)
            ('A', 'Y') => 8, // win (6, 2)
            ('A', 'Z') => 3, // lose (0, 3)
            ('B', 'X') => 1, // lose (0, 1)
            ('B', 'Y') => 5, // tie (3, 2)
            ('B', 'Z') => 9, // win (6, 3)
            ('C', 'X') => 7, // win (6, 1)
            ('C', 'Y') => 2, // lose (0, 2)
            ('C', 'Z') => 6, // tie (3, 3)
            _ => panic!("Day 2 Part 1 - bad round pattern!")
        }
    }
    return total_score;
}

/// Solves AOC 2022 Day 2 Part 2 // ###
fn solve_part2(_rounds: &Vec<(char, char)>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 2 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day02_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(14163, solution);
    }

    /// Tests the Day 2 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day02_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
