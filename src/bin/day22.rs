use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use aoc2022::utils::cartography::{CardinalDirection, Point2D};

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

/// Used to represent the minimum and maximum x- and y-values for the cube sides.
struct MinMax {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl MinMax {
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn contains_point(&self, point: &Point2D) -> bool {
        self.min_x <= point.get_x()
            && self.max_x >= point.get_x()
            && self.min_y <= point.get_y()
            && self.max_y >= point.get_y()
    }
}

/// Type returned from the input parser function.
type ProblemInput = (HashMap<Point2D, TileType>, Vec<Instruction>);

lazy_static!(
    static ref SIDE1_MINMAX: MinMax = MinMax::new(100, 149, 0, 49);
    static ref SIDE2_MINMAX: MinMax = MinMax::new(50, 99, 0, 49);
    static ref SIDE3_MINMAX: MinMax = MinMax::new(50, 99, 50, 99);
    static ref SIDE4_MINMAX: MinMax = MinMax::new(50, 99, 100, 149);
    static ref SIDE5_MINMAX: MinMax = MinMax::new(0, 49, 100, 149);
    static ref SIDE6_MINMAX: MinMax = MinMax::new(0, 49, 150, 199);
);

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
    {
        let mut y = 0;
        for line in input_file_chunks[0].lines() {
            if line.is_empty() {
                continue;
            }
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => {
                        tile_map.insert(Point2D::new(x, y), TileType::Space);
                    }
                    '#' => {
                        tile_map.insert(Point2D::new(x, y), TileType::Wall);
                    }
                    _ => (),
                }
                x += 1;
            }
            y += 1;
        }
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
    let (monkey_map, instructions) = problem_input;
    let start_x = monkey_map
        .keys()
        .filter(|elem| elem.get_y() == 0)
        .map(|elem| elem.get_x())
        .min()
        .unwrap();
    let mut loc = Point2D::new(start_x, 0);
    let mut dirn = CardinalDirection::East;
    for instruct in instructions {
        match instruct {
            Instruction::RotateLeft => dirn = dirn.rotate90_counterclockwise(),
            Instruction::RotateRight => dirn = dirn.rotate90_clockwise(),
            Instruction::Steps { num } => {
                for _ in 0..*num {
                    let check_point =  match dirn {
                        CardinalDirection::North => {
                            let mut temp = loc.check_move_point(0, -1);
                            if !monkey_map.contains_key(&temp) {
                                let new_y = monkey_map
                                    .keys()
                                    .filter(|elem| elem.get_x() == loc.get_x())
                                    .map(|elem| elem.get_y())
                                    .max()
                                    .unwrap();
                                temp.set_y(new_y);
                            }
                            temp
                        }
                        CardinalDirection::East => {
                            let mut temp = loc.check_move_point(1, 0);
                            if !monkey_map.contains_key(&temp) {
                                let new_x = monkey_map
                                    .keys()
                                    .filter(|elem| elem.get_y() == loc.get_y())
                                    .map(|elem| elem.get_x())
                                    .min()
                                    .unwrap();
                                temp.set_x(new_x);
                            }
                            temp
                        }
                        CardinalDirection::South => {
                            let mut temp = loc.check_move_point(0, 1);
                            if !monkey_map.contains_key(&temp) {
                                let new_y = monkey_map
                                    .keys()
                                    .filter(|elem| elem.get_x() == loc.get_x())
                                    .map(|elem| elem.get_y())
                                    .min()
                                    .unwrap();
                                temp.set_y(new_y);
                            }
                            temp
                        }
                        CardinalDirection::West => {
                            let mut temp = loc.check_move_point(-1, 0);
                            if !monkey_map.contains_key(&temp) {
                                let new_x = monkey_map
                                    .keys()
                                    .filter(|elem| elem.get_y() == loc.get_y())
                                    .map(|elem| elem.get_x())
                                    .max()
                                    .unwrap();
                                temp.set_x(new_x);
                            }
                            temp
                        }
                    };
                    if *monkey_map.get(&check_point).unwrap() == TileType::Wall {
                        break;
                    }
                    loc = check_point;
                }
            }
        }
    }
    let facing: i64 = match dirn {
        CardinalDirection::East => 0,
        CardinalDirection::South => 1,
        CardinalDirection::West => 2,
        CardinalDirection::North => 3,
    };
    (loc.get_y() + 1) * 1000 + (loc.get_x() + 1) * 4 + facing
}

