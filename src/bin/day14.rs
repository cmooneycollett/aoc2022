use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Regolith Reservoir";
const PROBLEM_INPUT_FILE: &str = "./input/day14.txt";
const PROBLEM_DAY: u64 = 14;

/// Represents a single tile type in the cave map.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TileType {
    Rock,
    Sand
}

/// Processes the AOC 2022 Day 14 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 14 input file in the format required by the solver functions.
/// Returned value is hashmap representing the locations of cave rock specified in the input file.
fn process_input_file(filename: &str) -> HashMap<Point2D, TileType> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut cave_map: HashMap<Point2D, TileType> = HashMap::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut points: Vec<(i64, i64)> = vec![];
        for point_raw in line.split(" -> ") {
            let pair = point_raw.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            points.push((pair[0], pair[1]));
        }
        // Draw the rock
        for i in 1..points.len() {
            let x_vals = vec![points[i - 1].0, points[i].0];
            let y_vals = vec![points[i - 1].1, points[i].1];
            let x_from = *x_vals.iter().min().unwrap();
            let y_from = *y_vals.iter().min().unwrap();
            let x_to = *x_vals.iter().max().unwrap();
            let y_to = *y_vals.iter().max().unwrap();
            if x_from == x_to {
                for y in y_from..=y_to {
                    cave_map.insert(Point2D::new(x_from, y), TileType::Rock);
                }
            } else if y_from == y_to {
                for x in x_from..=x_to {
                    cave_map.insert(Point2D::new(x, y_from), TileType::Rock);
                }
            } else {
                panic!("Day 14 - bad element in cave map line!");
            }
        }
    }
    cave_map
}

/// Solves AOC 2022 Day 14 Part 1 // ###
fn solve_part1(input: &HashMap<Point2D, TileType>) -> usize {
    let mut cave_map = input.clone();
    let max_y = cave_map.keys().map(|loc| loc.get_y()).max().unwrap();
    loop {
        let mut sand_loc = Point2D::new(500, 0);
        let mut sand_in_abyss = false;
        loop {
            // Check if the sand is in the abyss
            if sand_loc.get_y() > max_y {
                sand_in_abyss = true;
                break;
            }
            // Check where the sand moves to
            if !cave_map.contains_key(&sand_loc.check_move_point(0, 1)) {
                // Try to move directly down
                sand_loc.move_point(0, 1);
                continue;
            } else if !cave_map.contains_key(&sand_loc.check_move_point(-1, 1)) {
                // Try to move down diag left
                sand_loc.move_point(-1, 1);
                continue;
            } else if !cave_map.contains_key(&sand_loc.check_move_point(1, 1)) {
                // Try to move down diag right
                sand_loc.move_point(1, 1);
                continue
            } else {
                // Sand comes to rest
                cave_map.insert(sand_loc, TileType::Sand);
                break;
            }

        }
        if sand_in_abyss {
            return cave_map.values().filter(|tile| **tile == TileType::Sand).count();
        }
    }
}

/// Solves AOC 2022 Day 14 Part 2 // ###
fn solve_part2(_input: &HashMap<Point2D, TileType>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 14 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day14_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(719, solution);
    }

    /// Tests the Day 14 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day14_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
