use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Pyroclastic Flow";
const PROBLEM_INPUT_FILE: &str = "./input/day17.txt";
const PROBLEM_DAY: u64 = 17;

#[derive(Copy, Clone, PartialEq, Eq)]
enum RockType {
    RockHorizBar,
    RockCross,
    RockL,
    RockVertBar,
    RockSquare,
}

/// Processes the AOC 2022 Day 17 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 17 input file in the format required by the solver functions.
/// Returned value is vector of chars from the input file.
fn process_input_file(filename: &str) -> Vec<char> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    return raw_input.trim().chars().collect::<Vec<char>>();
}

/// Solves AOC 2022 Day 17 Part 1 // ###
fn solve_part1(input: &[char]) -> i64 {
    // Generate rock type cycle
    let rock_types: Vec<RockType> = vec![
        RockType::RockHorizBar,
        RockType::RockCross,
        RockType::RockL,
        RockType::RockVertBar,
        RockType::RockSquare,
    ];
    let mut rock_type_cycle = rock_types.iter().cycle();
    // Generate jet pattern cycle
    let mut jet_patt_cycle = input.iter().cycle();
    // Initialise set to record rock location
    let mut rock_max_height: i64 = -1;
    let mut rock_locations: HashSet<Point2D> = HashSet::new();
    for x in 0..7 {
        rock_locations.insert(Point2D::new(x, -1));
    }
    for _ in 0..2022 {
        // Generate new rock
        let rock_type = *rock_type_cycle.next().unwrap();
        let mut rock = generate_new_rock(rock_type, rock_max_height + 4);
        loop {
            // Push rock
            let dirn = jet_patt_cycle.next().unwrap();
            match dirn {
                '<' => {
                    let mut has_collision = false;
                    let mut new_rock: HashSet<Point2D> = HashSet::new();
                    for tile in rock.iter() {
                        if tile.get_x() == 0
                            || rock_locations.contains(&tile.check_move_point(-1, 0))
                        {
                            has_collision = true;
                            break;
                        }
                        new_rock.insert(tile.check_move_point(-1, 0));
                    }
                    if !has_collision {
                        rock = new_rock;
                    }
                }
                '>' => {
                    let mut has_collision = false;
                    let mut new_rock: HashSet<Point2D> = HashSet::new();
                    for tile in rock.iter() {
                        if tile.get_x() == 6
                            || rock_locations.contains(&tile.check_move_point(1, 0))
                        {
                            has_collision = true;
                            break;
                        }
                        new_rock.insert(tile.check_move_point(1, 0));
                    }
                    if !has_collision {
                        rock = new_rock;
                    }
                }
                _ => panic!("Bad jet pattern character!"),
            }
            // Check for down movement
            let mut has_collision = false;
            let mut new_rock: HashSet<Point2D> = HashSet::new();
            for tile in rock.iter() {
                if rock_locations.contains(&tile.check_move_point(0, -1)) {
                    has_collision = true;
                    break;
                }
                new_rock.insert(tile.check_move_point(0, -1));
            }
            // Check if the rock cannot move down
            if !has_collision {
                rock = new_rock;
            } else {
                let new_rock_max_height = rock.iter().map(|point| point.get_y()).max().unwrap();
                if new_rock_max_height > rock_max_height {
                    rock_max_height = new_rock_max_height;
                }
                rock_locations.extend(rock);
                break;
            }
        }
    }
    rock_max_height + 1
}

/// Solves AOC 2022 Day 17 Part 2 // ###
fn solve_part2(_input: &[char]) -> i64 {
    0
}

/// Generates a new set of points representing the given rock type and at the specified height for
/// bottom left.
fn generate_new_rock(rock_type: RockType, y: i64) -> HashSet<Point2D> {
    let mut output: HashSet<Point2D> = HashSet::new();
    match rock_type {
        RockType::RockHorizBar => {
            let root_tile = Point2D::new(2, y);
            output.insert(root_tile);
            output.insert(root_tile.check_move_point(1, 0));
            output.insert(root_tile.check_move_point(2, 0));
            output.insert(root_tile.check_move_point(3, 0));
        }
        RockType::RockCross => {
            let root_tile = Point2D::new(3, y + 2);
            output.insert(root_tile);
            output.insert(root_tile.check_move_point(-1, -1));
            output.insert(root_tile.check_move_point(0, -1));
            output.insert(root_tile.check_move_point(1, -1));
            output.insert(root_tile.check_move_point(0, -2));
        }
        RockType::RockL => {
            let root_tile = Point2D::new(2, y);
            output.insert(root_tile);
            output.insert(root_tile.check_move_point(1, 0));
            output.insert(root_tile.check_move_point(2, 0));
            output.insert(root_tile.check_move_point(2, 1));
            output.insert(root_tile.check_move_point(2, 2));
        }
        RockType::RockVertBar => {
            let root_tile = Point2D::new(2, y);
            output.insert(root_tile);
            output.insert(root_tile.check_move_point(0, 1));
            output.insert(root_tile.check_move_point(0, 2));
            output.insert(root_tile.check_move_point(0, 3));
        }
        RockType::RockSquare => {
            let root_tile = Point2D::new(2, y);
            output.insert(root_tile);
            output.insert(root_tile.check_move_point(1, 0));
            output.insert(root_tile.check_move_point(0, 1));
            output.insert(root_tile.check_move_point(1, 1));
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 17 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day17_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(3071, solution);
    }

    /// Tests the Day 17 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day17_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
