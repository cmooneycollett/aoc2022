use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Monkey in the Middle";
const PROBLEM_INPUT_FILE: &str = "./input/day11.txt";
const PROBLEM_DAY: u64 = 11;

/// Represents a single monkey.
#[derive(Clone)]
struct Monkey {
    items: VecDeque<u128>,
    op: Operation,
    test_mod: u128,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected: u128,
}

impl Monkey {
    /// Creates a new monkey, with items inspected set to 0.
    pub fn new(
        items: VecDeque<u128>,
        op: Operation,
        test_mod: u128,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            op,
            test_mod,
            true_monkey,
            false_monkey,
            items_inspected: 0,
        }
    }

    /// Monkey inspects and throws each of its items in order.
    pub fn inspect_and_throw(&mut self, reduce_worry: bool, supermodulo: u128) -> Vec<(usize, u128)> {
        let mut thrown_items: Vec<(usize, u128)> = vec![];
        loop {
            if self.items.is_empty() {
                break;
            }
            // Inspect item
            self.items_inspected += 1;
            match self.op {
                Operation::Add { value } => self.items[0] += value,
                Operation::Mult { value } => self.items[0] *= value,
                Operation::Pow { value } => self.items[0] = self.items[0].pow(value),
            }
            // Reduce the worry
            if reduce_worry {
                self.items[0] /= 3;
            }
            // Apply the supermodulo to reduce the item worry to stop it becoming too large
            self.items[0] %= supermodulo;
            if self.items[0] == 0 {
                self.items[0] = supermodulo;
            }
            // Check for throw
            let new_monkey = {
                if self.items[0] % self.test_mod == 0 {
                    self.true_monkey
                } else {
                    self.false_monkey
                }
            };
            thrown_items.push((new_monkey, self.items.pop_front().unwrap()));
        }
        thrown_items
    }
}

/// Represents an operator performed on the worry level of items by monkey.
#[derive(Clone, Copy)]
enum Operation {
    Add { value: u128 },
    Mult { value: u128 },
    Pow { value: u32 },
}

/// Processes the AOC 2022 Day 11 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 11 input file in the format required by the solver functions.
/// Returned value is vector of monkeys specified in the input file.
fn process_input_file(filename: &str) -> Vec<Monkey> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut output: Vec<Monkey> = vec![];
    let regex_monkey = Regex::new(concat!(
        r#"Monkey (\d+):%  Starting items: (.*)%  Operation: new = old (.*)%"#,
        r#"  Test: divisible by (\d+)%    If true: throw to monkey (\d+)%"#,
        r#"    If false: throw to monkey (\d+)"#
    ))
    .unwrap();
    for split in raw_input
        .trim()
        .split("\n\n")
        .map(|group| group.replace('\n', "%"))
    {
        let caps = regex_monkey.captures(&split).unwrap();
        let items: VecDeque<u128> = caps[2]
            .split(", ")
            .map(|value| value.parse::<u128>().unwrap())
            .collect::<VecDeque<u128>>();
        let op = {
            if &caps[3] == "* old" {
                Operation::Pow { value: 2 }
            } else if caps[3].starts_with("+") {
                let value = caps[3].split("+ ").nth(1).unwrap().parse::<u128>().unwrap();
                Operation::Add { value }
            } else if caps[3].starts_with("*") {
                let value = caps[3].split("* ").nth(1).unwrap().parse::<u128>().unwrap();
                Operation::Mult { value }
            } else {
                panic!("Day 11 - bad operation");
            }
        };
        let test_mod = caps[4].parse::<u128>().unwrap();
        let true_monkey = caps[5].parse::<usize>().unwrap();
        let false_monkey = caps[6].parse::<usize>().unwrap();
        output.push(Monkey::new(items, op, test_mod, true_monkey, false_monkey));
    }
    output
}

/// Solves AOC 2022 Day 11 Part 1 // Calculates the resulting monkey business level after 20 rounds
/// with worry reduction in place.
fn solve_part1(initial_monkeys: &Vec<Monkey>) -> u128 {
    get_monkey_business(initial_monkeys, 20, true)
}

/// Solves AOC 2022 Day 11 Part 2 // Calculates the resulting monkey business level after 10,000
/// rounds without worry reduction in place.
fn solve_part2(initial_monkeys: &Vec<Monkey>) -> u128 {
    get_monkey_business(initial_monkeys, 10000, false)
}

/// Conducts a given number of rounds of monkey business.
fn get_monkey_business(initial_monkeys: &Vec<Monkey>, rounds: u128, reduce_worry: bool) -> u128 {
    let mut monkeys = initial_monkeys.clone();
    let supermodulo: u128 = monkeys.iter().map(|m| m.test_mod).product();
    for _ in 0..rounds {
        // Conduct rounds
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].inspect_and_throw(reduce_worry, supermodulo);
            for (new_monkey, item) in thrown_items {
                monkeys[new_monkey].items.push_back(item);
            }
        }
    }
    let mut output = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u128>>();
    output.sort();
    return output.iter().rev().take(2).product();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 11 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day11_p1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(99840, solution);
    }

    /// Tests the Day 11 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day11_p2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(20683044837, solution);
    }
}
