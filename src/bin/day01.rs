use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Calorie Counting";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

/// Processes the AOC 2022 Day 1 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 1 input file in the format required by the solver functions.
/// Returned value is vector containing vectors with the calorie values for each elf.
fn process_input_file(filename: &str) -> Vec<Vec<u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let elf_splits = raw_input.split("\n\n");
    let mut elf_packs: Vec<Vec<u64>> = vec![];
    for elf_split in elf_splits {
        let elf_calories = elf_split
            .trim()
            .split('\n')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        elf_packs.push(elf_calories);
    }
    elf_packs
}

/// Solves AOC 2022 Day 1 Part 1 // Returns the maximum total calories across each of the elf packs.
fn solve_part1(elf_packs: &[Vec<u64>]) -> u64 {
    return elf_packs.iter().map(|x| x.iter().sum()).max().unwrap();
}

/// Solves AOC 2022 Day 1 Part 2 // Returns the total calories for the elf packs with the top three
/// calorie totals.
fn solve_part2(elf_packs: &[Vec<u64>]) -> u64 {
    let mut sums = elf_packs
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<u64>>();
    sums.sort();
    return sums.iter().rev().take(3).sum();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 1 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day01_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(72478, solution);
    }

    /// Tests the Day 1 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day01_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(210367, solution);
    }
}
