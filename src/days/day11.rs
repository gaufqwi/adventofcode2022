use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

extern crate regex;

use regex::Regex;

pub struct Problem();

#[derive(Debug,Clone)]
pub enum Operation {
    Add,
    Multiply
}

#[derive(Debug,Clone)]
pub enum Operand {
    Value(i64),
    Old
}

#[derive(Debug,Clone)]
pub struct Monkey {
    stuff: Vec<i64>,
    operation: Operation,
    operand: Operand,
    test_divisor: i64,
    true_dest: usize,
    false_dest: usize,
    inspections: i64
}
#[derive(Debug)]
pub struct MonkeyArena {
    modulus: i64,
    monkeys: Vec<Monkey>
}

impl MonkeyArena {
    fn new() -> Self {
        Self {
            modulus: 1,
            monkeys: Vec::new()
        }
    }

    fn add_monkey(&mut self, stuff: Vec<i64>, operation: Operation, operand: Operand, test_divisor: i64, true_dest: usize, false_dest: usize) {
        self.monkeys.push(Monkey {
            stuff,
            operation,
            operand,
            test_divisor,
            true_dest,
            false_dest,
            inspections: 0
        });
        self.modulus *= test_divisor;
    }

    fn get_activity(&self) -> Vec<i64> {
        return self.monkeys.clone().into_iter().map(|x| x.inspections).collect();
    }

    fn simulate_turn(&mut self, divide: bool) {
        for i in 0..self.monkeys.len() {
            self.simulate_monkey(i, divide);
        }
    }

    fn simulate_monkey(&mut self, which: usize, divide: bool) {
        for i in 0..self.monkeys[which].stuff.len() {
            let mut worry = self.monkeys[which].stuff.remove(0);
            if let Operand::Value(x) = self.monkeys[which].operand {
                match self.monkeys[which].operation {
                    Operation::Add => worry += x,
                    Operation::Multiply => worry *= x
                }
            } else {
                match self.monkeys[which].operation {
                    Operation::Add => worry += worry,
                    Operation::Multiply => worry *= worry
                }
            }
            if divide {
                worry /= 3;
            }
            worry %= self.modulus;
            if worry % self.monkeys[which].test_divisor == 0 {
                let dest = self.monkeys[which].true_dest;
                self.monkeys[dest].stuff.push(worry);
            } else {
                let dest = self.monkeys[which].false_dest;
                self.monkeys[dest].stuff.push(worry);
            }
            self.monkeys[which].inspections += 1;
        }
    }


}

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut arena = MonkeyArena::new();

        parse_monkeys(&mut arena, input.raw().trim());

        for i in 0..20 {
            arena.simulate_turn(true);
        }
        let mut activity = arena.get_activity();
        activity.sort();
        activity.reverse();

        let result = activity[0] * activity[1];

        result.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut arena = MonkeyArena::new();

        parse_monkeys(&mut arena, input.raw().trim());

        for i in 0..10000 {
            arena.simulate_turn(false);
        }
        let mut activity = arena.get_activity();
        activity.sort();
        activity.reverse();

        let result = activity[0] * activity[1];

        result.to_string()
    }
}

fn parse_monkeys(arena: &mut MonkeyArena, input_text: &str) {
    for monkey_entry in input_text.split("\n\n") {
        let mut monkey_lines = monkey_entry.lines();

        let mut re = Regex::new(r"^Monkey (\d+):$").unwrap();
        let mut captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let num = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();

        re = Regex::new(r"^\s*Starting items: (.*)$").unwrap();
        captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let stuff: Vec<i64> = captures.get(1).unwrap().as_str().split(", ")
            .map(|x| x.parse::<i64>().unwrap()).collect();

        re = Regex::new(r"^\s*Operation: new = old ([+*]) (\d+|old)$").unwrap();
        captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let operation = if captures.get(1).unwrap().as_str() == "*" {Operation::Multiply } else { Operation::Add };
        let t = captures.get(2).unwrap().as_str();
        let operand : Operand;
        if t == "old" {
            operand = Operand::Old;
        } else {
            operand = Operand::Value(t.parse::<i64>().unwrap());
        }

        re = Regex::new(r"^\s*Test: divisible by (\d+)$").unwrap();
        captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let test_divisor = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();

        re = Regex::new(r"^\s*If true: throw to monkey (\d+)$").unwrap();
        captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let true_dest = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

        re = Regex::new(r"^\s*If false: throw to monkey (\d+)$").unwrap();
        captures = re.captures(monkey_lines.next().unwrap()).unwrap();
        let false_dest = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

        arena.add_monkey(stuff, operation, operand, test_divisor, true_dest, false_dest);
    }
}