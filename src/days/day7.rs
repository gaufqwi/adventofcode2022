use std::collections::HashMap;
//use std::rc::Rc;

use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

#[derive(Debug)]
pub struct Dir {
    name: String,
    parent: Option<usize>,
    dirs: HashMap<String, usize>,
    files: HashMap<String, i32>,
    size: Option<i32>
}

impl Dir {
    fn new(parent : usize, name: &str) -> Self {
        Dir {
            name: String::from(name),
            parent: Some(parent),
            dirs: HashMap::new(),
            files: HashMap::new(),
            size: None
        }
    }
}

pub struct Arena {
    contents: Vec<Dir>
}

impl Arena {
    fn new() -> Self {
        Arena {
            contents: vec![Dir { name: String::from("/"), parent: None, dirs: HashMap::new(), files: HashMap::new(), size: None }]
        }
    }

    fn get_parent(&self, parent: usize) -> usize {
        self.contents[parent].parent.unwrap()
    }

    fn add_dir(&mut self, parent: usize, name: &str) -> usize {
        match self.contents[parent].dirs.get(name) {
            Some(&index) => index,
            None => {
                let index = self.contents.len();
                self.contents.push(Dir::new(parent, name) );
                self.contents[parent].dirs.insert(String::from(name), index);
                index
            }
        }
    }

    fn add_file(&mut self, parent: usize, name: &str, filesize: i32) {
        self.contents[parent].files.insert(String::from(name), filesize);
    }

    fn get_size(&mut self, dir: usize) -> i32 {
        match self.contents[dir].size {
            Some(s) => s,
            None => {
                let mut total = 0;
                for v in self.contents[dir].files.values() {
                    total += *v;
                }
                let dirs: Vec<usize> = self.contents[dir].dirs.values().map(|x| *x).collect();
                //let dirs = self.contents.to_vec();
                for d in dirs {
                    total += self.get_size(d);
                }
                self.contents[dir].size = Some(total);
                total
            }
        }
    }

    fn pprint(&self, dir: usize, depth: i32)  {
        let mut pad = String::from("");
        for i in 0..depth {
            pad.push_str("    ");
        }
        let mut ftotal = 0;
        for v in self.contents[dir].files.values() {
            ftotal += *v;
        }
        println!("{}{} ({}) [{}]", pad, self.contents[dir].name, self.contents[dir].size.unwrap(), ftotal);
        let dirs: Vec<usize> = self.contents[dir].dirs.values().map(|x| *x).collect();
        for d in dirs {
            self.pprint(d, depth+1);
        }
    }
}


impl AOCProblem for Problem {

    fn part1(&self, input: AOCInput) -> String {
        let mut arena = Arena::new();
        let mut curdir : usize = 0;

        for l in input.as_lines() {
            let components: Vec<&str> = l.split(' ').collect();
            if components[0] == "$" && components[1] == "cd" {
                // CD command
                match components[2] {
                    "/" => {
                        curdir = 0
                    }
                    ".." => {
                        curdir = arena.get_parent(curdir);
                    }
                    subdir => {
                        curdir = arena.add_dir(curdir, subdir);
                    }
                }
            } else if components[0].chars().next().unwrap().is_numeric() {
                let filesize = components[0].parse::<i32>().unwrap();
                arena.add_file(curdir, components[1], filesize);
            }
        }
        arena.get_size(0);
        let mut total = 0;
        for d in arena.contents {
            let t = d.size.unwrap();
            if t <= 100000 {
                total += t;
            }
        }
        total.to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut arena = Arena::new();
        let mut curdir : usize = 0;

        for l in input.as_lines() {
            let components: Vec<&str> = l.split(' ').collect();
            if components[0] == "$" && components[1] == "cd" {
                // CD command
                match components[2] {
                    "/" => {
                        curdir = 0
                    }
                    ".." => {
                        curdir = arena.get_parent(curdir);
                    }
                    subdir => {
                        curdir = arena.add_dir(curdir, subdir);
                    }
                }
            } else if components[0].chars().next().unwrap().is_numeric() {
                let filesize = components[0].parse::<i32>().unwrap();
                arena.add_file(curdir, components[1], filesize);
            }
        }
        let mut size_of_best = arena.get_size(0);
        let needed = 30000000 - (70000000 - size_of_best);
        for i in 0..(arena.contents.len()) {
            let t = arena.contents[i].size.unwrap();
            if (t >= needed) && (t < size_of_best) {
                size_of_best = t;
            }
        }
        size_of_best.to_string()
    }
} 