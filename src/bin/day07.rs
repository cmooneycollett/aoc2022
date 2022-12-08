use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "No Space Left On Device";
const PROBLEM_INPUT_FILE: &str = "./input/day07.txt";
const PROBLEM_DAY: u64 = 7;

/// Represents a file or directory in a file system.
enum FsItem {
    File { size: usize },
    Directory { parent_dir: String, name: String },
}

/// Processes the AOC 2022 Day 7 input file and solves both parts of the problem. Solutions are
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
    println!("[*] TOTAL:  {:.2?}", input_parser_duration + p1_duration + p2_duration);
    println!("==================================================");
}

/// Processes the AOC 2022 Day 7 input file in the format required by the solver functions.
/// Returned value is hashmap containing each directory (full path name) mapped to the vector of
/// fsitems contained in the directory.
fn process_input_file(filename: &str) -> HashMap<String, Vec<FsItem>> {
    // Read contents of problem input file
    let binding = fs::read_to_string(filename).unwrap();
    let raw_input = binding.trim();
    // Process input file contents into data structure
    let mut output: HashMap<String, Vec<FsItem>> = HashMap::new();
    let mut current_dir: VecDeque<String> = VecDeque::new();
    let cd_regex = Regex::new(r"^[$] cd (.*)$").unwrap();
    let file_regex = Regex::new(r"^(\d+) (.*)$").unwrap();
    let dir_regex = Regex::new(r"^dir (.*)$").unwrap();
    let lines = raw_input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();
    let mut cursor = 0;
    loop {
        if cursor >= lines.len() {
            break;
        }
        if cd_regex.is_match(&lines[cursor]) {
            let caps = cd_regex.captures(&lines[cursor]).unwrap();
            let dir = caps[1].to_string();
            if dir == ".." {
                current_dir.pop_back();
                cursor += 1;
            } else if dir == "/" {
                current_dir.push_back(dir);
                let cwd = current_dir
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join("");
                output.insert(cwd, vec![]);
                cursor += 2;
            } else {
                current_dir.push_back(format!("{}/", dir));
                let cwd = current_dir
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join("");
                output.insert(cwd, vec![]);
                cursor += 2;
            }
        } else {
            let cwd = current_dir
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join("");
            loop {
                if cursor >= lines.len() || cd_regex.is_match(&lines[cursor]) {
                    break;
                } else if file_regex.is_match(&lines[cursor]) {
                    let caps = file_regex.captures(&lines[cursor]).unwrap();
                    let size = caps[1].parse::<usize>().unwrap();
                    let file_item = FsItem::File { size };
                    output.get_mut(&cwd).unwrap().push(file_item);
                } else if dir_regex.is_match(&lines[cursor]) {
                    let caps = dir_regex.captures(&lines[cursor]).unwrap();
                    let name = caps[1].to_string();
                    let dir_item = FsItem::Directory {
                        parent_dir: cwd.to_string(),
                        name,
                    };
                    output.get_mut(&cwd).unwrap().push(dir_item);
                } else {
                    panic!("Day 7 - bad file system item!");
                }
                cursor += 1;
            }
        }
    }
    output
}

/// Solves AOC 2022 Day 7 Part 1 // Calculates the total size of all directories that have a size of
/// at most 100,000.
fn solve_part1(dirs: &HashMap<String, Vec<FsItem>>) -> usize {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    find_dir_sizes(dirs, &mut dir_sizes, &String::from("/"));
    return dir_sizes
        .values()
        .copied()
        .filter(|size| *size <= 100000)
        .sum();
}

/// Solves AOC 2022 Day 7 Part 2 // Finds the size of the smallest directory that would free up
/// enough space if deleted.
fn solve_part2(dirs: &HashMap<String, Vec<FsItem>>) -> usize {
    let max_fs_size: usize = 70000000;
    let req_free_space: usize = 30000000;
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    find_dir_sizes(dirs, &mut dir_sizes, &String::from("/"));
    let free_space = max_fs_size - dir_sizes.get("/").unwrap();
    // Calculate extra amount of free space
    let delta = req_free_space - free_space;
    let mut candidate_dirs = dir_sizes
        .values()
        .copied()
        .filter(|size| *size >= delta)
        .collect::<Vec<usize>>();
    candidate_dirs.sort();
    candidate_dirs[0]
}

/// Finds the sizes of all directories below the given dir. Size of directories are updated into the
/// dir_sizes hashmap.
fn find_dir_sizes(
    dirs: &HashMap<String, Vec<FsItem>>,
    dir_sizes: &mut HashMap<String, usize>,
    dir: &String,
) {
    let mut total_size = 0;
    for fs_item in dirs.get(dir).unwrap().iter() {
        match fs_item {
            FsItem::Directory { parent_dir, name } => {
                let cwd = format!("{}{}/", parent_dir, name);
                find_dir_sizes(dirs, dir_sizes, &cwd);
                total_size += dir_sizes.get(&cwd).unwrap();
            }
            FsItem::File { size } => {
                total_size += size;
            }
        }
    }
    dir_sizes.insert(dir.to_string(), total_size);
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 7 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day07_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1432936, solution);
    }

    /// Tests the Day 7 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day07_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(272298, solution);
    }
}
