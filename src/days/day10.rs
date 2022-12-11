use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut total = 0;
        let mut cycle = 1;
        let mut x = 1;

        for l in input.as_lines() {
            let instruction: Vec<&str> = l.split(' ').collect();
            let mut cycle_time = 1;
            let mut delta = 0;
            if instruction.len() == 2 {
                cycle_time = 2;
                delta = instruction[1].parse::<i32>().unwrap();
            }

            for i in 0..cycle_time {
                if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                    total += x * cycle;
                }
                cycle += 1;
            }
            x += delta;
        }

        total.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut pixel = 0;
        let mut x : i32 = 1;
        let mut crt = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for i in 0..6 {
            crt[i].resize(40, '.');
        }

        for l in input.as_lines() {
            let instruction: Vec<&str> = l.split(' ').collect();
            let mut cycle_time = 1;
            let mut delta : i32 = 0;
            if instruction.len() == 2 {
                cycle_time = 2;
                delta = instruction[1].parse::<i32>().unwrap();
            }

            for i in 0..cycle_time {
                let row = pixel / 40;
                let col = pixel % 40;
                if x-1 == col || x == col || x+1 == col {
                    crt[row as usize][col as usize] = '#';
                }
                pixel += 1;
            }
            x += delta;
        }

        // Show screen
        for i in 0..6 {
            let s : String = crt[i].clone().into_iter().collect();
            println!("{}", s);
        }

        String::from("See above ^^^")
    }
} 