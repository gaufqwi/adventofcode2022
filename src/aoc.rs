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
}