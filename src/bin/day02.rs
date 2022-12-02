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
/// Returned value is vector of strings extracted from the lines of the input file.
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

/// Solves AOC 2022 Day 2 Part 1 // Returns the total score from playing the "rock paper scissors"
/// game, assuming everything goes exactly according to the strategy guide.
fn solve_part1(rounds: &[String]) -> u64 {
    let mut total_score = 0;
    for line in rounds {
        // A / X: rock (1), B / Y: paper (2), C / Z: scissors (3), lose (0), draw (3), win (6)
        total_score += match line.as_str() {
            "A X" => 4, // draw (3, 1)
            "A Y" => 8, // win (6, 2)
            "A Z" => 3, // lose (0, 3)
            "B X" => 1, // lose (0, 1)
            "B Y" => 5, // draw (3, 2)
            "B Z" => 9, // win (6, 3)
            "C X" => 7, // win (6, 1)
            "C Y" => 2, // lose (0, 2)
            "C Z" => 6, // draw (3, 3)
            _ => panic!("Day 2 Part 1 - bad round pattern!"),
        }
    }
    total_score
}

/// Solves AOC 2022 Day 2 Part 2 // Returns the total score from playing the "rock paper scissors"
/// game, with the second second shape representing the required outcome from the round.
fn solve_part2(rounds: &[String]) -> u64 {
    let mut total_score = 0;
    for line in rounds {
        // A: rock (1), B: paper (2), C: scissors (3), X: lose (0), Y: draw (3), Z: win (6)
        total_score += match line.as_str() {
            "A X" => 3, // lose (0, 3)
            "A Y" => 4, // draw (3, 1)
            "A Z" => 8, // win (6, 2)
            "B X" => 1, // lose (0, 1)
            "B Y" => 5, // draw (3, 2)
            "B Z" => 9, // win (6, 3)
            "C X" => 2, // lose (0, 2)
            "C Y" => 6, // draw (3, 3)
            "C Z" => 7, // win (6, 1)
            _ => panic!("Day 2 Part 2 - bad round pattern!"),
        }
    }
    total_score
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
        let solution = solve_part2(&input);
        assert_eq!(12091, solution);
    }
}
