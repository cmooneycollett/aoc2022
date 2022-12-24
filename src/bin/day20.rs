use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Grove Positioning System";
const PROBLEM_INPUT_FILE: &str = "./input/day20.txt";
const PROBLEM_DAY: u64 = 20;

const PART2_DECRYPTION_KEY: i64 = 811589153;

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

/// Solves AOC 2022 Day 20 Part 1 // Finds the sum of the three numbers that form the grove
/// co-ordinates.
fn solve_part1(values: &[i64]) -> i64 {
    // Conduct one round of mixing
    let values = mix_values(values, 1);
    // Find grove co-ordinates sum
    find_grove_coordinates_sum(values)
}

/// Solves AOC 2022 Day 20 Part 2 // Finds the sum of the three numbers that form the grove
/// co-ordinates after applying the decryption key to the input values and mixing them 10 times.
fn solve_part2(values: &[i64]) -> i64 {
    // Prepare the input values list for mixing - apply decryption key
    let values = values
        .iter()
        .copied()
        .map(|val| val * PART2_DECRYPTION_KEY)
        .collect::<Vec<i64>>();
    // Conduct 10 rounds of mixing
    let values = mix_values(&values, 10);
    // Find grove co-ordinates sum
    find_grove_coordinates_sum(values)
}

/// Finds the sum of the three values that form the grove co-ordinates.
fn find_grove_coordinates_sum(values: Vec<i64>) -> i64 {
    let index_0 = values.iter().position(|elem| *elem == 0).unwrap();
    let val_1000 = values[(index_0 + 1000) % values.len()];
    let val_2000 = values[(index_0 + 2000) % values.len()];
    let val_3000 = values[(index_0 + 3000) % values.len()];
    val_1000 + val_2000 + val_3000
}

/// Conducts one round of value mixing.
fn mix_values(input_values: &[i64], rounds: u64) -> Vec<i64> {
    let mut values = input_values
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    for _ in 0..rounds {
        for i in 0..values.len() {
            // Find cursor
            let cursor = values.iter().position(|elem| elem.0 == i).unwrap();
            // Find the new index
            let new_index = calculate_new_index(cursor, &values);
            // Shift the old value
            let old_value = values[cursor];
            match new_index.cmp(&cursor) {
                Ordering::Less => {
                    values.insert(new_index, old_value);
                    values.remove(cursor + 1);
                }
                Ordering::Greater => {
                    values.remove(cursor);
                    values.insert(new_index, old_value);
                }
                Ordering::Equal => (),
            }
        }
    }
    values
        .iter()
        .copied()
        .map(|elem| elem.1)
        .collect::<Vec<i64>>()
}

/// Calculates the new index for the value at the given cursor location.
fn calculate_new_index(cursor: usize, values: &Vec<(usize, i64)>) -> usize {
    let cursor_signed = cursor as i64;
    let temp_index = (cursor_signed + values[cursor].1) % (values.len() - 1) as i64;
    if temp_index < 0 {
        values.len() - 1 - temp_index.unsigned_abs() as usize
    } else {
        temp_index as usize
    }
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
        let solution = solve_part2(&input);
        assert_eq!(8927480683, solution);
    }
}
