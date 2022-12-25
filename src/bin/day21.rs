use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

const PROBLEM_NAME: &str = "Monkey Math";
const PROBLEM_INPUT_FILE: &str = "./input/day21.txt";
const PROBLEM_DAY: u64 = 21;

lazy_static! {
    static ref REGEX_TOKEN: Regex = Regex::new(r"(\(|\)|\d+|\+|\-|\*|/|[a-z+])").unwrap();
}

#[derive(Clone, PartialEq, Eq)]
enum Operation {
    Nop { value: i64 },
    Variable { var: String },
    Add { left: String, right: String },
    Subtract { left: String, right: String },
    Multiply { left: String, right: String },
    Divide { left: String, right: String },
    Equal { left: String, right: String },
}

/// Processes the AOC 2022 Day 21 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 21 input file in the format required by the solver functions.
/// Returned value is hashmap of monkey names mapped to their operation.
fn process_input_file(filename: &str) -> HashMap<String, Operation> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_nop = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
    let regex_add = Regex::new(r"^([a-z]+): ([a-z]+) \+ ([a-z]+)$").unwrap();
    let regex_subtract = Regex::new(r"^([a-z]+): ([a-z]+) \- ([a-z]+)$").unwrap();
    let regex_multiply = Regex::new(r"^([a-z]+): ([a-z]+) \* ([a-z]+)$").unwrap();
    let regex_divide = Regex::new(r"^([a-z]+): ([a-z]+) / ([a-z]+)$").unwrap();
    let mut output: HashMap<String, Operation> = HashMap::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(caps) = regex_nop.captures(line) {
            let name = caps[1].to_string();
            let value = caps[2].parse::<i64>().unwrap();
            output.insert(name, Operation::Nop { value });
        } else if let Some(caps) = regex_add.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Add { left, right });
        } else if let Some(caps) = regex_subtract.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Subtract { left, right });
        } else if let Some(caps) = regex_multiply.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Multiply { left, right });
        } else if let Some(caps) = regex_divide.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Divide { left, right });
        } else {
            panic!("Day 21 - bad input line!");
        }
    }
    output
}

/// Solves AOC 2022 Day 21 Part 1 // Determines the number that the monkey named "root" will yell
/// out.
fn solve_part1(monkey_ops: &HashMap<String, Operation>) -> i64 {
    determine_monkey_yell_value("root", monkey_ops).unwrap()
}

/// Solves AOC 2022 Day 21 Part 2 // ###
fn solve_part2(monkey_ops: &HashMap<String, Operation>) -> i64 {
    let mut humn_i = 0;
    let mut monkey_ops_mod = monkey_ops.clone();
    let old_root_op = monkey_ops.get("root").unwrap();
    let new_root_op = match old_root_op {
        Operation::Add { left, right } => Operation::Equal {
            left: left.to_string(),
            right: right.to_string(),
        },
        Operation::Subtract { left, right } => Operation::Equal {
            left: left.to_string(),
            right: right.to_string(),
        },
        Operation::Multiply { left, right } => Operation::Equal {
            left: left.to_string(),
            right: right.to_string(),
        },
        Operation::Divide { left, right } => Operation::Equal {
            left: left.to_string(),
            right: right.to_string(),
        },
        _ => panic!("Bad \"root\" old op!"),
    };
    monkey_ops_mod.insert(String::from("root"), new_root_op);
    monkey_ops_mod.insert(String::from("humn"), Operation::Variable { var: String::from("humn") });
    // print expression
    let root_expr = generate_monkey_expression("root", &monkey_ops_mod);
    let sides = root_expr.split(" = ").map(|side| side.to_string()).collect::<Vec<String>>();
    let rpn = convert_to_rpn(&sides[1]);
    let result = evaluate_rpn(&rpn);

    // let test = "4 + 18 / (9 - 3)";
    // let rpn = convert_to_rpn(test);
    // let result = evaluate_rpn(&rpn);




    // println!("{}", root_expr);
    println!("[{}] {:?}", result, rpn);
    // loop {
    //     if humn_i % 10000 == 0 {
    //         println!("trying to yell {}...", humn_i);
    //     }
    //     // let mut new_monkey_ops = monkey_ops_mod.clone();
    //     monkey_ops_mod.insert(String::from("humn"), Operation::Nop { value: humn_i });
    //     if let Some(_) = determine_monkey_yell_value("root", &monkey_ops_mod) {
    //         return humn_i;
    //     }
    //     humn_i += 1;
    //     // let new_root_ops = Operation::Equal { left: old_root_op., right: old_root_op.right };
    // }
    0
}

fn evaluate_rpn(rpn: &Vec<String>) -> u64 {
    let mut stack: VecDeque<String> = VecDeque::new();
    for token in rpn {
        if let Ok(_) = token.parse::<u64>() {
            stack.push_back(token.to_string());
        } else {
            let right = stack.pop_back().unwrap().parse::<u64>().unwrap();
            let left = stack.pop_back().unwrap().parse::<u64>().unwrap();
            let result = match token.as_str() {
                "+" => {
                    left + right
                }
                "-" => {
                    left - right
                }
                "*" => {
                    left * right
                }
                "/" => {
                    left / right
                }
                _ => panic!("Bad token in RPN evaluation: {}", token),
            };
            stack.push_back(result.to_string());
        }
    }
    stack.pop_back().unwrap().parse::<u64>().unwrap()
}

