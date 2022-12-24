use std::collections::{HashSet, VecDeque};
use std::{fs, mem};

use itertools::Itertools;

use crate::Square::Empty;

type InputType = [[Square; H]; W];
type Point = (usize, usize);
type PointInTime = (usize, usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Square {
    Empty,
    R,
    D,
    L,
    U,
}

impl Square {
    fn from_char(c: char) -> Square {
        match c {
            '.' => Empty,
            '>' => Self::R,
            'v' => Self::D,
            '<' => Self::L,
            '^' => Self::U,
            _ => panic!(),
        }
    }
}

const W: usize = 120;
const H: usize = 25;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day24/input.txt").unwrap();

    let mut input: InputType = [[Empty; H]; W];
    for (y, line) in file.lines().skip(1).take(H).enumerate() {
        for (x, c) in line.trim().chars().skip(1).take(W).enumerate() {
            input[x][y] = Square::from_char(c);
        }
    }

    input
}

fn wrapping_sub(lhs: usize, rhs: usize, modulus: usize) -> usize {
    ((lhs as isize) - (rhs as isize)).rem_euclid(modulus as isize) as usize
}

fn get_moved_blizzard_points(&(x, y, time): &PointInTime) -> [Point; 4] {
    [
        ((x + time) % W, y),           // R
        (x, (y + time) % H),           // D
        (wrapping_sub(x, time, W), y), // L
        (x, wrapping_sub(y, time, H)), // U
    ]
}

const DIRECTIONS_TO_CHECK: [Square; 4] = [Square::L, Square::U, Square::R, Square::D];

fn check_square(input: &InputType, point_in_time: &PointInTime) -> bool {
    let &(x, y, time) = point_in_time;

    if x >= W || y >= H {
        return false;
    }

    if time == 0 {
        return input[x][y] == Empty;
    }

    let points_to_check = get_moved_blizzard_points(point_in_time);

    for i in 0..4 {
        let (xx, yy) = points_to_check[i];
        let direction = DIRECTIONS_TO_CHECK[i];

        if input[xx][yy] == direction {
            return false;
        }
    }

    true
}

fn get_valid_moves_in_time(input: &InputType, &(x, y, time): &PointInTime) -> Vec<PointInTime> {
    [
        (x, y),
        (x + 1, y),
        (x, y + 1),
        (x.saturating_sub(1), y),
        (x, y.saturating_sub(1)),
    ]
    .into_iter()
    .dedup()
    .filter_map(|(xx, yy)| {
        let new_point_in_time = (xx, yy, time + 1);

        if check_square(input, &new_point_in_time) {
            Some(new_point_in_time)
        } else {
            None
        }
    })
    .collect()
}

fn bfs(input: &InputType, (x, y): Point, time: usize, dest: Point) -> Option<usize> {
    let mut visited = HashSet::<PointInTime>::from([(x, y, time)]);
    let mut queue = VecDeque::<PointInTime>::from([(x, y, time)]);

    while let Some(point_in_time) = queue.pop_front() {
        let moves = get_valid_moves_in_time(input, &point_in_time);

        for new_point_in_time in moves {
            if !visited.contains(&new_point_in_time) {
                if (new_point_in_time.0, new_point_in_time.1) == dest {
                    return Some(new_point_in_time.2);
                }

                visited.insert(new_point_in_time);
                queue.push_back(new_point_in_time);
            }
        }
    }

    None
}

fn simulate(input: InputType) -> (usize, usize) {
    let mut part1 = None;
    let mut part2 = 1;
    let mut src = (0, 0);
    let mut dest = (W - 1, H - 1);

    for _ in 0..3 {
        for time in part2.. {
            if let Some(result) = bfs(&input, src, time, dest) {
                part2 = result + 1;
                break;
            }
        }

        if part1.is_none() {
            part1 = Some(part2);
        }

        mem::swap(&mut src, &mut dest);
    }

    (part1.unwrap(), part2)
}

pub fn main() {
    let input = read_input();

    let (part1, part2) = simulate(input);

    println!("--- Day 24 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 288);
    assert_eq!(part2, 861);
}
