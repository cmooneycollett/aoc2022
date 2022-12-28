use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;

use aoc2022::utils::cartography::{CardinalDirection, CompassDirection, Point2D};

const PROBLEM_NAME: &str = "Unstable Diffusion";
const PROBLEM_INPUT_FILE: &str = "./input/day23.txt";
const PROBLEM_DAY: u64 = 23;

/// Type declaration to simplify the declaration of the move checks function slice.
type MoveCheckSlice = [fn(&Point2D, &HashSet<Point2D>) -> Option<CardinalDirection>; 4];

lazy_static! {
    static ref MOVE_CHECKS: MoveCheckSlice = [
        check_for_north_move,
        check_for_south_move,
        check_for_west_move,
        check_for_east_move,
    ];
}

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
    for round in 0..10 {
        conduct_diffusion_round(&mut elves, round);
    }
    // Find min and max x- and y-values
    calculate_empty_spaces_in_bounding_rect(elves)
}

/// Solves AOC 2022 Day 23 Part 2 // Determines the first round in which none of the elves move.
fn solve_part2(start_elves: &HashSet<Point2D>) -> usize {
    let mut elves = start_elves.clone();
    let mut round: usize = 0;
    loop {
        // Check if none of the elves move during the diffusion round
        if conduct_diffusion_round(&mut elves, round) {
            return round + 1;
        }
        round += 1;
    }
}

/// Conducts a single diffusion round and updates the elf locations. Returns true if none of the
/// elves moved during the round.
fn conduct_diffusion_round(elf_locs: &mut HashSet<Point2D>, round: usize) -> bool {
    let mut new_elf_locations: HashMap<Point2D, Vec<Point2D>> = HashMap::new();
    let mut no_move_count = 0;
    // check each elf
    for old_loc in elf_locs.iter() {
        // Check surrounding elves
        let mut no_move = true;
        for s_loc in old_loc.get_surrounding_points() {
            if elf_locs.contains(&s_loc) {
                no_move = false;
                break;
            }
        }
        // Checks if the elf is staying in its current location for the next round
        if no_move {
            no_move_count += 1;
            if let Entry::Vacant(e) = new_elf_locations.entry(*old_loc) {
                e.insert(vec![*old_loc]);
            } else {
                new_elf_locations.get_mut(old_loc).unwrap().push(*old_loc);
            }
            continue;
        }
        // Check for valid move
        no_move = true;
        let i = round % 4;
        for di in 0..4 {
            if let Some(dirn) = MOVE_CHECKS[(i + di) % 4](old_loc, elf_locs) {
                let new_loc = match dirn {
                    CardinalDirection::North => old_loc.peek_move_point(0, -1),
                    CardinalDirection::South => old_loc.peek_move_point(0, 1),
                    CardinalDirection::West => old_loc.peek_move_point(-1, 0),
                    CardinalDirection::East => old_loc.peek_move_point(1, 0),
                };
                if let Entry::Vacant(e) = new_elf_locations.entry(new_loc) {
                    e.insert(vec![*old_loc]);
                } else {
                    new_elf_locations.get_mut(&new_loc).unwrap().push(*old_loc);
                }
                no_move = false;
                break;
            }
        }
        // Check if there was no valid move for the elf
        if no_move {
            no_move_count += 1;
            if let Entry::Vacant(e) = new_elf_locations.entry(*old_loc) {
                e.insert(vec![*old_loc]);
            } else {
                new_elf_locations.get_mut(old_loc).unwrap().push(*old_loc);
            }
            continue;
        }
    }
    // Update the elves
    *elf_locs = HashSet::new();
    for (new_loc, old_locs) in new_elf_locations.iter() {
        match old_locs.len().cmp(&1) {
            Ordering::Equal => _ = elf_locs.insert(*new_loc),
            Ordering::Greater => elf_locs.extend(old_locs),
            Ordering::Less => panic!("Should not have empty value in new elf locations!"),
        }
    }
    // Check the number of elves that did not move
    no_move_count == elf_locs.len()
}

/// Calculates the number of empty spaces in the smallest rectangle containing each of the elves.
fn calculate_empty_spaces_in_bounding_rect(elves: HashSet<Point2D>) -> usize {
    let min_x = elves.iter().map(|loc| loc.x()).min().unwrap();
    let max_x = elves.iter().map(|loc| loc.x()).max().unwrap();
    let min_y = elves.iter().map(|loc| loc.y()).min().unwrap();
    let max_y = elves.iter().map(|loc| loc.y()).max().unwrap();
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
        let solution = solve_part2(&input);
        assert_eq!(965, solution);
    }

    /// Tests the Day 23 Part 1 solver method against example input 001.
    #[test]
    fn test_day23_part1_t001() {
        let input = process_input_file("./input/test/day23_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(110, solution);
    }

    /// Tests the Day 23 Part 1 solver method against example input 001.
    #[test]
    fn test_day23_part2_t001() {
        let input = process_input_file("./input/test/day23_t001.txt");
        let solution = solve_part2(&input);
        assert_eq!(20, solution);
    }
}
