use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point3D;

const PROBLEM_NAME: &str = "Boiling Boulders";
const PROBLEM_INPUT_FILE: &str = "./input/day18.txt";
const PROBLEM_DAY: u64 = 18;

/// Used to record the minimum and maximum axis values amongst the observed cubes.
struct MinMax3D {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

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
fn solve_part2(observed_cubes: &HashSet<Point3D>) -> u64 {
    calculate_external_surface_area(observed_cubes)
}

/// Calculates the external surface of the observed cubes using a BFS search method.
fn calculate_external_surface_area(observed_cubes: &HashSet<Point3D>) -> u64 {
    let minmax = calculate_minmax_for_observed_cubes(observed_cubes);
    let start_cube = Point3D::new(minmax.min_x, minmax.min_y, minmax.min_z);
    // Initialise the visit queue and visited locations set
    let mut visit_queue: VecDeque<Point3D> = VecDeque::new();
    visit_queue.push_back(start_cube);
    let mut visited: HashSet<Point3D> = HashSet::new();
    visited.insert(start_cube);
    let mut external_surface_area = 0;
    // Keep going until there are no more cubes to visit
    while !visit_queue.is_empty() {
        // Get the next location to check
        let cube = visit_queue.pop_front().unwrap();
        for adj_cube in get_adjacent_cubes(&cube) {
            // Skip the adjacent cube if it's outside bounding volume or already visited
            if visited.contains(&adj_cube)
                || check_if_adjacent_cube_is_out_of_bounds(&adj_cube, &minmax)
            {
                continue;
            }
            // Check if an external cube side has been found
            if observed_cubes.contains(&adj_cube) {
                external_surface_area += 1;
                continue;
            }
            // Visit the adjacent cube
            visited.insert(adj_cube);
            visit_queue.push_back(adj_cube);
        }
    }
    external_surface_area
}

/// Checks if the given adjacent cube is out of bounds. Returns true if the adjacent cube is outside
/// of the bounding volume. Otherwise, returns false;
fn check_if_adjacent_cube_is_out_of_bounds(adj_cube: &Point3D, minmax: &MinMax3D) -> bool {
    return adj_cube.get_x() < minmax.min_x
        || adj_cube.get_x() > minmax.max_x
        || adj_cube.get_y() < minmax.min_y
        || adj_cube.get_y() > minmax.max_y
        || adj_cube.get_z() < minmax.min_z
        || adj_cube.get_z() > minmax.max_z;
}

/// Calculates the total number of faces amongst the observed cubes that are not connected to
/// another cube.
fn calculate_total_surface_area(observed_cubes: &HashSet<Point3D>) -> u64 {
    let mut total_surface_area = 0;
    for cube in observed_cubes {
        for adj_cube in get_adjacent_cubes(cube) {
            if !observed_cubes.contains(&adj_cube) {
                total_surface_area += 1;
            }
        }
    }
    total_surface_area
}

/// Gets the cubes adjacent to the given cube (not including diagonals).
fn get_adjacent_cubes(cube: &Point3D) -> Vec<Point3D> {
    let mut output: Vec<Point3D> = vec![];
    // -dx
    output.push(cube.check_move_point(-1, 0, 0));
    // +dx
    output.push(cube.check_move_point(1, 0, 0));
    // -dy
    output.push(cube.check_move_point(0, -1, 0));
    // +dy
    output.push(cube.check_move_point(0, 1, 0));
    // -dz
    output.push(cube.check_move_point(0, 0, -1));
    // +dz
    output.push(cube.check_move_point(0, 0, 1));
    // Result
    output
}

/// Determines the minimum and maximum x-, y- and z-values for the volume that entirely contains the
/// observed cubes such that none of the observed cubes are at the edge. These x-, y- an z-values
/// (min and max) are calculated by finding the minimum and maximum x-, y- and z-values amongst the
/// observed cubes, then subtracting 1 for the minimum values and adding 1 for the maximum values.
fn calculate_minmax_for_observed_cubes(observed_cubes: &HashSet<Point3D>) -> MinMax3D {
    // X
    let min_x = observed_cubes
        .iter()
        .map(|elem| elem.get_x())
        .min()
        .unwrap();
    let max_x = observed_cubes
        .iter()
        .map(|elem| elem.get_x())
        .max()
        .unwrap();
    // Y
    let min_y = observed_cubes
        .iter()
        .map(|elem| elem.get_y())
        .min()
        .unwrap();
    let max_y = observed_cubes
        .iter()
        .map(|elem| elem.get_y())
        .max()
        .unwrap();
    // Z
    let min_z = observed_cubes
        .iter()
        .map(|elem| elem.get_z())
        .min()
        .unwrap();
    let max_z = observed_cubes
        .iter()
        .map(|elem| elem.get_z())
        .max()
        .unwrap();
    MinMax3D {
        min_x: min_x - 1,
        max_x: max_x + 1,
        min_y: min_y - 1,
        max_y: max_y + 1,
        min_z: min_z - 1,
        max_z: max_z + 1,
    }
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
        let solution = solve_part2(&input);
        assert_eq!(2524, solution);
    }

    /// Tests the Day 18 Part 1 solver method against example input 001.
    #[test]
    fn test_day18_p1_t001() {
        let input = process_input_file("./input/test/day18_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(64, solution);
    }

    /// Tests the Day 18 Part 1 solver method against example input 001.
    #[test]
    fn test_day18_p2_t001() {
        let input = process_input_file("./input/test/day18_t001.txt");
        let solution = solve_part2(&input);
        assert_eq!(58, solution);
    }
}
