use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;

const SIZE: usize = 8;
const DIVISORS: [Int; SIZE] = [11, 7, 3, 5, 17, 13, 19, 2];
const DIVISORS_PRODUCT: Int = 9_699_690;
const TARGETS: [(usize, usize); SIZE] = [
    (2, 7),
    (0, 2),
    (7, 5),
    (6, 4),
    (1, 0),
    (6, 3),
    (4, 1),
    (5, 3),
];

type Int = u64;
type InputType = HashMap<usize, RefCell<VecDeque<Int>>>;

fn read_input() -> InputType {
    HashMap::from([
        (0, RefCell::new(VecDeque::from([50, 70, 54, 83, 52, 78]))),
        (1, RefCell::new(VecDeque::from([71, 52, 58, 60, 71]))),
        (
            2,
            RefCell::new(VecDeque::from([66, 56, 56, 94, 60, 86, 73])),
        ),
        (3, RefCell::new(VecDeque::from([83, 99]))),
        (4, RefCell::new(VecDeque::from([98, 98, 79]))),
        (5, RefCell::new(VecDeque::from([76]))),
        (6, RefCell::new(VecDeque::from([52, 51, 84, 54]))),
        (
            7,
            RefCell::new(VecDeque::from([82, 86, 91, 79, 94, 92, 59, 94])),
        ),
    ])
}

fn monkey_op(idx: usize, item: Int, part1: bool) -> Int {
    let item = match idx {
        0 => item * 3,
        1 => item * item,
        2 => item + 1,
        3 => item + 8,
        4 => item + 3,
        5 => item + 4,
        6 => item * 17,
        7 => item + 7,
        _ => panic!(),
    };

    if part1 {
        item / 3
    } else {
        item % DIVISORS_PRODUCT
    }
}

fn test_and_get_monkey_target(idx: usize, item: Int) -> usize {
    if item % DIVISORS[idx] == 0 {
        TARGETS[idx].0
    } else {
        TARGETS[idx].1
    }
}

fn play(monkeys: InputType, part1: bool) -> Int {
    let mut inspect_count: [Int; SIZE] = Default::default();

    for _ in 0..(if part1 { 20 } else { 10_000 }) {
        (0..SIZE).for_each(|idx| {
            let mut items = monkeys.get(&idx).unwrap().borrow_mut();
            inspect_count[idx] += items.len() as Int;

            while let Some(item) = items.pop_front() {
                let item = monkey_op(idx, item, part1);
                let target = test_and_get_monkey_target(idx, item);

                monkeys.get(&target).unwrap().borrow_mut().push_back(item);
            }
        });
    }

    inspect_count
        .iter()
        .sorted_unstable()
        .rev()
        .take(2)
        .product()
}

fn part1(monkey_items: InputType) -> Int {
    play(monkey_items, true)
}

fn part2(monkey_items: InputType) -> Int {
    play(monkey_items, false)
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 11 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 102399);
    assert_eq!(part2, 23641658401);
}
