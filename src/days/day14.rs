use std::collections::HashMap;
use std::cmp;
use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

extern crate regex;

use regex::Regex;

pub struct Problem();

#[derive(PartialEq)]
pub enum Stuff {
    Rock,
    Sand
}

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut map : HashMap<(u32, u32), Stuff> = HashMap::new();

        parse_input(&input, &mut map);

        simulate1(&mut map).to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut map : HashMap<(u32, u32), Stuff> = HashMap::new();

        parse_input(&input, &mut map);

        simulate2(&mut map).to_string()
    }
}

fn parse_input(input: &AOCInput, map: &mut HashMap<(u32, u32), Stuff>) {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    for l in input.as_lines() {
        //print!("Line: ");
        let mut iter = re.captures_iter(l);
        let firstcap = iter.next().unwrap();
        let mut last = (firstcap[1].parse::<u32>().unwrap(), firstcap[2].parse::<u32>().unwrap());
        for cap in iter {
            let this = (cap[1].parse::<u32>().unwrap(), cap[2].parse::<u32>().unwrap());
            if last.0 == this.0 {
                for i in cmp::min(last.1, this.1)..(cmp::max(last.1, this.1)+1) {
                    map.insert((last.0, i), Stuff::Rock);
                }
            } else {
                for i in cmp::min(last.0, this.0)..(cmp::max(last.0, this.0)+1) {
                    map.insert((i, last.1), Stuff::Rock);
                }
            }
            last = this;
        }
    }
}

fn simulate1(map: &mut HashMap<(u32, u32), Stuff>) -> u32 {
    let mut depth = 0;
    for (_, d) in map.keys() {
        depth = cmp::max(*d, depth);
    }

    let mut count = 0;
    loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 == depth {
                return count;
            }
            if map.get(&(sand.0, sand.1 + 1)) == None {
                sand = (sand.0, sand.1 + 1);
            } else if map.get(&(sand.0 - 1, sand.1 + 1)) == None {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if map.get(&(sand.0 + 1, sand.1 + 1)) == None {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                map.insert(sand, Stuff::Sand);
                count += 1;
                break;
            }
        }
    }
}

fn simulate2(map: &mut HashMap<(u32, u32), Stuff>) -> u32 {
    let mut depth = 0;
    for (_, d) in map.keys() {
        depth = cmp::max(*d, depth);
    }

    let mut count = 0;
    loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 == depth + 1  {
                map.insert(sand, Stuff::Sand);
                count += 1;
                break;
            } else if map.get(&(sand.0, sand.1 + 1)) == None {
                sand = (sand.0, sand.1 + 1);
            } else if map.get(&(sand.0 - 1, sand.1 + 1)) == None {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if map.get(&(sand.0 + 1, sand.1 + 1)) == None {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                if sand == (500, 0) {
                    return count + 1;
                }
                map.insert(sand, Stuff::Sand);
                count += 1;
                break;
            }
        }
    }
}
