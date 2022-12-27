use core::panic;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque, HashSet};
use std::fs;
use std::time::Instant;
use std::vec;

use aoc2022::utils::cartography::{CardinalDirection, Point2D};

const PROBLEM_NAME: &str = "Blizzard Basin";
// const PROBLEM_INPUT_FILE: &str = "./input/day24.txt";
const PROBLEM_INPUT_FILE: &str = "./input/test/day24_t001.txt";
const PROBLEM_DAY: u64 = 24;

/// Represents the area of ground bounding the blizzards.
struct MinMax2D {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl MinMax2D {
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

type ProblemInput = (
    Point2D,
    Point2D,
    MinMax2D,
    HashMap<Point2D, Vec<CardinalDirection>>,
);

/// Processes the AOC 2022 Day 24 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 24 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut start_loc: Option<Point2D> = None;
    let mut end_loc: Option<Point2D> = None;
    let mut blizzard_locs: HashMap<Point2D, Vec<CardinalDirection>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in raw_input.trim().lines().enumerate() {
        if y > max_y {
            max_y = y;
        }
        for (x, tile) in line.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }
            if y == 0 && tile == '.' {
                start_loc = Some(Point2D::new(x as i64, y as i64));
                break;
            }
            let loc = Point2D::new(x as i64, y as i64);
            match tile {
                '^' => _ = blizzard_locs.insert(loc, vec![CardinalDirection::North]),
                '>' => _ = blizzard_locs.insert(loc, vec![CardinalDirection::East]),
                'v' => _ = blizzard_locs.insert(loc, vec![CardinalDirection::South]),
                '<' => _ = blizzard_locs.insert(loc, vec![CardinalDirection::West]),
                '.' => end_loc = Some(loc),
                _ => (),
            };
        }
    }
    let minmax = MinMax2D::new(1, max_x as i64 - 1, 1, max_y as i64 - 1);
    (start_loc.unwrap(), end_loc.unwrap(), minmax, blizzard_locs)
}

/// Solves AOC 2022 Day 24 Part 1 // ###
fn solve_part1(input: &ProblemInput) -> u64 {
    let (start_loc, end_loc, minmax, blizzard_locs) = input;
    let mut blizzard_state: (u64, HashMap<Point2D, Vec<CardinalDirection>>, HashSet<Point2D>) =
        (0, blizzard_locs.clone(), blizzard_locs.keys().copied().collect::<HashSet<Point2D>>());
    let mut visit_queue: VecDeque<(u64, Point2D)> = VecDeque::from([(0, *start_loc)]);
    while !visit_queue.is_empty() {
        // Get the next location to visit
        let (minutes, loc) = visit_queue.pop_front().unwrap();
        if blizzard_state.2.contains(&loc) {
            continue;
        }
        // Update the blizzard state
        if minutes > blizzard_state.0 {
            blizzard_state = update_blizzard_state(&blizzard_state, minmax);
        }
        // Check if the end location has been reached
        if loc == *end_loc {
            return minutes;
        }
        visit_queue.push_back((minutes + 1, loc));
        for next_loc in get_valid_next_locations(&loc, minmax, &blizzard_state) {
            visit_queue.push_back((minutes + 1, next_loc));
        }
    }
    panic!("Should not get here!");
}

fn get_valid_next_locations(
    loc: &Point2D,
    minmax: &MinMax2D,
    blizzard_state: &(u64, HashMap<Point2D, Vec<CardinalDirection>>, HashSet<Point2D>),
) -> Vec<Point2D> {
    let mut output: Vec<Point2D> = vec![];
    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let next_loc = loc.check_move_point(dx, dy);
        if next_loc.get_x() < minmax.min_x || next_loc.get_x() > minmax.max_x || next_loc.get_y() < minmax.min_y || next_loc.get_y() > minmax.max_y {
            continue;
        }
        if blizzard_state.2.contains(&next_loc) {
            continue;
        }
        output.push(next_loc);
    }
    output
}

fn update_blizzard_state(
    blizzard_state: &(u64, HashMap<Point2D, Vec<CardinalDirection>>, HashSet<Point2D>),
    minmax: &MinMax2D,
) -> (u64, HashMap<Point2D, Vec<CardinalDirection>>, HashSet<Point2D>) {
    let mut new_blizzard_map: HashMap<Point2D, Vec<CardinalDirection>> = HashMap::new();
    for (loc, blizzards) in blizzard_state.1.iter() {
        for bliz in blizzards {
            let new_loc = match bliz {
                CardinalDirection::North => {
                    let mut temp_loc = loc.check_move_point(0, -1);
                    if temp_loc.get_y() < minmax.min_y {
                        temp_loc.set_y(minmax.max_y);
                    }
                    temp_loc
                }
                CardinalDirection::East => {
                    let mut temp_loc = loc.check_move_point(1, 0);
                    if temp_loc.get_x() > minmax.max_x {
                        temp_loc.set_x(minmax.min_x);
                    }
                    temp_loc
                }
                CardinalDirection::South => {
                    let mut temp_loc = loc.check_move_point(0, 1);
                    if temp_loc.get_y() > minmax.max_y {
                        temp_loc.set_y(minmax.min_y);
                    }
                    temp_loc
                }
                CardinalDirection::West => {
                    let mut temp_loc = loc.check_move_point(-1, 0);
                    if temp_loc.get_x() < minmax.min_x {
                        temp_loc.set_x(minmax.max_x);
                    }
                    temp_loc
                }
            };
            if let Entry::Vacant(e) = new_blizzard_map.entry(new_loc) {
                e.insert(vec![*bliz]);
            } else {
                new_blizzard_map.get_mut(&new_loc).unwrap().push(*bliz);
            }
        }
    }
    (blizzard_state.0 + 1, new_blizzard_map.clone(), new_blizzard_map.keys().copied().collect::<HashSet<Point2D>>())
}

/// Solves AOC 2022 Day 24 Part 2 // ###
fn solve_part2(_input: &ProblemInput) -> String {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 24 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day24_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 24 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day24_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
