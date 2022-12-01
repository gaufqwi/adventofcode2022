use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut mem : Vec<i32> = input.raw().trim().split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        mem[1] = 12;
        mem[2] = 2;
        run(mem).to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let basemem : Vec<i32> = input.raw().trim().split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for noun in 0..100 {
            for verb in 0..100 {
                let mut mem = basemem.clone();
                mem[1] = noun;
                mem[2] = verb;
                if run(mem) == 19690720 {
                    return (noun*100 + verb).to_string();
                }
            }
        }
        String::from("Search failed")
    }

}

fn run (mut mem : Vec<i32>) -> i32 {
        // let mut mem : Vec<i32> = input.raw().trim().split(",")
        //     .map(|x| x.parse::<i32>().unwrap())
        //     .collect();
        let mut pc = 0;
        let mut op = mem[pc];
        while op != 99 {
            let op1 = mem[mem[pc+1] as usize];
            let op2 = mem[mem[pc+2] as usize];
            let dest = mem[pc+3];
            if op == 1 {
                mem[dest as usize] = op1 + op2;
            } else if op == 2 {
                mem[dest as usize] = op1 * op2;
            } else {
                panic!("Bad opcode");
            }
            pc = pc + 4;
            op = mem[pc];
        }
        mem[0]
}