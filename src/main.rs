#![allow(dead_code, unused_variables)]

extern crate reqwest;
extern crate core;

mod aoc;
mod days;

use std::{env, fs};
use std::io::{self, Read};
use std::fs::File;
use std::error::Error;
use crate::aoc::AOCProblem;
use aoc::AOCInput;

const YEAR: &str = "2022";

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Need exactly two arguments")
    }
    let day : &str = &args[1];
    let part : &str = &args[2];

    fs::create_dir_all("cache/").expect("Problem creating cache directory");
    let cookie = get_cookie().expect("Problem reading cookie from file");

    let input = get_input(YEAR, day, cookie.trim())
        .expect("Problem getting input");

    // A really inelegant way to do polymorphism but I couldn't figure out a better one
    let problem : Box<dyn AOCProblem> = match day {
        "1" => Box::new(days::day1::Problem()),
        "2" => Box::new(days::day2::Problem()),
        "3" => Box::new(days::day3::Problem()),
        "4" => Box::new(days::day4::Problem()),
        "5" => Box::new(days::day5::Problem()),
        "6" => Box::new(days::day6::Problem()),
        "7" => Box::new(days::day7::Problem()),
        "8" => Box::new(days::day8::Problem()),
        "9" => Box::new(days::day9::Problem()),
        "10" => Box::new(days::day10::Problem()),
        "11" => Box::new(days::day11::Problem()),
        "12" => Box::new(days::day12::Problem()),
        "13" => Box::new(days::day13::Problem()),
        "14" => Box::new(days::day14::Problem()),
        "15" => Box::new(days::day15::Problem()),
        "16" => Box::new(days::day16::Problem()),
        "17" => Box::new(days::day17::Problem()),
        "18" => Box::new(days::day18::Problem()),
        "19" => Box::new(days::day19::Problem()),
        "20" => Box::new(days::day20::Problem()),
        "21" => Box::new(days::day21::Problem()),
        "22" => Box::new(days::day22::Problem()),
        "23" => Box::new(days::day23::Problem()),
        "24" => Box::new(days::day24::Problem()),
        "25" => Box::new(days::day25::Problem()),
        _ => panic!("Unknown day")
    };
    println!("Solution for {} day {} part {}: {}", YEAR, day, part, problem.solve(&part, input))
}

fn get_input(year :&str, day: &str, cookie: &str) -> Result<AOCInput, Box<dyn Error>> {
    let filename = format!("cache/input_{}_{}.txt", year, day);
    let mut data = String::new();
    //let mut input_file = File::open(filename)?;
    match File::open(&filename) {
        Ok(mut input_file) => {
            input_file.read_to_string(&mut data)?;
            Ok(AOCInput::new(data))
        }
        _ => {
            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let client = reqwest::blocking::Client::new();
            let data = client.get(url)
                .header("Cookie", format!("session={}", cookie))
                .send()?
                .text()?;
            fs::write(&filename, &data)?;
            Ok(AOCInput::new(data))
        }
    }
    //input_file.read_to_string(&mut input)?;
    //Ok(input)
}

fn get_cookie() -> Result<String, io::Error> {
    let mut cookie_file = File::open("cookie.txt")?;
    let mut cookie = String::new();
    cookie_file.read_to_string(&mut cookie)?;
    Ok(cookie)
}