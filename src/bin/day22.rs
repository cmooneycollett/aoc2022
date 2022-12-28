use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use aoc2022::utils::cartography::{CardinalDirection, MinMax2D, Point2D};

const PROBLEM_NAME: &str = "Monkey Map";
const PROBLEM_INPUT_FILE: &str = "./input/day22.txt";
const PROBLEM_DAY: u64 = 22;

/// Represents a single instruction used to navigate the monkey map.
enum Instruction {
    RotateLeft,
    RotateRight,
    Steps { num: u64 },
}

/// Represents a single type of tile on the monkey map.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Space,
    Wall,
}

/// Type returned from the input parser function.
type ProblemInput = (HashMap<Point2D, TileType>, Vec<Instruction>);

// These cube faces are specific to the tile arrangements in the problem input file.
lazy_static! {
    static ref SIDE1_MINMAX: MinMax2D = MinMax2D::new(100, 149, 0, 49);
    static ref SIDE2_MINMAX: MinMax2D = MinMax2D::new(50, 99, 0, 49);
    static ref SIDE3_MINMAX: MinMax2D = MinMax2D::new(50, 99, 50, 99);
    static ref SIDE4_MINMAX: MinMax2D = MinMax2D::new(50, 99, 100, 149);
    static ref SIDE5_MINMAX: MinMax2D = MinMax2D::new(0, 49, 100, 149);
    static ref SIDE6_MINMAX: MinMax2D = MinMax2D::new(0, 49, 150, 199);
}

/// Processes the AOC 2022 Day 22 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 22 input file in the format required by the solver functions.
/// Returned value is tuple containing hashmap with tile locations and vector of navigation
/// instructions.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut tile_map: HashMap<Point2D, TileType> = HashMap::new();
    let mut instructions: Vec<Instruction> = vec![];
    let input_file_chunks = raw_input
        .split("\n\n")
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>();
    // Process the tile map chunk
    let mut y = 0;
    for line in input_file_chunks[0].lines() {
        if line.is_empty() {
            continue;
        }
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => {
                    tile_map.insert(Point2D::new(x as i64, y), TileType::Space);
                }
                '#' => {
                    tile_map.insert(Point2D::new(x as i64, y), TileType::Wall);
                }
                _ => (),
            }
        }
        y += 1;
    }
    // Process the instructions chunk
    let regex_token = Regex::new(r"(L|R|\d+)").unwrap();
    for token in regex_token.find_iter(&input_file_chunks[1]) {
        let token = token.as_str();
        match token {
            "L" => instructions.push(Instruction::RotateLeft),
            "R" => instructions.push(Instruction::RotateRight),
            _ => instructions.push(Instruction::Steps {
                num: token.parse::<u64>().unwrap(),
            }),
        }
    }
    (tile_map, instructions)
}

/// Solves AOC 2022 Day 22 Part 1 // Determines the final password after navigating through the
/// monkey map.
fn solve_part1(problem_input: &ProblemInput) -> i64 {
    // Initialise the starting location and direction for the protagonist
    let (monkey_map, instructions) = problem_input;
    let mut loc = determine_start_location(monkey_map);
    let mut dirn = CardinalDirection::East;
    for instruct in instructions {
        match instruct {
            Instruction::RotateLeft => dirn = dirn.rotate90_counterclockwise(),
            Instruction::RotateRight => dirn = dirn.rotate90_clockwise(),
            Instruction::Steps { num } => {
                for _ in 0..*num {
                    // Calculate the next location from taking step with the edge-wrap rules
                    let next_loc = match dirn {
                        CardinalDirection::North => get_new_loc_north_edgewrap(loc, monkey_map),
                        CardinalDirection::East => get_new_loc_east_edgewrap(loc, monkey_map),
                        CardinalDirection::South => get_new_loc_south_edgewrap(loc, monkey_map),
                        CardinalDirection::West => get_new_loc_west_edgewrap(loc, monkey_map),
                    };
                    // Stop executing the movement instruction if the next loc contains a WALL
                    if *monkey_map.get(&next_loc).unwrap() == TileType::Wall {
                        break;
                    }
                    // Update the location
                    loc = next_loc;
                }
            }
        }
    }
    // Return the final password score from the location and direction
    calculate_final_password_score(&dirn, &loc)
}

