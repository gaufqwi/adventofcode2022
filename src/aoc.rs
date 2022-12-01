use std::str::Lines;

pub trait AOCProblem {
    fn solve (&self, part : &str, input : AOCInput) -> String {
        match part {
            "1" => self.part1(input),
            "2" => self.part2(input),
            _ => panic!("Unknown part")
        }
    }

    fn part1 (&self, input : AOCInput ) -> String {
        String::from("Part 1 not implemented")
    }

    fn part2 (&self, input : AOCInput ) -> String {
        String::from("Part 2 not implemented")
    }
}

pub struct AOCInput {
    data : String
}

impl AOCInput {
    pub fn new (data : String) -> AOCInput {
        AOCInput {
            data: data.clone()
        }
    }

    // Utility methods to return the data in various ways

    // Raw string
    pub fn raw (&self) -> &str {
        &self.data
    }

    // Iterator over lines
    pub fn as_lines (&self) -> Lines<'_> {
        self.data.lines()
    }

    // Vec of numbers
    pub fn as_numvec (&self) -> Vec<i64> {
        self.data.lines().map(|x| x.parse::<i64>().unwrap()).collect()
    }
}