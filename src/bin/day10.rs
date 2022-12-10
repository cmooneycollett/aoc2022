use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Cathode-Ray Tube";
const PROBLEM_INPUT_FILE: &str = "./input/day10.txt";
const PROBLEM_DAY: u64 = 10;

/// Represents the different instructions for the display CPU.
enum Instruction {
    Noop,
    Addx { value: i64 },
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
    let regex_addx = Regex::new(r"^addx (-?\d+)$").unwrap();
    let mut output: Vec<Instruction> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Find the instruction type
        if regex_noop.is_match(line) {
            output.push(Instruction::Noop);
        } else if regex_addx.is_match(line) {
            let caps = regex_addx.captures(line).unwrap();
            let value = caps[1].parse::<i64>().unwrap();
            output.push(Instruction::Addx { value });
        }
    }
    output
}

/// Solves AOC 2022 Day 10 Part 1 // Returns the sum of the signal strengths from the 20th, 60th,
/// 100th, 140th, 180th and 220th clock cycle.
fn solve_part1(input: &[Instruction]) -> i64 {
    // Initialise register
    let mut reg_x = 1;
    let mut clock_cycle = 0; // clock cycle 0 is the 1st clock cycle
    let mut target_cycles: HashSet<i64> = HashSet::from([19, 59, 99, 139, 179, 219]);
    let mut signal_strength_sum = 0;
    for instruct in input {
        // Break early if all target cycles have been processed.
        if target_cycles.is_empty() {
            break;
        }
        // Process the instruction
        match instruct {
            Instruction::Noop => {
                // Single cycle for noop
                add_signal_strength_if_targeted(
                    &mut target_cycles,
                    clock_cycle,
                    &mut signal_strength_sum,
                    reg_x,
                );
                clock_cycle += 1;
            }
            Instruction::Addx { value } => {
                // Two cycles for addx
                for _ in 0..2 {
                    add_signal_strength_if_targeted(
                        &mut target_cycles,
                        clock_cycle,
                        &mut signal_strength_sum,
                        reg_x,
                    );
                    clock_cycle += 1;
                }
                reg_x += value;
            }
        }
    }
    signal_strength_sum
}

/// Solves AOC 2022 Day 10 Part 2 // Determines the eight capital letters displayed on the CRT
/// screen after processing the instructions.
fn solve_part2(input: &[Instruction]) -> String {
    let mut reg_x = 1;
    let mut clock_cycle = 0;
    // Output array (bool) where false represents dark pixel and true represents lit pixel
    let mut output_array: [[bool; 40]; 6] = [[false; 40]; 6];
    for instruct in input {
        // Break when the whole CRT screen has been rendered
        if clock_cycle > 240 {
            break;
        }
        // Process the instruction
        match instruct {
            Instruction::Noop => {
                // Single cycle for noop
                draw_pixel_if_selected(reg_x, clock_cycle, &mut output_array);
                clock_cycle += 1
            }
            Instruction::Addx { value } => {
                // Two cycles for addx
                for _ in 0..2 {
                    draw_pixel_if_selected(reg_x, clock_cycle, &mut output_array);
                    clock_cycle += 1
                }
                reg_x += value;
            }
        }
    }
    // Decode the output array
    decode_output_array(&output_array)
}

/// Checks if a target clock cycle has been reached. If so, it is removed from the set of the target
/// clock cycles and the current signal strength (product of clock cycle ordinal position and reg_x)
/// is added to the output sum.
fn add_signal_strength_if_targeted(
    target_cycles: &mut HashSet<i64>,
    clock_cycle: i64,
    signal_strength_sum: &mut i64,
    reg_x: i64,
) {
    if target_cycles.contains(&clock_cycle) {
        *signal_strength_sum += reg_x * (clock_cycle + 1);
        target_cycles.remove(&clock_cycle);
    }
}

/// Checks if the pixel denoted by the current clock cycle is to be drawn (if the three-pixel
/// sprite located by the reg_x value overlaps with the pixel denoted by the current clock cycle).
/// If so, the corresponding pixel location in the output array is set to be true.
fn draw_pixel_if_selected(reg_x: i64, clock_cycle: i64, output_array: &mut [[bool; 40]; 6]) {
    if ((reg_x - 1)..=(reg_x + 1)).contains(&(clock_cycle % 40)) {
        let x = (clock_cycle as usize) % 40;
        let y = (clock_cycle as usize) / 40;
        output_array[y][x] = true;
    }
}

/// Decodes the output array by determining the eight capital letters represented in the array.
fn decode_output_array(output_array: &[[bool; 40]; 6]) -> String {
    // Output
    let mut output = String::new();
    // Letter sequences (6 rows of 5 chars all concatenated for each letter)
    let letters: HashMap<&str, char> = HashMap::from([
        (".##..#..#.#..#.####.#..#.#..#.", 'A'),
        ("###..#..#.###..#..#.#..#.###..", 'B'),
        (".###.#....#....#....#.....###.", 'C'),
        ("###..#..#.#..#.#..#.#..#.###..", 'D'),
        ("####.#....####.#....#....####.", 'E'),
        ("####.#....###..#....#....#....", 'F'),
        ("####.#..#.#....#.##.#..#.####.", 'G'),
        ("#..#.#..#.####.#..#.#..#.#..#.", 'H'),
        ("#####..#....#....#....#..#####", 'I'),
        ("..##....#....#....#.#..#..##..", 'J'),
        ("#..#.#.#..##...#.#..#.#..#..#.", 'K'),
        ("#....#....#....#....#....####.", 'L'),
        ("#...###.###.#.##...##...##...#", 'M'),
        ("#...###..##.#.##..###...##...#", 'N'),
        ("####.#..#.#..#.#..#.#..#.####.", 'O'),
        ("###..#..#.#..#.###..#....#....", 'P'),
        (".##..#..#.#..#.#..#..###.....#", 'Q'),
        ("###..#..#.#..#.###..#.#..#..#.", 'R'),
        (".###.#....#.....##.....#.###..", 'S'),
        ("#####..#....#....#....#....#..", 'T'),
        ("#..#.#..#.#..#.#..#.#..#..##..", 'U'),
        ("#...##...##...##...#.#.#...#..", 'V'),
        ("#...##...##.#.##.#.##.#.######", 'W'),
        ("#...#.#.#...#....#...#.#.#...#", 'X'),
        ("#...#.#.#...#....#....#....#..", 'Y'),
        ("####....#...#...#...#....####.", 'Z'),
    ]);
    // Construct output for each of the eight character boxes (6px high and 5px wide)
    for i in 0..8 {
        let mut letter_key = String::new();
        for row in output_array {
            for x in 0..5 {
                // Determine the char representation of the pixel from bool array
                let pixel = {
                    if row[x + i * 5] {
                        '#'
                    } else {
                        '.'
                    }
                };
                // Push the pixel char to the end of the letter key
                letter_key.push(pixel);
            }
        }
        // Determine which capital letter is represented in the current character box
        if letters.contains_key(letter_key.as_str()) {
            output.push(*letters.get(letter_key.as_str()).unwrap());
        } else {
            output.push('#'); // Placeholder for invalid letter representation
        }
    }
    // Return the result string
    output
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
        let solution = solve_part2(&input);
        assert_eq!("RKAZAJBR", &solution);
    }
}
