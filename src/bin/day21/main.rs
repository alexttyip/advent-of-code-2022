use std::collections::{HashMap, VecDeque};
use std::fs;

use itertools::Itertools;

use crate::Job::{Number, Operation};

type Int = u64;
type InputType = HashMap<String, Job>;

#[derive(Clone)]
enum Job {
    Number(Int),
    Operation(String, String, String),
}

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day21/input.txt")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (monkey, job) = line
                .split(": ")
                .map(|s| s.to_string())
                .collect_tuple::<(_, _)>()?;

            let monkey_job = if let Ok(number) = job.parse() {
                Number(number)
            } else {
                let (a, op, b) = job
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect_tuple::<(_, _, _)>()?;

                Operation(a, op, b)
            };

            Some((monkey, monkey_job))
        })
        .collect()
}

fn get_queue_of_operations(input: &InputType) -> VecDeque<String> {
    input
        .iter()
        .filter_map(|(key, value)| {
            if let Operation(..) = value {
                Some(key.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn do_maths(lhs: &Int, op: &str, rhs: &Int) -> Int {
    match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        _ => unreachable!(),
    }
}

fn part1(mut input: InputType) -> Int {
    let mut queue = get_queue_of_operations(&input);

    while let Some(monkey) = queue.pop_front() {
        let Some(Operation(lhs_name, op, rhs_name)) = input.get(&monkey) else {
            unreachable!();
        };

        let lhs = input.get(lhs_name).unwrap();
        let rhs = input.get(rhs_name).unwrap();

        let (lhs, rhs) = match (lhs, rhs) {
            (Number(lhs), Number(rhs)) => (lhs, rhs),
            _ => {
                queue.push_back(monkey);
                continue;
            }
        };

        let monkey_num = do_maths(lhs, op, rhs);

        if monkey == "root" {
            return monkey_num;
        }

        input.insert(monkey.to_string(), Number(monkey_num));
    }

    unreachable!();
}

fn part2(mut input: InputType) -> Int {
    let mut queue = get_queue_of_operations(&input);

    input.entry("humn".to_string()).and_modify(|v| {
        *v = Operation(String::new(), String::new(), String::new());
    });
    input.entry("root".to_string()).and_modify(|v| {
        if let Operation(_, op, _) = v {
            *op = "=".to_string();
        };
    });

    let mut target_monkey = String::from("root");
    let mut target = 0;

    while target_monkey != "humn" {
        let Some(monkey) = queue.pop_front()  else {
            unreachable!();
        };

        let Some(Operation(lhs_name, op, rhs_name)) = input.get(&monkey) else {
            unreachable!();
        };

        let lhs = input.get(lhs_name).unwrap();
        let rhs = input.get(rhs_name).unwrap();

        let (lhs, rhs) = match (lhs, rhs) {
            (Number(lhs), Number(rhs)) => (lhs, rhs),
            (Number(lhs), Operation(..)) => {
                queue.push_back(monkey.to_string());

                if monkey == target_monkey {
                    target_monkey = rhs_name.to_string();
                    target = match op.as_str() {
                        "+" => target - lhs,
                        "-" => lhs - target,
                        "*" => target / lhs,
                        "/" => lhs / target,
                        "=" => *lhs,
                        _ => unreachable!(),
                    };
                }

                continue;
            }
            (Operation(..), Number(rhs)) => {
                queue.push_back(monkey.to_string());

                if monkey == target_monkey {
                    target_monkey = lhs_name.to_string();
                    target = match op.as_str() {
                        "+" => target - rhs,
                        "-" => target + rhs,
                        "*" => target / rhs,
                        "/" => target * rhs,
                        "=" => *rhs,
                        _ => unreachable!(),
                    };
                }

                continue;
            }
            _ => {
                queue.push_back(monkey);
                continue;
            }
        };

        let monkey_num = do_maths(lhs, op, rhs);
        input.insert(monkey.to_string(), Number(monkey_num));
    }

    target
}

pub fn main() {
    let input = read_input();

    println!("--- Day 21 ---");

    let part1 = part1(input.clone());
    println!("Part 1: {}", part1);

    let part2 = part2(input);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 194058098264286);
    assert_eq!(part2, 3592056845086);
}
