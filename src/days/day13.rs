use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

extern crate serde_json;

use serde_json::{json, Value};

pub struct Problem();

#[derive(PartialEq)]
pub enum Comparison {
    Right,
    Wrong,
    Unknown
}

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {

        let mut total = 0;
        let mut i = 1;
        for pair in input.raw().trim().split("\n\n") {
            let mut lines = pair.lines();

            let mut first : Value = serde_json::from_str(lines.next().unwrap()).unwrap();
            let mut second : Value = serde_json::from_str(lines.next().unwrap()).unwrap();

            if compare(&mut first, &mut second) == Comparison::Right {
                total += i;
            }

            i += 1;
        }


        total.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        String::from("Part 2")
    }
}

fn compare (a: &mut Value, b: &mut Value) -> Comparison {
    let mut a_vec = a.as_array().unwrap().into_iter();
    let mut b_vec = b.as_array().unwrap().into_iter();

    loop {
        let a_el = a_vec.next();
        let b_el = b_vec.next();
        match a_el {
            None => {
                match b_el {
                    None => return Comparison::Unknown,
                    _ => return Comparison::Right
                }
            },
            Some(Value::Number(x)) => {
                match b_el {
                    None => return Comparison::Wrong,
                    // Number
                    Some(Value::Number(y)) => {
                        if x.as_i64().unwrap() < y.as_i64().unwrap() {
                            return Comparison::Right
                        } else if x.as_i64().unwrap() > y.as_i64().unwrap() {
                            return Comparison::Wrong
                        }
                    },
                    // Array
                    Some(y) => {
                        let mut ym = y.clone();
                        let r = compare(&mut json!([x]), &mut ym);
                        if r != Comparison::Unknown {
                            return r;
                        }
                    }
                }
            },
            Some(x) => {
                match b_el {
                    None => return Comparison::Wrong,
                    Some(Value::Number(y)) => {
                        let mut xm = x.clone();
                        let r = compare(&mut xm, &mut json!([y]));
                        if r != Comparison::Unknown {
                            return r;
                        }
                    },
                    Some(y) => {
                        let mut xm = x.clone();
                        let mut ym = y.clone();
                        let r = compare(&mut xm, &mut ym);
                        if r != Comparison::Unknown {
                            return r;
                        }
                    }
                }
            }
        }
    }
}