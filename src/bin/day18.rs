use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point3D;

const PROBLEM_NAME: &str = "Boiling Boulders";
const PROBLEM_INPUT_FILE: &str = "./input/day18.txt";
const PROBLEM_DAY: u64 = 18;

/// Processes the AOC 2022 Day 18 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 18 input file in the format required by the solver functions.
/// Returned value is vector of Point3D structs using the co-ordinates listed in the input file.
fn process_input_file(filename: &str) -> HashSet<Point3D> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut output: HashSet<Point3D> = HashSet::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let coords = line
            .split(",")
            .map(|elem| elem.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        output.insert(Point3D::new(coords[0], coords[1], coords[2]));
    }
    output
}

/// Solves AOC 2022 Day 18 Part 1 // Determines the surface area of the scanned lava droplet.
fn solve_part1(observed_cubes: &HashSet<Point3D>) -> u64 {
    calculate_total_surface_area(observed_cubes)
}

/// Solves AOC 2022 Day 18 Part 2 // Determines the external surface area of the scanned lava
/// droplet.
fn solve_part2(_cubes: &HashSet<Point3D>) -> u64 {
    0
}

/// Calculates the total number of faces amongst the observed cubes that are not connected to
/// another cube.
fn calculate_total_surface_area(observed_cubes: &HashSet<Point3D>) -> u64 {
    let mut total_surface_area = 0;
    for cube in observed_cubes {
        // -dx
        if !observed_cubes.contains(&cube.check_move_point(-1, 0, 0)) {
            total_surface_area += 1;
        }
        // +dx
        if !observed_cubes.contains(&cube.check_move_point(1, 0, 0)) {
            total_surface_area += 1;
        }
        // -dy
        if !observed_cubes.contains(&cube.check_move_point(0, -1, 0)) {
            total_surface_area += 1;
        }
        // +dy
        if !observed_cubes.contains(&cube.check_move_point(0, 1, 0)) {
            total_surface_area += 1;
        }
        // -dz
        if !observed_cubes.contains(&cube.check_move_point(0, 0, -1)) {
            total_surface_area += 1;
        }
        // +dz
        if !observed_cubes.contains(&cube.check_move_point(0, 0, 1)) {
            total_surface_area += 1;
        }
    }
    total_surface_area
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 18 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day18_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(4332, solution);
    }

    /// Tests the Day 18 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day18_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
