use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut buf = vec!['X', 'X', 'X', 'X'];
        for (i, c) in input.raw().trim().chars().enumerate() {
            rotate(&mut buf, 4, c);
            //dbg!("{}", &buf);
            if all_different(&buf, 4) && i >= 3 {
                return (i + 1).to_string();
            }
        }
        String::from("failure")
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut buf = vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'];
        for (i, c) in input.raw().trim().chars().enumerate() {
            rotate(&mut buf, 14, c);
            //dbg!("{}", &buf);
            if all_different(&buf, 14) && i >= 13 {
                return (i + 1).to_string();
            }
        }
        String::from("failure")
    }
}

fn rotate (buf : &mut Vec<char>, size : usize, c : char) {
    for i in 0..(size-1) {
        buf[i] = buf[i+1]
    }
    buf[size-1] = c;
}

fn all_different (buf : &Vec<char>, size : usize) -> bool {
    for i in 0..(size-1) {
        for j in (i+1)..size {
            if buf[i] == buf[j] {
                return false;
            }
        }
    }
    true
}