use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut total : u32 = 0;
        for l in input.as_lines() {
            let first = &l[..l.len()/2 as usize];
            let second = &l[l.len()/2 as usize..];
            let first_set : HashSet<char, RandomState> = HashSet::from_iter(first.chars());
            let second_set : HashSet<char, RandomState> = HashSet::from_iter(second.chars());
            let common = first_set.intersection(&second_set).next().unwrap();
            if common.is_lowercase() {
                total += (*common as u32) - 96;
            } else {
                total += (*common as u32) - 64 + 26;
            }
        }
        total.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut total : u32 = 0;
        let mut sacks = vec!["", "", ""];
        for (i, l) in input.as_lines().enumerate() {
            let j = i % 3;
            sacks[j] = l;
            if j == 2 {
                let first_set : HashSet<char, RandomState> = HashSet::from_iter(sacks[0].chars());
                let second_set : HashSet<char, RandomState> = HashSet::from_iter(sacks[1].chars());
                let third_set : HashSet<char, RandomState> = HashSet::from_iter(sacks[2].chars());
                let inter1 : HashSet<char, RandomState> = HashSet::from_iter(first_set.intersection(&second_set).into_iter().map(|c| *c));
                let common = third_set.intersection(&inter1).next().unwrap();
                if common.is_lowercase() {
                    total += (*common as u32) - 96;
                } else {
                    total += (*common as u32) - 64 + 26;
                }
            }
        }
        total.to_string()
    }
} 