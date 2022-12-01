use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut max = 0;
        let elves = input.raw().trim().split("\n\n");
        for elf in elves {
            let cal = elf.lines().map(|x| x.parse::<i32>().unwrap()).sum();
            if cal > max {
                max = cal;
            }
        }
        max.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut calories : Vec<i32> = Vec::new();
        let elves = input.raw().trim().split("\n\n");
        for elf in elves {
            let cal = elf.lines().map(|x| x.parse::<i32>().unwrap()).sum();
            calories.push(cal);
        }
        calories.sort();
        calories.reverse();
        (calories[0] + calories[1] + calories[2]).to_string()
    }
} 