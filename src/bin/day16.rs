use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Proboscidea Volcanium";
// const PROBLEM_INPUT_FILE: &str = "./input/day16.txt";
const PROBLEM_INPUT_FILE: &str = "./input/test/day16_t001.txt";
const PROBLEM_DAY: u64 = 16;

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

fn get_visitable_flow_valves(
    start_valve: &str,
    valve_flow_rates: &HashMap<String, u64>,
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

fn determine_possible_paths(
    valve_connections: &HashMap<String, Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64,
) -> Vec<Vec<String>> {
    let mut possible_paths: Vec<Vec<String>> = vec![];
    determine_possible_paths_recursive(
        "AA",
        vec![],
        minutes_allowed,
        &mut possible_paths,
        valve_connections,
        valve_activation_times,
    );
    possible_paths
}

fn determine_possible_paths_recursive(
    current_valve: &str,
    current_path: Vec<String>,
    time_remaining: u64,
    possible_paths: &mut Vec<Vec<String>>,
    valve_connections: &HashMap<String, Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
) {
    for next_valve in valve_activation_times.keys() {
        // println!("current valve: {} // next valve: {}", current_valve, next_valve);
        if next_valve == "AA"
            || current_path.contains(next_valve)
            || !valve_activation_times.contains_key(next_valve)
            || *valve_activation_times
                .get(current_valve)
                .unwrap()
                .get(next_valve)
                .unwrap()
                >= time_remaining
        {
            continue;
        }
        let mut new_path = current_path.to_vec();
        new_path.push(next_valve.to_string());
        let new_time_remaining = time_remaining
            - valve_activation_times
                .get(current_valve)
                .unwrap()
                .get(next_valve)
                .unwrap();
        determine_possible_paths_recursive(
            next_valve,
            new_path,
            new_time_remaining,
            possible_paths,
            valve_connections,
            valve_activation_times,
        );
    }
    possible_paths.push(current_path);
}

/// Gets the time required to move from a valve with flow (or the start valve "AA") to another valve
/// with flow.
fn get_valve_activation_times(
    valve_flow_rates: &HashMap<String, u64>,
    valve_connections: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, u64>> {
    let mut output: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let mut valid_valves: HashSet<String> = HashSet::new();
    valid_valves.insert(String::from("AA"));
    for (valve, flow_rate) in valve_flow_rates.iter() {
        if *flow_rate == 0 {
            continue;
        }
        valid_valves.insert(valve.to_string());
    }
    for valve in valid_valves.iter() {
        let valve_activation_times =
            get_visitable_flow_valves(&valve, valve_flow_rates, valve_connections, &valid_valves);
        output.insert(valve.to_string(), valve_activation_times);
    }
    output
}

fn get_pressure_released_for_path(
    path: &Vec<String>,
    valve_flow_rates: &HashMap<String, u64>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64,
) -> u64 {
    let mut minutes_remaining = minutes_allowed;
    let mut pressure_per_minute = 0;
    let mut total_pressure_released = 0;
    for i in 0..path.len() {
        // Get activation time
        let activation_time = {
            if i == 0 {
                valve_activation_times
                    .get("AA")
                    .unwrap()
                    .get(&path[i])
                    .unwrap()
            } else {
                valve_activation_times
                    .get(&path[i - 1])
                    .unwrap()
                    .get(&path[i])
                    .unwrap()
            }
        };
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

/// Solves AOC 2022 Day 16 Part 1 // ###
fn solve_part1(input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    let (valve_flow_rates, valve_connections) = input;
    let valve_activation_times = &get_valve_activation_times(valve_flow_rates, valve_connections);
    let possible_paths = determine_possible_paths(valve_connections, valve_activation_times, 30);
    let mut max_pressure_released = 0;
    for path in possible_paths.iter() {
        let pressure_released =
            get_pressure_released_for_path(path, valve_flow_rates, valve_activation_times, 30);
        if pressure_released > max_pressure_released {
            max_pressure_released = pressure_released;
        }
    }
    max_pressure_released
}

/// Solves AOC 2022 Day 16 Part 2 // ###
fn solve_part2(input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    let (valve_flow_rates, valve_connections) = input;
    let valve_activation_times = &get_valve_activation_times(valve_flow_rates, valve_connections);
    // Find my paths
    let possible_paths = determine_possible_paths(valve_connections, valve_activation_times, 26);
    // Find the elephant paths
    let possible_paths_simul = determine_possible_paths_simul(&possible_paths, valve_connections, valve_activation_times, 26);
    // Find the maximum pressure released
    let mut maximum_pressure_released = 0;
    for (protag_path, ele_path) in possible_paths_simul.iter() {
        let mut pressure_released = 0;
        pressure_released += get_pressure_released_for_path(protag_path, valve_flow_rates, valve_activation_times, 26);
        pressure_released += get_pressure_released_for_path(ele_path, valve_flow_rates, valve_activation_times, 26);
        if pressure_released > maximum_pressure_released {
            maximum_pressure_released = pressure_released;
        }
    }
    maximum_pressure_released
}

fn determine_possible_paths_simul(
    possible_paths: &Vec<Vec<String>>,
    valve_connections: &HashMap<String, Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    minutes_allowed: u64
) -> Vec<(Vec<String>, Vec<String>)> {
    let mut output: Vec<(Vec<String>, Vec<String>)> = vec![];
    for path in possible_paths {
        println!("Getting elephant paths for: {:?}", path);
        let mut possible_paths_simul: Vec<(Vec<String>, Vec<String>)> = vec![];
        determine_possible_paths_simul_recursive(path, "AA", vec![], minutes_allowed, &mut possible_paths_simul, valve_connections, valve_activation_times);
        output.extend(possible_paths_simul);
    }
    output
}

fn determine_possible_paths_simul_recursive(
    protagonist_path: &Vec<String>,
    current_valve: &str,
    current_path: Vec<String>,
    time_remaining: u64,
    possible_paths: &mut Vec<(Vec<String>, Vec<String>)>,
    valve_connections: &HashMap<String, Vec<String>>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
) {
    for next_valve in valve_activation_times.keys() {
        if next_valve == "AA"
            || current_path.contains(next_valve)
            || protagonist_path.contains(next_valve)
            || !valve_activation_times.contains_key(next_valve)
            || *valve_activation_times
                .get(current_valve)
                .unwrap()
                .get(next_valve)
                .unwrap()
                >= time_remaining
        {
            continue;
        }
        let mut new_path = current_path.to_vec();
        new_path.push(next_valve.to_string());
        let new_time_remaining = time_remaining
            - valve_activation_times
                .get(current_valve)
                .unwrap()
                .get(next_valve)
                .unwrap();
        determine_possible_paths_simul_recursive(
            protagonist_path,
            next_valve,
            new_path,
            new_time_remaining,
            possible_paths,
            valve_connections,
            valve_activation_times,
        );
    }
    possible_paths.push((protagonist_path.to_vec(), current_path));
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
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