/// Solves AOC 2022 Day 22 Part 2 // Determines the final password after navigating through the
/// monkey map using the cube-fold wrapping rules.
fn solve_part2(problem_input: &ProblemInput) -> i64 {
    // Initialise the starting location and direction for the protagonist
    let (monkey_map, instructions) = problem_input;
    let mut loc = determine_start_location(monkey_map);
    let mut dirn = CardinalDirection::East;
    for instruct in instructions {
        match instruct {
            Instruction::RotateLeft => dirn = dirn.rotate90_counterclockwise(),
            Instruction::RotateRight => dirn = dirn.rotate90_clockwise(),
            Instruction::Steps { num } => {
                for _ in 0..*num {
                    // Calculate the next loc and dirn from taking a step using cube-wrap rules
                    let (next_loc, next_dirn) = match dirn {
                        CardinalDirection::North => get_new_loc_dirn_north_cube(loc, monkey_map),
                        CardinalDirection::East => get_new_loc_dirn_east_cube(loc, monkey_map),
                        CardinalDirection::South => get_new_loc_dirn_south_cube(loc, monkey_map),
                        CardinalDirection::West => get_new_loc_dirn_west_cube(loc, monkey_map),
                    };
                    // Stop executing the movement instruction is the next loc contains a WALL
                    if *monkey_map.get(&next_loc).unwrap() == TileType::Wall {
                        break;
                    }
                    // Movement is not blocked, so update the current location and direction
                    loc = next_loc;
                    dirn = next_dirn;
                }
            }
        }
    }
    // Return the final password score from the final direction and location
    calculate_final_password_score(&dirn, &loc)
}

/// Determines the top-left-most location in the monkey map, which will be the starting location.
fn determine_start_location(monkey_map: &HashMap<Point2D, TileType>) -> Point2D {
    let start_x = monkey_map
        .keys()
        .filter(|elem| elem.y() == 0)
        .map(|elem| elem.x())
        .min()
        .unwrap();
    Point2D::new(start_x, 0)
}

/// Gets the new location that would result from the protagonist travelling NORTH by one tile and
/// following the edge-wrap rules (going to a tile not in the map results in the protagonist
/// wrapping around to the other end of the same row or column respectively).
fn get_new_loc_north_edgewrap(loc: Point2D, monkey_map: &HashMap<Point2D, TileType>) -> Point2D {
    let mut temp_loc = loc.peek_move_point(0, -1);
    if !monkey_map.contains_key(&temp_loc) {
        let new_y = monkey_map
            .keys()
            .filter(|elem| elem.x() == loc.x())
            .map(|elem| elem.y())
            .max()
            .unwrap();
        temp_loc.set_y(new_y);
    }
    temp_loc
}

/// Gets the new location that would result from the protagonist travelling EAST by one tile and
/// following the edge-wrap rules (going to a tile not in the map results in the protagonist
/// wrapping around to the other end of the same row or column respectively).
fn get_new_loc_east_edgewrap(loc: Point2D, monkey_map: &HashMap<Point2D, TileType>) -> Point2D {
    let mut temp_loc = loc.peek_move_point(1, 0);
    if !monkey_map.contains_key(&temp_loc) {
        let new_x = monkey_map
            .keys()
            .filter(|elem| elem.y() == loc.y())
            .map(|elem| elem.x())
            .min()
            .unwrap();
        temp_loc.set_x(new_x);
    }
    temp_loc
}

/// Gets the new location that would result from the protagonist travelling SOUTH by one tile and
/// following the edge-wrap rules (going to a tile not in the map results in the protagonist
/// wrapping around to the other end of the same row or column respectively).
fn get_new_loc_south_edgewrap(loc: Point2D, monkey_map: &HashMap<Point2D, TileType>) -> Point2D {
    let mut temp_loc = loc.peek_move_point(0, 1);
    if !monkey_map.contains_key(&temp_loc) {
        let new_y = monkey_map
            .keys()
            .filter(|elem| elem.x() == loc.x())
            .map(|elem| elem.y())
            .min()
            .unwrap();
        temp_loc.set_y(new_y);
    }
    temp_loc
}

