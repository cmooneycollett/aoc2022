use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::{CardinalDirection, CompassDirection, Point2D};

const PROBLEM_NAME: &str = "Unstable Diffusion";
const PROBLEM_INPUT_FILE: &str = "./input/day23.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day23_t001.txt";
const PROBLEM_DAY: u64 = 23;

/// Processes the AOC 2022 Day 23 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 23 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> HashSet<Point2D> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut elves: HashSet<Point2D> = HashSet::new();
    for (y, line) in raw_input.trim().lines().enumerate() {
        for (x, tile) in line.trim().chars().enumerate() {
            if tile == '#' {
                elves.insert(Point2D::new(x as i64, y as i64));
            }
        }
    }
    elves
}

/// Solves AOC 2022 Day 23 Part 1 // Determines the number of empty tiles within the smallest
/// rectangle containing all elves after 10 rounds of movement.
fn solve_part1(start_elves: &HashSet<Point2D>) -> usize {
    let mut elves = start_elves.clone();
    // Initialise the move check array
    let move_checks: [fn(&Point2D, &HashSet<Point2D>) -> Option<CardinalDirection>; 4] = [
        check_for_north_move,
        check_for_south_move,
        check_for_west_move,
        check_for_east_move,
    ];
    for round in 0..10 {
        let mut new_elf_locations: HashMap<Point2D, Vec<Point2D>> = HashMap::new();
        // check each elf
        for elf_loc in elves.iter() {
            // Check surrounding elves
            let mut no_move = true;
            for s_loc in elf_loc.get_surrounding_points() {
                if elves.contains(&s_loc) {
                    no_move = false;
                    break;
                }
            }
            // Checks if the elf is staying in its current location for the next round
            if no_move {
                if new_elf_locations.contains_key(&elf_loc) {
                    new_elf_locations.get_mut(&elf_loc).unwrap().push(*elf_loc);
                } else {
                    new_elf_locations.insert(*elf_loc, vec![*elf_loc]);
                }
                continue;
            }
            // Check for valid move
            no_move = true;
            let i = round % 4;
            for di in 0..4 {
                if let Some(dirn) = move_checks[(i + di) % 4](&elf_loc, &elves) {
                    let new_loc = match dirn {
                        CardinalDirection::North => elf_loc.check_move_point(0, -1),
                        CardinalDirection::South => elf_loc.check_move_point(0, 1),
                        CardinalDirection::West => elf_loc.check_move_point(-1, 0),
                        CardinalDirection::East => elf_loc.check_move_point(1, 0),
                    };
                    if new_elf_locations.contains_key(&new_loc) {
                        new_elf_locations.get_mut(&new_loc).unwrap().push(*elf_loc);
                    } else {
                        new_elf_locations.insert(new_loc, vec![*elf_loc]);
                    }
                    no_move = false;
                    break;
                }
            }
            // Check if there was no valid move for the elf
            if no_move {
                if new_elf_locations.contains_key(&elf_loc) {
                    new_elf_locations.get_mut(&elf_loc).unwrap().push(*elf_loc);
                } else {
                    new_elf_locations.insert(*elf_loc, vec![*elf_loc]);
                }
                continue;
            }
        }
        // Update the elves
        elves = HashSet::new();
        for (new_loc, old_locs) in new_elf_locations.iter() {
            if old_locs.len() == 1 {
                elves.insert(*new_loc);
            } else if old_locs.len() > 1 {
                elves.extend(old_locs);
            }
        }
    }
    // Find min and max x- and y-values
    calculate_empty_spaces_in_bounding_rect(elves)
}

/// Solves AOC 2022 Day 23 Part 2 // ###
fn solve_part2(_input: &HashSet<Point2D>) -> usize {
    0
}

/// Calculates the number of empty spaces in the smallest rectangle containing each of the elves.
fn calculate_empty_spaces_in_bounding_rect(elves: HashSet<Point2D>) -> usize {
    let min_x = elves.iter().map(|loc| loc.get_x()).min().unwrap();
    let max_x = elves.iter().map(|loc| loc.get_x()).max().unwrap();
    let min_y = elves.iter().map(|loc| loc.get_y()).min().unwrap();
    let max_y = elves.iter().map(|loc| loc.get_y()).max().unwrap();
    let grid_size: usize = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
    grid_size - elves.len()
}

/// Checks if moving to the NORTH is a valid move for the elf.
fn check_for_north_move(elf_loc: &Point2D, elves: &HashSet<Point2D>) -> Option<CardinalDirection> {
    for dirn in [
        CompassDirection::NorthEast,
        CompassDirection::North,
        CompassDirection::NorthWest,
    ] {
        if elves.contains(&elf_loc.check_move_in_direction(dirn)) {
            return None;
        }
    }
    Some(CardinalDirection::North)
}

/// Checks if moving to the SOUTH is a valid move for the elf.
fn check_for_south_move(elf_loc: &Point2D, elves: &HashSet<Point2D>) -> Option<CardinalDirection> {
    for dirn in [
        CompassDirection::SouthEast,
        CompassDirection::South,
        CompassDirection::SouthWest,
    ] {
        if elves.contains(&elf_loc.check_move_in_direction(dirn)) {
            return None;
        }
    }
    Some(CardinalDirection::South)
}

/// Checks if moving to the WEST is a valid move for the elf.
fn check_for_west_move(elf_loc: &Point2D, elves: &HashSet<Point2D>) -> Option<CardinalDirection> {
    for dirn in [
        CompassDirection::NorthWest,
        CompassDirection::West,
        CompassDirection::SouthWest,
    ] {
        if elves.contains(&elf_loc.check_move_in_direction(dirn)) {
            return None;
        }
    }
    Some(CardinalDirection::West)
}

/// Checks if moving to the EAST is a valid move for the elf.
fn check_for_east_move(elf_loc: &Point2D, elves: &HashSet<Point2D>) -> Option<CardinalDirection> {
    for dirn in [
        CompassDirection::NorthEast,
        CompassDirection::East,
        CompassDirection::SouthEast,
    ] {
        if elves.contains(&elf_loc.check_move_in_direction(dirn)) {
            return None;
        }
    }
    Some(CardinalDirection::East)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 23 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day23_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(3689, solution);
    }

    /// Tests the Day 23 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day23_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 23 Part 1 solver method against example input 001.
    #[test]
    fn test_day23_part1_t001() {
        let input = process_input_file("./input/test/day23_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(110, solution);
    }
}
