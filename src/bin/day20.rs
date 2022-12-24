use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Grove Positioning System";
const PROBLEM_INPUT_FILE: &str = "./input/day20.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day20_t001.txt";
const PROBLEM_DAY: u64 = 20;

/// Processes the AOC 2022 Day 20 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 20 input file in the format required by the solver functions.
/// Returned value is vector of integers listed in the input file.
fn process_input_file(filename: &str) -> Vec<i64> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

/// Solves AOC 2022 Day 20 Part 1 // ###
fn solve_part1(input: &[i64]) -> i64 {
    let mut mixed_values = input
        .iter()
        .map(|val| (*val, false))
        .collect::<Vec<(i64, bool)>>();
    let mut cursor = 0;
    loop {
        if cursor >= mixed_values.len() {
            break;
        }
        if mixed_values[cursor].1 {
            cursor += 1;
            continue;
        }
        let new_index = {
            let cursor_signed = cursor as i64;
            let temp_index =
                (cursor_signed + mixed_values[cursor].0) % (mixed_values.len() - 1) as i64;
            if temp_index < 0 {
                mixed_values.len() - 1 - temp_index.unsigned_abs() as usize
            } else {
                temp_index as usize
            }
        };
        let old_value = mixed_values[cursor];
        // mixed_values.remove(cursor);
        // mixed_values.insert(new_index, old_value);
        if new_index < cursor {
            mixed_values.insert(new_index, old_value);
            mixed_values.remove(cursor + 1);
        } else if new_index > cursor {
            mixed_values.remove(cursor);
            mixed_values.insert(new_index, old_value);
        }
        mixed_values[new_index] = (mixed_values[new_index].0, true);
    }
    let index_0 = mixed_values.iter().position(|elem| elem.0 == 0).unwrap();
    let val_1000 = mixed_values[(index_0 + 1000) % mixed_values.len()].0;
    let val_2000 = mixed_values[(index_0 + 2000) % mixed_values.len()].0;
    let val_3000 = mixed_values[(index_0 + 3000) % mixed_values.len()].0;
    val_1000 + val_2000 + val_3000
}

/// Solves AOC 2022 Day 20 Part 2 // ###
fn solve_part2(_input: &[i64]) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 20 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day20_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(2215, solution);
    }

    /// Tests the Day 20 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day20_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
