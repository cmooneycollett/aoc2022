use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Proboscidea Volcanium";
const PROBLEM_INPUT_FILE: &str = "./input/day16.txt";
const PROBLEM_DAY: u64 = 16;

const PART1_MINUTES: u64 = 30;
const PART2_MINUTES: u64 = 26;

/// Processes the AOC 2022 Day 16 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 16 input file in the format required by the solver functions.
/// Returned value is tuple containing hashmaps with the valve flow rates and valve connections.
fn process_input_file(filename: &str) -> (HashMap<String, u64>, HashMap<String, Vec<String>>) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_line = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)$",
    )
    .unwrap();
    let mut valve_flow_rates: HashMap<String, u64> = HashMap::new();
    let mut valve_connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in raw_input.lines() {
        // Trim input line and ignore empty line
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Extract field data from input line
        let caps = regex_line.captures(line).unwrap();
        let valve = &caps[1];
        let flow_rate = caps[2].parse::<u64>().unwrap();
        let connections = caps[3]
            .split(", ")
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>();
        valve_flow_rates.insert(valve.to_string(), flow_rate);
        valve_connections.insert(valve.to_string(), connections);
    }
    (valve_flow_rates, valve_connections)
}

/// Solves AOC 2022 Day 16 Part 1 // Gets the maximum pressure that can be released by opening
/// valves in the volcano over 30 minutes.
fn solve_part1(input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    // Calculate the valve activation times from 
    let (valve_flow_rates, valve_connections) = input;
    let valve_activation_times = &get_valve_activation_times(valve_flow_rates, valve_connections);
    let possible_paths = determine_possible_paths("AA", valve_activation_times, PART1_MINUTES);
    let mut max_pressure_released = 0;
    for path in possible_paths.iter() {
        let pressure_released = get_pressure_released_for_path(
            path,
            valve_flow_rates,
            valve_activation_times,
            PART1_MINUTES,
        );
        if pressure_released > max_pressure_released {
            max_pressure_released = pressure_released;
        }
    }
    max_pressure_released
}

/// Determines the amount of pressure released over the allowed time by following the given path.
/// The time required to move to and activate a valve is provided as parameter to this function.
fn get_pressure_released_for_path(
    path: &Vec<String>,
    valve_flow_rates: &HashMap<String, u64>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64,
) -> u64 {
    let mut minutes_remaining = minutes_allowed;
    let mut pressure_per_minute = 0;
    let mut total_pressure_released = 0;
    // Start from the second element, since the first element is not moved TO
    for i in 1..path.len() {
        // Get activation time
        let activation_time = valve_activation_times
            .get(&path[i - 1])
            .unwrap()
            .get(&path[i])
            .unwrap();
        // Sum up pressure released while travelling to and activating valve
        total_pressure_released += pressure_per_minute * activation_time;
        // Add new valve's flow rate to the pressure released per minute
        pressure_per_minute += valve_flow_rates.get(&path[i]).unwrap();
        // Reduce the time remaining by the activation time
        minutes_remaining -= activation_time;
    }
    // Use up the remaining time to release pressure
    total_pressure_released += pressure_per_minute * minutes_remaining;
    // Result the resulting pressure released
    total_pressure_released
}

/// Solves AOC 2022 Day 16 Part 2 // Gets the maximum pressures that can be released by opening
/// valves alongside the elephant over 26 minutes.
fn solve_part2(input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    let (valve_flow_rates, valve_connections) = input;
    let valve_activation_times = &get_valve_activation_times(valve_flow_rates, valve_connections);
    // Find the protagonist paths
    let possible_paths = determine_possible_paths("AA", valve_activation_times, PART2_MINUTES);
    let mut maximum_pressure_released = 0;
    // testing vars
    let mut protag_path_count = 0;
    for protagonist_path in possible_paths.iter() {
        // testing vars
        protag_path_count += 1;
        if protag_path_count % 100 == 0 {
            println!("[+] Checking protagonist path {} / {}", protag_path_count, possible_paths.len());
        }
        // Find the paths the elephant could take for a given protagonist path
        let elephant_paths = get_elephant_paths("AA", protagonist_path, valve_activation_times, PART2_MINUTES);
        for ele_path in elephant_paths.iter() {
            let mut pressure_released = 0;
            // Calculate the pressure released over the allowed time by the protagonist and elephant
            pressure_released += get_pressure_released_for_path(
                protagonist_path,
                valve_flow_rates,
                valve_activation_times,
                PART2_MINUTES,
            );
            pressure_released += get_pressure_released_for_path(
                ele_path,
                valve_flow_rates,
                valve_activation_times,
                PART2_MINUTES,
            );
            // Check if a new maximum pressure released value has been found
            if pressure_released > maximum_pressure_released {
                maximum_pressure_released = pressure_released;
            }
        }
    }
    maximum_pressure_released
}

