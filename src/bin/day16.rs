use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;
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
) -> HashMap<String, u64> {
    let mut visit_queue: VecDeque<(u64, String)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut output: HashMap<String, u64> = HashMap::new();
    visit_queue.push_back((0, start_valve.to_string()));
    visited.insert(start_valve.to_string());
    while !visit_queue.is_empty() {
        // Get next valve to visit
        let (steps, valve) = visit_queue.pop_front().unwrap();
        if *valve_flow_rates.get(&valve).unwrap() > 0 {
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
) -> Vec<Vec<String>> {
    let mut possible_paths: Vec<Vec<String>> = vec![];
    determine_possible_paths_recursive(
        "AA",
        vec![],
        30,
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
    for next_valve in valve_connections.get(current_valve).unwrap() {
        if current_path.contains(next_valve)
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

/// Solves AOC 2022 Day 16 Part 1 // ###
fn solve_part1(input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    let (valve_flow_rates, valve_connections) = input;
    // let mut open_valves: HashSet<String> = HashSet::new();
    let valves_with_flow = valve_flow_rates
        .iter()
        .filter(|(_, v)| **v > 0)
        .map(|entry| entry.0.to_string())
        .collect::<Vec<String>>();
    let mut valve_activation_times: HashMap<String, HashMap<String, u64>> = HashMap::new();
    for valve in valve_flow_rates.keys() {
        let activation_times =
            get_visitable_flow_valves(valve, valve_flow_rates, valve_connections);
        valve_activation_times.insert(valve.to_string(), activation_times);
    }
    let mut max_pressure_released = 0;
    let mut old_valves_opened: Vec<String> = vec![];
    let mut perms_checked: u64 = 0;
    for perm in valves_with_flow.iter().permutations(valves_with_flow.len()) {
        perms_checked += 1;
        if perms_checked % 1000000 == 0 {
            println!("[*] perms checked: {}", perms_checked);
        }
        let mut skip_perm = true;
        for i in 0..old_valves_opened.len() {
            if *perm[i] != old_valves_opened[i] {
                skip_perm = false;
                break;
            }
        }
        if skip_perm && old_valves_opened.len() > 0 {
            continue;
        }
        println!("[+] trying perm: {:?}", perm);
        let (pressure_released, valves_opened) =
            try_permutation_for_pressure(&perm, &valve_activation_times, valve_flow_rates);
        old_valves_opened = valves_opened.to_vec();
        // println!(">>>> valves opened: {:?}", valves_opened);
        if pressure_released > max_pressure_released {
            max_pressure_released = pressure_released;
            println!(">>>> pressure released: {}", pressure_released);
            println!(">>>> * NEW MAXIMUM *");
        }
    }
    max_pressure_released
}

fn try_permutation_for_pressure(
    perm: &Vec<&String>,
    valve_activation_times: &HashMap<String, HashMap<String, u64>>,
    valve_flow_rates: &HashMap<String, u64>,
) -> (u64, Vec<String>) {
    let mut time_remaining = 30;
    let mut pressure_released = 0;
    let mut pressure_per_minute = 0;
    let mut valves_opened: Vec<String> = vec![];
    for i in 0..perm.len() {
        let valve_from = {
            if i == 0 {
                "AA"
            } else {
                perm[i - 1]
            }
        };
        let valve_to = perm[i];
        let activation_time = *valve_activation_times
            .get(valve_from)
            .unwrap()
            .get(valve_to)
            .unwrap();
        if activation_time >= time_remaining {
            pressure_released += pressure_per_minute * time_remaining;
            break;
        }
        time_remaining -= activation_time;
        pressure_released += pressure_per_minute * activation_time;
        pressure_per_minute += valve_flow_rates.get(valve_to).unwrap();
        valves_opened.push(valve_to.to_string());
    }
    (pressure_released, valves_opened)
}

/// Solves AOC 2022 Day 16 Part 2 // ###
fn solve_part2(_input: &(HashMap<String, u64>, HashMap<String, Vec<String>>)) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 16 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day16_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
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