/// Converts the given expression to Reverse Polish Notation (RPN).
fn convert_to_rpn(expr: &str) -> Vec<String> {
    let expr = expr.replace(' ', "");
    let mut op_stack: VecDeque<&str> = VecDeque::new();
    let mut output: Vec<&str> = vec![];
    for token in REGEX_TOKEN.find_iter(&expr) {
        let token = token.as_str();
        if let Ok(_) = token.parse::<u64>() {
            output.push(token);
        } else if token == "(" {
            op_stack.push_back(token);
        } else if token == ")" {
            while *op_stack.back().unwrap() != "(" {
                output.push(op_stack.pop_back().unwrap());
            }
            // Discard left parenthesis at top of operator stack
            op_stack.pop_back().unwrap();
        } else {
            while !op_stack.is_empty() && *op_stack.back().unwrap() != "(" && get_precedence(op_stack.back().unwrap()) > get_precedence(token) {
                output.push(op_stack.pop_back().unwrap());
            }
            op_stack.push_back(token);
        }
    }
    while !op_stack.is_empty() {
        output.push(op_stack.pop_back().unwrap());
    }
    output.iter().map(|token| token.to_string()).collect::<Vec<String>>()
}

/// Gets the precedence of the given operator token.
fn get_precedence(token: &str) -> u64 {
    match token {
        "*" => 3,
        "/" => 3,
        "+" => 2,
        "-" => 2,
        _ => panic!("Bad token for precedence check: {}", token),
    }
}

/// Generates the mathematical expression that will provide the value to be yelled by the monkey.
fn generate_monkey_expression(name: &str, monkey_ops: &HashMap<String, Operation>) -> String {
    String::from(match monkey_ops.get(name).unwrap() {
        Operation::Nop { value } => format!("{}", value),
        Operation::Variable { var } => format!("{}", var),
        Operation::Add { left, right } => format!(
            "({} + {})",
            generate_monkey_expression(left, monkey_ops),
            generate_monkey_expression(right, monkey_ops)
        ),
        Operation::Subtract { left, right } => format!(
            "({} - {})",
            generate_monkey_expression(left, monkey_ops),
            generate_monkey_expression(right, monkey_ops)
        ),
        Operation::Multiply { left, right } => format!(
            "({} * {})",
            generate_monkey_expression(left, monkey_ops),
            generate_monkey_expression(right, monkey_ops)
        ),
        Operation::Divide { left, right } => format!(
            "({} / {})",
            generate_monkey_expression(left, monkey_ops),
            generate_monkey_expression(right, monkey_ops)
        ),
        Operation::Equal { left, right } => format!(
            "{} = {}",
            generate_monkey_expression(left, monkey_ops),
            generate_monkey_expression(right, monkey_ops)
        ),
    })
}

/// Determines the value that will be yelled by the named monkey.
fn determine_monkey_yell_value(name: &str, monkey_ops: &HashMap<String, Operation>) -> Option<i64> {
    match monkey_ops.get(name).unwrap() {
        Operation::Nop { value } => Some(*value),
        Operation::Add { left, right } => Some(
            determine_monkey_yell_value(left, monkey_ops).unwrap()
                + determine_monkey_yell_value(right, monkey_ops).unwrap(),
        ),
        Operation::Subtract { left, right } => Some(
            determine_monkey_yell_value(left, monkey_ops).unwrap()
                - determine_monkey_yell_value(right, monkey_ops).unwrap(),
        ),
        Operation::Multiply { left, right } => Some(
            determine_monkey_yell_value(left, monkey_ops).unwrap()
                * determine_monkey_yell_value(right, monkey_ops).unwrap(),
        ),
        Operation::Divide { left, right } => Some(
            determine_monkey_yell_value(left, monkey_ops).unwrap()
                / determine_monkey_yell_value(right, monkey_ops).unwrap(),
        ),
        Operation::Equal { left, right } => {
            if determine_monkey_yell_value(left, monkey_ops).unwrap()
                == determine_monkey_yell_value(right, monkey_ops).unwrap()
            {
                Some(determine_monkey_yell_value(left, monkey_ops).unwrap())
            } else {
                None
            }
        }
        Operation::Variable { var } => panic!(
            "Cannot determine monkey yell value with unknown variable: {}",
            var
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 21 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day21_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(268597611536314, solution);
    }

    /// Tests the Day 21 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day21_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 21 Part 2 solver method against example input 001.
    #[test]
    fn test_day21_part1_t001() {
        let input = process_input_file("./input/test/day21_t001.txt");
        let solution = solve_part1(&input);
        assert_eq!(152, solution);
    }

    /// Tests the Day 21 Part 2 solver method against example input 001.
    #[test]
    fn test_day21_part2_t001() {
        let input = process_input_file("./input/test/day21_t001.txt");
        let solution = solve_part2(&input);
        assert_eq!(301, solution);
    }
}