/// Gets the time required to move from a valve with a non-zero flow rate (or the start valve "AA")
/// to another valve with flow.
fn get_valve_activation_times(
    valve_flow_rates: &HashMap<String, u64>,
    valve_connections: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, u64>> {
    let mut output: HashMap<String, HashMap<String, u64>> = HashMap::new();
    // Determine the valves to include in the activation time map
    let mut valid_valves: HashSet<String> = HashSet::new();
    valid_valves.insert(String::from("AA"));
    for (valve, flow_rate) in valve_flow_rates.iter() {
        if *flow_rate == 0 {
            continue;
        }
        valid_valves.insert(valve.to_string());
    }
    // Find the activation times for other valid valves for each valid valve
    for valve in valid_valves.iter() {
        let valve_activation_times =
            get_activation_times_from_start_valve(valve, valve_connections, &valid_valves);
        output.insert(valve.to_string(), valve_activation_times);
    }
    output
}

/// Gets the times required to move to and activate the valid valves from the given start valve.
fn get_activation_times_from_start_valve(
    start_valve: &str,
    valve_connections: &HashMap<String, Vec<String>>,
    valid_valves: &HashSet<String>,
) -> HashMap<String, u64> {
    let mut visit_queue: VecDeque<(u64, String)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut output: HashMap<String, u64> = HashMap::new();
    visit_queue.push_back((0, start_valve.to_string()));
    visited.insert(start_valve.to_string());
    while !visit_queue.is_empty() {
        // Get next valve to visit
        let (steps, valve) = visit_queue.pop_front().unwrap();
        if valid_valves.contains(valve.as_str()) {
            output.insert(valve.to_string(), steps + 1);
        }
        // Get next nodes to visit
        for next_valve in valve_connections.get(&valve).unwrap() {
            if !visited.contains(next_valve) {
                visited.insert(next_valve.to_string());
                visit_queue.push_back((steps + 1, next_valve.to_string()));
            }
        }
    }
    output
}

/// Determines the paths that are possible in the allowed time when starting from the given start
/// valve.
fn determine_possible_paths(
    start_valve: &str,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64,
) -> Vec<Vec<String>> {
    let mut possible_paths: Vec<Vec<String>> = vec![];
    determine_possible_paths_recursive(
        vec![String::from(start_valve)],
        minutes_allowed,
        &mut possible_paths,
        valve_activation_times,
    );
    possible_paths
}

/// Recursive helper function to find possible paths by building from the current path.
fn determine_possible_paths_recursive(
    current_path: Vec<String>,
    time_remaining: u64,
    possible_paths: &mut Vec<Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
) {
    let current_valve = current_path.last().unwrap();
    for next_valve in valve_activation_times.keys() {
        // Look up the activation time
        let activation_time = *valve_activation_times
            .get(current_valve)
            .unwrap()
            .get(next_valve)
            .unwrap();
        // Check if the next valve represents a valid move
        if current_path.contains(next_valve) || activation_time >= time_remaining {
            continue;
        }
        // Form the new path
        let mut new_path = current_path.clone();
        new_path.push(next_valve.to_string());
        // Keep finding new paths
        determine_possible_paths_recursive(
            new_path,
            time_remaining - activation_time,
            possible_paths,
            valve_activation_times,
        );
    }
    possible_paths.push(current_path);
}

/// Determines the possible paths the elephant could take in the time allowed for a given path taken
/// by the protagonist.
fn get_elephant_paths(
    start_valve: &str,
    protagonist_path: &Vec<String>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64,
) -> Vec<Vec<String>> {
    let mut elephant_paths: Vec<Vec<String>> = vec![];
    get_elephant_paths_recursive(
        protagonist_path,
        vec![String::from(start_valve)],
        minutes_allowed,
        &mut elephant_paths,
        valve_activation_times,
    );
    elephant_paths
}

/// Recursive helper function to find the possible elephant paths for a given protagonist path.
fn get_elephant_paths_recursive(
    protagonist_path: &Vec<String>,
    current_path: Vec<String>,
    time_remaining: u64,
    possible_paths: &mut Vec<Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
) {
    // Get reference to the current valve - the last valve of the current path
    let current_valve = current_path.last().unwrap();
    for next_valve in valve_activation_times.keys() {
        // Get the activation time for the next valve from the current valve
        let activation_time = *valve_activation_times
            .get(current_valve)
            .unwrap()
            .get(next_valve)
            .unwrap();
        // Check if the next valve is a valid move
        if current_path.contains(next_valve)
            || protagonist_path.contains(next_valve)
            || activation_time >= time_remaining
        {
            continue;
        }
        // Form the new path
        let mut new_path = current_path.clone();
        new_path.push(next_valve.to_string());
        // Keep building elephant paths
        get_elephant_paths_recursive(
            protagonist_path,
            new_path,
            time_remaining - activation_time,
            possible_paths,
            valve_activation_times,
        );
    }
    // No more possible moves from the current path so add the current path to the possible paths
    possible_paths.push(current_path);
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 16 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day16_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1767, solution);
    }

    /// Tests the Day 16 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day16_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2528, solution);
    }
}
