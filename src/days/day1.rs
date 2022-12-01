use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut total = 0;
        let masses = input.as_numvec();
        for m in masses {
            total = total + (m / 3 - 2);
        }
        total.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut total = 0;
        let masses = input.as_numvec();
        for m in masses {
            let mut fuel = (m / 3) - 2;
            while fuel > 0 {
                total = total + fuel;
                fuel = fuel / 3 - 2;
            }
        }
        total.to_string()
    }
} 