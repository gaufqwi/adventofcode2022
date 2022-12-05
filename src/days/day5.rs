use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

extern crate regex;

use regex::Regex;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut lines = input.as_lines();
        let mut stacks : Vec<Vec<char>>= Vec::new();

        for i in 0..9 {
            stacks.push(Vec::<char>::new());
        }

        while let Some(l) = lines.next() {
            if l.chars().nth(1).unwrap() == '1' {
                lines.next();
                break;
            }
            for i in 0..9 {
                let c = l.chars().nth(1 + 4 * i).unwrap();
                if c != ' ' {
                    //stacks[i].push(c);
                    stacks.get_mut(i).unwrap().push(c);
                }
            }
        }
        for i in 0..9 {
            stacks.get_mut(i).unwrap().reverse();
        }

        let dir_re = Regex::new(r"move ([0-9]+) from ([0-9]) to ([0-9])").unwrap();
        while let Some(l) = lines.next() {
            let cap = dir_re.captures(l).unwrap();
            let num  = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let from = cap.get(2).unwrap().as_str().parse::<i32>().unwrap() - 1;
            let to = cap.get(3).unwrap().as_str().parse::<i32>().unwrap() - 1;
            for i in 0..num {
                let c = stacks.get_mut(from as usize).unwrap().pop().unwrap();
                stacks.get_mut(to as usize).unwrap().push(c);
            }
        }
        let mut result = String::new();
        for i in 0..9 {
            result.push(stacks.get_mut(i).unwrap().pop().unwrap())
        }
        result
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut lines = input.as_lines();
        let mut stacks : Vec<Vec<char>>= Vec::new();

        for i in 0..9 {
            stacks.push(Vec::<char>::new());
        }

        while let Some(l) = lines.next() {
            if l.chars().nth(1).unwrap() == '1' {
                lines.next();
                break;
            }
            for i in 0..9 {
                let c = l.chars().nth(1 + 4 * i).unwrap();
                if c != ' ' {
                    //stacks[i].push(c);
                    stacks.get_mut(i).unwrap().push(c);
                }
            }
        }
        for i in 0..9 {
            stacks.get_mut(i).unwrap().reverse();
        }

        let dir_re = Regex::new(r"move ([0-9]+) from ([0-9]) to ([0-9])").unwrap();
        while let Some(l) = lines.next() {
            let cap = dir_re.captures(l).unwrap();
            let num  = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let from = cap.get(2).unwrap().as_str().parse::<i32>().unwrap() - 1;
            let to = cap.get(3).unwrap().as_str().parse::<i32>().unwrap() - 1;
            let mut temp = Vec::<char>::new();
            for i in 0..num {
                let c = stacks.get_mut(from as usize).unwrap().pop().unwrap();
                temp.push(c)
            }
            for i in 0..num {
                stacks.get_mut(to as usize).unwrap().push(temp.pop().unwrap());
            }
        }
        let mut result = String::new();
        for i in 0..9 {
            result.push(stacks.get_mut(i).unwrap().pop().unwrap())
        }
        result
    }
}
