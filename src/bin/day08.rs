use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Treetop Tree House";
const PROBLEM_INPUT_FILE: &str = "./input/day08.txt";
const PROBLEM_DAY: u64 = 8;

/// Processes the AOC 2022 Day 8 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 8 input file in the format required by the solver functions.
/// Returned value is 2d vector of values representing tree heights given in the input file.
fn process_input_file(filename: &str) -> Vec<Vec<u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    return raw_input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
}

/// Solves AOC 2022 Day 8 Part 1 // Calculates the number of trees that are visible from outside the
/// grid.
fn solve_part1(tree_heights: &[Vec<u64>]) -> u64 {
    let max_y = tree_heights.len() - 1;
    let max_x = tree_heights[0].len() - 1;
    let mut total_visible = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if y == 0 || y == max_y {
                total_visible += 1;
                continue;
            }
            if x == 0 || x == max_x {
                total_visible += 1;
                continue;
            }
            let mut sides_visible = 4;
            // Left side
            for new_x in 0..x {
                if tree_heights[y][new_x] >= tree_heights[y][x] {
                    sides_visible -= 1;
                    break;
                }
            }
            // Top side
            for new_y in 0..y {
                if tree_heights[new_y][x] >= tree_heights[y][x] {
                    sides_visible -= 1;
                    break;
                }
            }
            // Right side
            for new_x in (x + 1)..=max_x {
                if tree_heights[y][new_x] >= tree_heights[y][x] {
                    sides_visible -= 1;
                    break;
                }
            }
            // Bottom side
            for new_y in (y + 1)..=max_y {
                if tree_heights[new_y][x] >= tree_heights[y][x] {
                    sides_visible -= 1;
                    break;
                }
            }
            // Check if tree is visible
            if sides_visible > 0 {
                total_visible += 1;
            }
        }
    }
    total_visible
}

/// Solves AOC 2022 Day 8 Part 2 // ###
fn solve_part2(_input: &[Vec<u64>]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 8 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day08_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1538, solution);
    }

    /// Tests the Day 8 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day08_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