/// Gets the new location that would result from the protagonist travelling WEST by one tile and
/// following the edge-wrap rules (going to a tile not in the map results in the protagonist
/// wrapping around to the other end of the same row or column respectively).
fn get_new_loc_west_edgewrap(loc: Point2D, monkey_map: &HashMap<Point2D, TileType>) -> Point2D {
    let mut temp_loc = loc.peek_move_point(-1, 0);
    if !monkey_map.contains_key(&temp_loc) {
        let new_x = monkey_map
            .keys()
            .filter(|elem| elem.y() == loc.y())
            .map(|elem| elem.x())
            .max()
            .unwrap();
        temp_loc.set_x(new_x);
    }
    temp_loc
}

/// Gets the new location that would result from the protagonist travelling NORTH by one tile and
/// following the cube-fold wrapping rules.
fn get_new_loc_dirn_north_cube(
    loc: Point2D,
    monkey_map: &HashMap<Point2D, TileType>,
) -> (Point2D, CardinalDirection) {
    let side_num = determine_current_side(&loc);
    let mut temp_loc = loc.peek_move_point(0, -1);
    let mut temp_dirn = CardinalDirection::North;
    if !monkey_map.contains_key(&temp_loc) {
        let (new_x, new_y) = {
            match side_num {
                1 => {
                    temp_dirn = CardinalDirection::North;
                    let delta_x = loc.x() - SIDE1_MINMAX.min_x();
                    (SIDE6_MINMAX.min_x() + delta_x, SIDE6_MINMAX.max_y())
                }
                2 => {
                    temp_dirn = CardinalDirection::East;
                    let delta_x = loc.x() - SIDE2_MINMAX.min_x();
                    (SIDE6_MINMAX.min_x(), SIDE6_MINMAX.min_y() + delta_x)
                }
                5 => {
                    temp_dirn = CardinalDirection::East;
                    let delta_x = loc.x() - SIDE5_MINMAX.min_x();
                    (SIDE3_MINMAX.min_x(), SIDE3_MINMAX.min_y() + delta_x)
                }
                _ => panic!("shouldn't get here!"),
            }
        };
        temp_loc = Point2D::new(new_x, new_y);
    }
    (temp_loc, temp_dirn)
}

/// Gets the new location that would result from the protagonist travelling EAST by one tile and
/// following the cube-fold wrapping rules.
fn get_new_loc_dirn_east_cube(
    loc: Point2D,
    monkey_map: &HashMap<Point2D, TileType>,
) -> (Point2D, CardinalDirection) {
    let side_num = determine_current_side(&loc);
    let mut temp_loc = loc.peek_move_point(1, 0);
    let mut temp_dirn = CardinalDirection::East;
    if !monkey_map.contains_key(&temp_loc) {
        let (new_x, new_y) = {
            match side_num {
                1 => {
                    temp_dirn = CardinalDirection::West;
                    let delta_y = loc.y() - SIDE1_MINMAX.min_y();
                    (SIDE4_MINMAX.max_x(), SIDE4_MINMAX.max_y() - delta_y)
                }
                3 => {
                    temp_dirn = CardinalDirection::North;
                    let delta_y = loc.y() - SIDE3_MINMAX.min_y();
                    (SIDE1_MINMAX.min_x() + delta_y, SIDE1_MINMAX.max_y())
                }
                4 => {
                    temp_dirn = CardinalDirection::West;
                    let delta_y = loc.y() - SIDE4_MINMAX.min_y();
                    (SIDE1_MINMAX.max_x(), SIDE1_MINMAX.max_y() - delta_y)
                }
                6 => {
                    temp_dirn = CardinalDirection::North;
                    let delta_y = loc.y() - SIDE6_MINMAX.min_y();
                    (SIDE4_MINMAX.min_x() + delta_y, SIDE4_MINMAX.max_y())
                }
                _ => panic!("shouldn't get here!"),
            }
        };
        temp_loc = Point2D::new(new_x, new_y);
    }
    (temp_loc, temp_dirn)
}

