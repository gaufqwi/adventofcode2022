use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut subsets = 0;
        for l in input.as_lines() {
            let (elfa, elfb) = l.split_once(",").unwrap();
            let (elfa_s, elfa_e) = elfa.split_once("-").unwrap();
            let (elfb_s, elfb_e) = elfb.split_once("-").unwrap();
            let elfa_si = elfa_s.parse::<i32>().unwrap();
            let elfa_ei = elfa_e.parse::<i32>().unwrap();
            let elfb_si = elfb_s.parse::<i32>().unwrap();
            let elfb_ei = elfb_e.parse::<i32>().unwrap();
            if (elfa_si >= elfb_si && elfa_ei <= elfb_ei) || (elfb_si >= elfa_si && elfb_ei <= elfa_ei) {
                subsets += 1;
            }
        }
        subsets.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut subsets = 0;
        for l in input.as_lines() {
            let (elfa, elfb) = l.split_once(",").unwrap();
            let (elfa_s, elfa_e) = elfa.split_once("-").unwrap();
            let (elfb_s, elfb_e) = elfb.split_once("-").unwrap();
            let elfa_si = elfa_s.parse::<i32>().unwrap();
            let elfa_ei = elfa_e.parse::<i32>().unwrap();
            let elfb_si = elfb_s.parse::<i32>().unwrap();
            let elfb_ei = elfb_e.parse::<i32>().unwrap();
            if (elfa_si >= elfb_si && elfa_si <= elfb_ei) || (elfa_ei >= elfb_si && elfa_si <= elfb_ei) {
                subsets += 1;
            }
        }
        subsets.to_string()
    }
} 