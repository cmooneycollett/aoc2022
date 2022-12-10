use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Rope Bridge";
const PROBLEM_INPUT_FILE: &str = "./input/day09.txt";
const PROBLEM_DAY: u64 = 9;

/// Represents a movement in a different cardinal direction with an associated number of steps.
#[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Left,
    Right,
}

/// Processes the AOC 2022 Day 9 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 9 input file in the format required by the solver functions.
/// Returned value is vector of tuples containing move type and number of steps.
fn process_input_file(filename: &str) -> Vec<(MoveType, usize)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut output: Vec<(MoveType, usize)> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let split = line.split(" ").collect::<Vec<&str>>();
        let steps = split[1].parse::<usize>().unwrap();
        match split[0] {
            "U" => {
                output.push((MoveType::Up, steps));
            },
            "R" => {
                output.push((MoveType::Right, steps));
            },
            "D" => {
                output.push((MoveType::Down, steps));
            },
            "L" => {
                output.push((MoveType::Left, steps));
            },
            _ => panic!("Day 9 - bad move type!"),
        }
    }
    output
}

/// Processes the rope moves and returns the number of unique locations visited by the tail knot.
fn process_rope_moves(instructions: &[(MoveType, usize)], rope_len: usize) -> usize {
    if rope_len == 0 {
        return 0;
    }
    let mut tail_locs: HashSet<Point2D> = HashSet::new();
    let mut knots: Vec<Point2D> = vec![Point2D::new(0, 0); rope_len].to_vec();
    tail_locs.insert(knots[rope_len - 1]);
    for (move_type, steps) in instructions {
        for _ in 0..*steps {
            // Move the first knot
            let mut new_knots: Vec<Point2D> = vec![];
            match move_type {
                MoveType::Up => new_knots.push(knots[0].check_move_point(0, -1)),
                MoveType::Down => new_knots.push(knots[0].check_move_point(0, 1)),
                MoveType::Left => new_knots.push(knots[0].check_move_point(-1, 0)),
                MoveType::Right => new_knots.push(knots[0].check_move_point(1, 0)),
            }
            // Now move the following knots
            for i in 1..rope_len {
                let delta_x = new_knots[i - 1].get_x() - knots[i].get_x();
                let delta_y = new_knots[i - 1].get_y() - knots[i].get_y();
                if delta_x.abs() >= 2 || delta_y.abs() >= 2 {
                    // Normalise delta_x
                    let dx = {
                        if delta_x == 0 || delta_x == 1 || delta_x == -1 {
                            0
                        } else if delta_x >= 2 {
                            1
                        } else if delta_x <= -2 {
                            -1
                        } else {
                            panic!("should not get here!");
                        }
                    };
                    // Normalise delta_y
                    let dy = {
                        if delta_y == 0 || delta_y == 1 || delta_y == -1 {
                            0
                        } else if delta_y >= 2 {
                            1
                        } else if delta_y <= -2 {
                            -1
                        } else {
                            panic!("should not get here!");
                        }
                    };
                    new_knots.push(knots[i].check_move_point(delta_x - dx, delta_y - dy));
                } else {
                    new_knots.push(knots[i]);
                }
            }
            // Update the knot locations and insert the tail knot location into set
            knots = new_knots;
            tail_locs.insert(knots[rope_len - 1]);
        }
    }
    tail_locs.len()
}

/// Solves AOC 2022 Day 9 Part 1 // Calculates the number of unique locations visited by the tail of
/// the rope (two knots).
fn solve_part1(instructions: &[(MoveType, usize)]) -> usize {
    process_rope_moves(instructions, 2)
}

/// Solves AOC 2022 Day 9 Part 2 // ###
fn solve_part2(_instructions: &[(MoveType, usize)]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 9 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day09_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(6311, solution);
    }

    /// Tests the Day 09 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day09_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