/// Gets the new location that would result from the protagonist travelling SOUTH by one tile and
/// following the cube-fold wrapping rules.
fn get_new_loc_dirn_south_cube(
    loc: Point2D,
    monkey_map: &HashMap<Point2D, TileType>,
) -> (Point2D, CardinalDirection) {
    let side_num = determine_current_side(&loc);
    let mut temp_loc = loc.peek_move_point(0, 1);
    let mut temp_dirn = CardinalDirection::South;
    if !monkey_map.contains_key(&temp_loc) {
        let (new_x, new_y) = {
            match side_num {
                1 => {
                    temp_dirn = CardinalDirection::West;
                    let delta_x = loc.x() - SIDE1_MINMAX.min_x();
                    (SIDE3_MINMAX.max_x(), SIDE3_MINMAX.min_y() + delta_x)
                }
                4 => {
                    temp_dirn = CardinalDirection::West;
                    let delta_x = loc.x() - SIDE4_MINMAX.min_x();
                    (SIDE6_MINMAX.max_x(), SIDE6_MINMAX.min_y() + delta_x)
                }
                6 => {
                    temp_dirn = CardinalDirection::South;
                    let delta_x = loc.x() - SIDE6_MINMAX.min_x();
                    (SIDE1_MINMAX.min_x() + delta_x, SIDE1_MINMAX.min_y())
                }
                _ => panic!("shouldn't get here!"),
            }
        };
        temp_loc = Point2D::new(new_x, new_y);
    }
    (temp_loc, temp_dirn)
}

/// Gets the new location that would result from the protagonist travelling WEST by one tile and
/// following the cube-fold wrapping rules.
fn get_new_loc_dirn_west_cube(
    loc: Point2D,
    monkey_map: &HashMap<Point2D, TileType>,
) -> (Point2D, CardinalDirection) {
    let side_num = determine_current_side(&loc);
    let mut temp_loc = loc.peek_move_point(-1, 0);
    let mut temp_dirn = CardinalDirection::West;
    if !monkey_map.contains_key(&temp_loc) {
        let (new_x, new_y) = {
            match side_num {
                2 => {
                    temp_dirn = CardinalDirection::East;
                    let delta_y = loc.y() - SIDE2_MINMAX.min_y();
                    (SIDE5_MINMAX.min_x(), SIDE5_MINMAX.max_y() - delta_y)
                }
                3 => {
                    temp_dirn = CardinalDirection::South;
                    let delta_y = loc.y() - SIDE3_MINMAX.min_y();
                    (SIDE5_MINMAX.min_x() + delta_y, SIDE5_MINMAX.min_y())
                }
                5 => {
                    temp_dirn = CardinalDirection::East;
                    let delta_y = loc.y() - SIDE5_MINMAX.min_y();
                    (SIDE2_MINMAX.min_x(), SIDE2_MINMAX.max_y() - delta_y)
                }
                6 => {
                    temp_dirn = CardinalDirection::South;
                    let delta_y = loc.y() - SIDE6_MINMAX.min_y();
                    (SIDE2_MINMAX.min_x() + delta_y, SIDE2_MINMAX.min_y())
                }
                _ => panic!("shouldn't get here!"),
            }
        };
        temp_loc = Point2D::new(new_x, new_y);
    }
    (temp_loc, temp_dirn)
}

/// Determines what side of the cube that the given location is on.
fn determine_current_side(loc: &Point2D) -> u64 {
    if SIDE1_MINMAX.contains_point(loc) {
        return 1;
    } else if SIDE2_MINMAX.contains_point(loc) {
        return 2;
    } else if SIDE3_MINMAX.contains_point(loc) {
        return 3;
    } else if SIDE4_MINMAX.contains_point(loc) {
        return 4;
    } else if SIDE5_MINMAX.contains_point(loc) {
        return 5;
    } else if SIDE6_MINMAX.contains_point(loc) {
        return 6;
    }
    panic!("Location is not on a cube side! {:?}", loc);
}

/// Calculates the final password score from the given location and direction.
fn calculate_final_password_score(dirn: &CardinalDirection, loc: &Point2D) -> i64 {
    let facing: i64 = match dirn {
        CardinalDirection::East => 0,
        CardinalDirection::South => 1,
        CardinalDirection::West => 2,
        CardinalDirection::North => 3,
    };
    (loc.y() + 1) * 1000 + (loc.x() + 1) * 4 + facing
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 22 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day22_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(149138, solution);
    }

    /// Tests the Day 22 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day22_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(153203, solution);
    }

    /// Tests the Day 22 Part 1 solver method against example input 001
    #[test]
    fn test_day22_part1_t001() {
        let input = process_input_file("./input/test/day22_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(6032, solution);
    }
}
