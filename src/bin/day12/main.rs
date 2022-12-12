use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;

type Int = u32;
type Point = (isize, isize);
type InputType = BTreeMap<Point, char>;
type Graph = BTreeMap<Point, Vec<Point>>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day12/input.txt")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect()
}

const DELTAS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_input_as_graph(
    input: &InputType,
) -> (Point, Point, BTreeMap<Point, Vec<Point>>, BTreeSet<Point>) {
    let mut s: Point = Default::default();
    let mut e: Point = Default::default();

    let mut a_coords = BTreeSet::<Point>::new();
    let graph: BTreeMap<Point, Vec<Point>> = input
        .iter()
        .map(|(&point, &c)| -> (Point, Vec<Point>) {
            if c == 'S' {
                s = point;
                a_coords.insert(point);
            }

            if c == 'a' {
                a_coords.insert(point);
            }

            if c == 'E' {
                e = point;
                return (point, vec![]);
            }

            let neighbours = DELTAS
                .iter()
                .filter_map(|&(dx, dy)| {
                    let new_coord = (point.0 + dx, point.1 + dy);

                    let Some(&neighbour) = input.get(&new_coord) else {
                        return None;
                    };

                    let mut neighbour = neighbour;

                    if neighbour == 'S' {
                        return None;
                    }

                    if neighbour == 'E' {
                        neighbour = 'z';
                    }

                    if c == 'S' || (neighbour as u8) <= (c as u8) + 1 {
                        return Some(new_coord);
                    }

                    None
                })
                .collect();

            (point, neighbours)
        })
        .collect();

    (s, e, graph, a_coords)
}

fn bfs(
    graph: &Graph,
    &src: &Point,
    &dest: &Point,
    dist: &mut BTreeMap<Point, RefCell<Int>>,
    pred: &mut BTreeMap<Point, Point>,
) {
    let mut visited: BTreeSet<Point> = BTreeSet::<Point>::new();
    *dist = graph
        .keys()
        .map(|&p| (p, RefCell::new(Int::MAX)))
        .collect::<BTreeMap<Point, RefCell<Int>>>();
    let mut queue = VecDeque::<Point>::new();

    visited.insert(src);
    *dist.get(&src).unwrap().borrow_mut() = 0;
    queue.push_back(src);

    while let Some(u) = queue.pop_front() {
        for &i in graph.get(&u).unwrap() {
            if !visited.contains(&i) {
                visited.insert(i);
                *dist.get(&i).unwrap().borrow_mut() = *dist.get(&u).unwrap().borrow() + 1;
                pred.entry(i).or_insert(u);
                queue.push_back(i);

                if i == dest {
                    return;
                }
            }
        }
    }
}

fn part1(input: InputType) -> Int {
    let (s, e, graph, a_coords) = parse_input_as_graph(&input);
    let mut a_coords = a_coords;
    a_coords.insert(s);

    let mut part1 = 0;
    let mut part2 = Int::MAX;

    for p in a_coords {
        let mut dist = BTreeMap::<Point, RefCell<Int>>::new();
        let mut pred = BTreeMap::<Point, Point>::new();

        bfs(&graph, &p, &e, &mut dist, &mut pred);

        let ans = *dist.get(&e).unwrap().borrow();

        if p == s {
            part1 = ans;
        }

        if ans < part2 {
            part2 = ans;
        }
    }

    // TODO improve by finding last a coord in path
    dbg!(part1, part2);

    0
}

fn part2(_input: InputType) -> Int {
    0
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 12 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
