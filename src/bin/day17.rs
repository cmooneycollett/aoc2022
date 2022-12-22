use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Pyroclastic Flow";
const PROBLEM_INPUT_FILE: &str = "./input/day17.txt";
const PROBLEM_DAY: u64 = 17;

const PART1_ROCKS: i64 = 2022;
const PART2_ROCKS: i64 = 1_000_000_000_000;
const PART2_SAMPLE_SIZE: i64 = 10000;

#[derive(Copy, Clone, PartialEq, Eq, EnumIter)]
enum RockType {
    RockHorizBar,
    RockCross,
    RockL,
    RockVertBar,
    RockSquare,
}

/// Represents a state of the cave after a rock has fallen.
#[derive(Copy, Clone, PartialEq, Eq)]
struct CaveState {
    max_height: i64,
    jet_index: i64,
    last_rock: RockType,
}

impl CaveState {
    pub fn new(max_height: i64, jet_index: i64, last_rock: RockType) -> Self {
        Self {
            max_height,
            jet_index,
            last_rock,
        }
    }
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

/// Solves AOC 2022 Day 17 Part 1 // Simulates sequence of 2022 rocks falling and returns the
/// maximum height of the rock at the end.
fn solve_part1(jet_chars: &[char]) -> i64 {
    let rock_states = conduct_rock_simulation(jet_chars, PART1_ROCKS);
    rock_states.last().unwrap().max_height
}

/// Solves AOC 2022 Day 17 Part 2 // Simulates sequence of 100 trillion rocks falling and returns
/// the maximum height of the resulting rock formation.
fn solve_part2(jet_chars: &[char]) -> i64 {
    let rock_states = conduct_rock_simulation(jet_chars, PART2_SAMPLE_SIZE);
    // Now try to find the offset, and period of the repeating height changes
    if let Some(max_rock_height) = calculate_rock_height_for_total_rocks(rock_states, PART2_ROCKS) {
        return max_rock_height;
    }
    panic!(
        "Day 17 Part 2 - did not find repeating period in sample size {}",
        PART2_SAMPLE_SIZE
    );
}

/// Calculates the resulting maximum rock height for the given total steps based on the rock states.
fn calculate_rock_height_for_total_rocks(
    rock_states: Vec<CaveState>,
    total_steps: i64,
) -> Option<i64> {
    let max_period_len = (rock_states.len() / 2) as i64;
    for period_len in 1..=max_period_len {
        for offset in 0..max_period_len {
            if offset + period_len * 2 >= max_period_len {
                break;
            }
            // Extract states from rock states
            let state0 = rock_states[offset as usize];
            let state1 = rock_states[(offset + period_len) as usize];
            let state2 = rock_states[(offset + period_len * 2) as usize];
            // Calculate the deltas for max height between the states
            let delta0 = state1.max_height - state0.max_height;
            let delta1 = state2.max_height - state1.max_height;
            // Check if the offset and length for the repeating period has been found
            if delta0 == delta1
                && state0.jet_index == state1.jet_index
                && state1.jet_index == state2.jet_index
                && state0.last_rock == state1.last_rock
                && state1.last_rock == state2.last_rock
            {
                // Calculate the maximum rock height after conducting the total number of steps
                let remaining_steps = total_steps - offset;
                let full_periods_remaining = remaining_steps / period_len;
                let extra_steps = remaining_steps % period_len;
                let steps_from_full_periods = full_periods_remaining * delta1;
                let steps_from_extra = rock_states[(offset + extra_steps - 1) as usize].max_height
                    - rock_states[offset as usize].max_height;
                return Some(state0.max_height + steps_from_extra + steps_from_full_periods);
            }
        }
    }
    None
}

/// Conducts the rock fall simulation for the given number of rocks and the gas jet directions.
/// Returns the cave states when each of the dropped rocks come to rest.
fn conduct_rock_simulation(jet_chars: &[char], num_rocks: i64) -> Vec<CaveState> {
    // Generate rock type cycle
    let mut rock_type_cycle = RockType::iter().cycle();
    // Generate jet pattern cycle
    let mut jet_patt_cycle = jet_chars.iter().cycle();
    // Initialise set to record rock location
    let mut rock_max_y: i64 = -1;
    let mut rock_locations: HashSet<Point2D> = HashSet::new();
    for x in 0..7 {
        rock_locations.insert(Point2D::new(x, -1));
    }
    // Records current max rock height and jet pattern index
    let mut rock_states: Vec<CaveState> = vec![];
    let mut jets_used = 0;
    for _ in 0..num_rocks {
        // Generate new rock
        let rock_type = rock_type_cycle.next().unwrap();
        let mut rock = generate_new_rock(rock_type, rock_max_y + 4);
        loop {
            // Push rock
            let dirn = jet_patt_cycle.next().unwrap();
            jets_used += 1;
            match dirn {
                '<' => {
                    handle_gas_jet_left(&mut rock, &rock_locations);
                }
                '>' => {
                    handle_gas_jet_right(&mut rock, &rock_locations);
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
                if new_rock_max_height > rock_max_y {
                    rock_max_y = new_rock_max_height;
                }
                rock_locations.extend(rock);
                break;
            }
        }
        // Record the current maximum height of the rock formation
        rock_states.push(CaveState::new(
            rock_max_y + 1,
            jets_used % jet_chars.len() as i64,
            rock_type,
        ));
    }
    rock_states
}

/// Handles a right-directed gas jet trying to move a rock.
fn handle_gas_jet_right(rock: &mut HashSet<Point2D>, rock_locations: &HashSet<Point2D>) {
    let mut has_collision = false;
    let mut new_rock: HashSet<Point2D> = HashSet::new();
    for tile in rock.iter() {
        if tile.get_x() == 6 || rock_locations.contains(&tile.check_move_point(1, 0)) {
            has_collision = true;
            break;
        }
        new_rock.insert(tile.check_move_point(1, 0));
    }
    if !has_collision {
        *rock = new_rock;
    }
}

/// Handles a left-directed gas jet trying to move a rock.
fn handle_gas_jet_left(rock: &mut HashSet<Point2D>, rock_locations: &HashSet<Point2D>) {
    let mut has_collision = false;
    let mut new_rock: HashSet<Point2D> = HashSet::new();
    for tile in rock.iter() {
        if tile.get_x() == 0 || rock_locations.contains(&tile.check_move_point(-1, 0)) {
            has_collision = true;
            break;
        }
        new_rock.insert(tile.check_move_point(-1, 0));
    }
    if !has_collision {
        *rock = new_rock;
    }
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
    fn test_day17_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(3071, solution);
    }

    /// Tests the Day 17 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day17_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1523615160362, solution);
    }

    /// Tests the Day 17 Part 1 solver method against the example input 001.
    #[test]
    fn test_day17_part1_t001() {
        let input = process_input_file("./input/test/day17_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(3068, solution);
    }

    /// Tests the Day 17 Part 2 solver method against the example input 001.
    #[test]
    fn test_day17_part2_t001() {
        let input = process_input_file("./input/test/day17_t001.txt");
        let solution = solve_part2(&input);
        assert_eq!(1514285714288, solution);
    }
}