/// Solves AOC 2022 Day 22 Part 2 // ###
fn solve_part2(problem_input: &ProblemInput) -> i64 {
    let (monkey_map, instructions) = problem_input;
    let start_x = monkey_map
        .keys()
        .filter(|elem| elem.get_y() == 0)
        .map(|elem| elem.get_x())
        .min()
        .unwrap();
    let mut loc = Point2D::new(start_x, 0);
    let mut dirn = CardinalDirection::East;
    for instruct in instructions {
        match instruct {
            Instruction::RotateLeft => dirn = dirn.rotate90_counterclockwise(),
            Instruction::RotateRight => dirn = dirn.rotate90_clockwise(),
            Instruction::Steps { num } => {
                for _ in 0..*num {
                    let side_num = determine_current_side(&loc);
                    let check_point =  match dirn {
                        CardinalDirection::North => {
                            let mut temp = loc.check_move_point(0, -1);
                            if !monkey_map.contains_key(&temp) {
                                let (new_x, new_y) = {
                                    match side_num {
                                        1 => {
                                            dirn = CardinalDirection::North;
                                            let delta_x = loc.get_x() - SIDE1_MINMAX.min_x;
                                            (SIDE6_MINMAX.min_x + delta_x, SIDE6_MINMAX.max_y)
                                        }
                                        2 => {
                                            dirn = CardinalDirection::East;
                                            let delta_x = loc.get_x() - SIDE2_MINMAX.min_x;
                                            (SIDE6_MINMAX.min_x, SIDE6_MINMAX.min_y + delta_x)
                                        }
                                        5 => {
                                            dirn = CardinalDirection::East;
                                            let delta_x = loc.get_x() - SIDE5_MINMAX.min_x;
                                            (SIDE3_MINMAX.min_x, SIDE3_MINMAX.min_y + delta_x)
                                        }
                                        _ => panic!("shouldn't get here!"),
                                    }
                                };
                                temp = Point2D::new(new_x, new_y);
                            }
                            temp
                        }
                        CardinalDirection::East => {
                            let mut temp = loc.check_move_point(1, 0);
                            if !monkey_map.contains_key(&temp) {
                                let (new_x, new_y) = {
                                    match side_num {
                                        1 => {
                                            dirn = CardinalDirection::West;
                                            let delta_y = loc.get_y() - SIDE1_MINMAX.min_y;
                                            (SIDE4_MINMAX.max_x, SIDE4_MINMAX.max_y - delta_y)
                                        }
                                        3 => {
                                            dirn = CardinalDirection::North;
                                            let delta_y = loc.get_y() - SIDE3_MINMAX.min_y;
                                            (SIDE1_MINMAX.min_x + delta_y, SIDE1_MINMAX.max_y)
                                        }
                                        4 => {
                                            dirn = CardinalDirection::West;
                                            let delta_y = loc.get_y() - SIDE4_MINMAX.min_y;
                                            (SIDE1_MINMAX.max_x, SIDE1_MINMAX.max_y - delta_y)
                                        }
                                        6 => {
                                            dirn = CardinalDirection::North;
                                            let delta_y = loc.get_y() - SIDE6_MINMAX.min_y;
                                            (SIDE4_MINMAX.min_x + delta_y, SIDE4_MINMAX.max_y)
                                        }
                                        _ => panic!("shouldn't get here!"),
                                    }
                                };
                                temp = Point2D::new(new_x, new_y);
                            }
                            temp
                        }
                        CardinalDirection::South => {
                            let mut temp = loc.check_move_point(0, 1);
                            if !monkey_map.contains_key(&temp) {
                                let (new_x, new_y) = {
                                    match side_num {
                                        1 => {
                                            dirn = CardinalDirection::West;
                                            let delta_x = loc.get_x() - SIDE1_MINMAX.min_x;
                                            (SIDE3_MINMAX.max_x, SIDE3_MINMAX.min_y + delta_x)
                                        }
                                        4 => {
                                            dirn = CardinalDirection::West;
                                            let delta_x = loc.get_x() - SIDE4_MINMAX.min_x;
                                            (SIDE6_MINMAX.max_x, SIDE6_MINMAX.min_y + delta_x)
                                        }
                                        6 => {
                                            dirn = CardinalDirection::South;
                                            let delta_x = loc.get_x() - SIDE6_MINMAX.min_x;
                                            (SIDE1_MINMAX.min_x + delta_x, SIDE1_MINMAX.min_y)
                                        }
                                        _ => panic!("shouldn't get here!"),
                                    }
                                };
                                temp = Point2D::new(new_x, new_y);
                            }
                            temp
                        }
                        CardinalDirection::West => {
                            let mut temp = loc.check_move_point(-1, 0);
                            if !monkey_map.contains_key(&temp) {
                                let (new_x, new_y) = {
                                    match side_num {
                                        2 => {
                                            dirn = CardinalDirection::East;
                                            let delta_y = loc.get_y() - SIDE2_MINMAX.min_y;
                                            (SIDE5_MINMAX.min_x, SIDE5_MINMAX.max_y - delta_y)
                                        }
                                        3 => {
                                            dirn = CardinalDirection::South;
                                            let delta_y = loc.get_y() - SIDE3_MINMAX.min_y;
                                            (SIDE5_MINMAX.min_x + delta_y, SIDE5_MINMAX.min_y)
                                        }
                                        5 => {
                                            dirn = CardinalDirection::East;
                                            let delta_y = loc.get_y() - SIDE5_MINMAX.min_y;
                                            (SIDE2_MINMAX.min_x, SIDE2_MINMAX.max_y - delta_y)
                                        }
                                        6 => {
                                            dirn = CardinalDirection::South;
                                            let delta_y = loc.get_y() - SIDE6_MINMAX.min_y;
                                            (SIDE2_MINMAX.min_x + delta_y, SIDE2_MINMAX.min_y)
                                        }
                                        _ => panic!("shouldn't get here!"),
                                    }
                                };
                                temp = Point2D::new(new_x, new_y);
                            }
                            temp
                        }
                    };
                    println!("{:?}", check_point);
                    if *monkey_map.get(&check_point).unwrap() == TileType::Wall {
                        break;
                    }
                    loc = check_point;
                }
            }
        }
    }
    let facing: i64 = match dirn {
        CardinalDirection::East => 0,
        CardinalDirection::South => 1,
        CardinalDirection::West => 2,
        CardinalDirection::North => 3,
    };
    (loc.get_y() + 1) * 1000 + (loc.get_x() + 1) * 4 + facing
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
        return 6
    }
    panic!("Location is not on a cube side! {:?}", loc);
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
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 22 Part 1 solver method against example input 001
    #[test]
    fn test_day22_part1_t001() {
        let input = process_input_file("./input/test/day22_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(6032, solution);
    }

    // /// Tests the Day 22 Part 2 solver method against the actual problem solution.
    // #[test]
    // fn test_day22_part2_t001() {
    //     let input = process_input_file("./input/test/day22_t001.txt");
    //     let solution = solve_part2(&input);
    //     assert_eq!(5031, solution);
    // }
}
