use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut score = 0;
        for l in input.as_lines() {
            let (opp, me) = l.split_once(" ").unwrap();
            match me {
                // rock
                "X" => {
                    score += 1;
                    if opp == "A" {
                        score += 3  // draw
                    } else if opp == "C" {
                        score += 6  // win
                    }
                }
                // paper
                "Y" => {
                    score += 2;
                    if opp == "B" {
                        score += 3  // draw
                    } else if opp == "A" {
                        score += 6  // win
                    }
                }
                // scissors
                "Z" => {
                    score += 3;
                    if opp == "C" {
                        score += 3  // draw
                    } else if opp == "B" {
                        score += 6  // win
                    }
                }
                _ => panic!("Bad input / parsing")
            }
        }
        score.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut score = 0;
        for l in input.as_lines() {
            let (opp, me) = l.split_once(" ").unwrap();
            match me {
                // lose
                "X" => {
                    if opp == "A" {
                        score += 3  // scissors < rock
                    } else if opp == "B" {
                        score += 1  // rock < paper
                    } else {
                        score += 2  // paper < scissors
                    }
                }
                // draw
                "Y" => {
                    score += 3;
                    if opp == "A" {
                        score += 1  // rock = rock
                    } else if opp == "B" {
                        score += 2  // paper = paper
                    } else {
                        score += 3  // scissors = scissors
                    }
                }
                // win
                "Z" => {
                    score += 6;
                    if opp == "A" {
                        score += 2  // paper > rock
                    } else if opp == "B" {
                        score += 3  // scissors > paper
                    } else {
                        score += 1  // rock > scissors
                    }
                }
                _ => panic!("Bad input / parsing")
            }
        }
        score.to_string()
    }
} 