use std::collections::HashSet;
use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut visited : HashSet<(i32, i32)> = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);

        visited.insert(tail);

        for l in input.as_lines() {
            let command = l.split_once(" ").unwrap();
            let dir = command.0;
            let distance = command.1.parse::<i32>().unwrap();

            for i in 0..distance {
                match dir {
                    "R" => head.0 += 1,
                    "L" => head.0 -= 1,
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    _ => panic!("Bad command")
                }
                tail = update_link(head, tail);
                visited.insert(tail);
            }
        }

        visited.len().to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut rope: Vec<(i32, i32)> = Vec::new();

        for i in 0..10 {
            rope.push((0, 0));
        }
        visited.insert(rope[9]);

        for l in input.as_lines() {
            let command = l.split_once(" ").unwrap();
            let dir = command.0;
            let distance = command.1.parse::<i32>().unwrap();

            for i in 0..distance {
                match dir {
                    "R" => rope[0].0 += 1,
                    "L" => rope[0].0 -= 1,
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    _ => panic!("Bad command")
                }
                for i in 1..10 {
                    rope[i] = update_link(rope[i - 1], rope[i]);
                }
                visited.insert(rope[9]);
            }
        }
        visited.len().to_string()
    }
}

fn update_link(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let delta = (head.0 - tail.0, head.1 - tail.1);
    let adelta = (delta.0.abs(), delta.1.abs());
    if adelta.0 <= 1 && adelta.1 <= 1 {
        return tail;
    }
    match adelta {
        (adx, 0) => (tail.0 + delta.0 / adx, tail.1),
        (0, ady) => (tail.0, tail.1 + delta.1 / ady),
        (adx, ady) => (tail.0 + delta.0 / adx, tail.1 + delta.1 / ady)
    }
}