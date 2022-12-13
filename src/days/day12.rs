use std::collections::{HashMap, HashSet};
use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

// Oof. A lot of cruft in this one.

#[derive(Debug,PartialEq)]
enum Distance {
    Unknown,
    Value(u32)
}

#[derive(Debug)]
pub struct MountainMap {
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
    neighbors: HashMap<(usize, usize),Vec<(usize,usize)>>,
    distances: HashMap<(usize, usize),Distance>,
    low_points: Vec<(usize, usize)>
    //connections: Vec<Vec<u8>>,
    //distances: Vec<Vec<Distance>>
}

impl MountainMap {
    fn new(heightmap: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> Self {
        let height = heightmap.len();
        let width = heightmap[0].len();
        let mut neighbors: HashMap<(usize, usize),Vec<(usize,usize)>> = HashMap::new();
        let mut distances: HashMap<(usize, usize),Distance> = HashMap::new();
        let mut low_points = Vec::new();
        low_points.push(start);
        for r in 0..height {
            for c in 0..width {
                let limit = heightmap[r][c] + 1;
                if limit == 1 {
                    low_points.push((r, c));
                }
                distances.insert((r, c), if (r, c) == start { Distance::Value(0) } else { Distance::Unknown} );
                let mut nvec = Vec::new();
                // Check right
                if c < width - 1 && heightmap[r][c+1] <= limit {
                    nvec.push((r, c+1));
                }
                // check left
                if c >= 1 && heightmap[r][c-1] <= limit {
                    nvec.push((r, c-1));
                }
                // check down
                if r < height - 1 && heightmap[r+1][c] <= limit {
                    nvec.push((r+1, c));
                }
                // check up
                if r >= 1 && heightmap[r - 1][c] <= limit {
                    nvec.push((r-1, c));
                }
                neighbors.insert((r, c), nvec);
            }
        }
        dbg!(low_points.len());
        Self {
            height,
            width,
            start,
            end,
            neighbors,
            distances,
            low_points
        }
    }

    fn reset(&mut self) {
        for r in 0..self.height {
            for c in 0..self.width {
                self.distances.insert((r, c), if (r, c) == self.start { Distance::Value(0) } else { Distance::Unknown });
            }
        }
    }

    fn find_end(&mut self, which: usize) -> u32 {
        let mut frontier = Vec::new();
        frontier.push(self.low_points[which]);
        let mut d = 1;
        'main: loop {
            let mut new_frontier = HashSet::new();
            while let Some(loc) = frontier.pop() {
                for n in self.neighbors.get(&loc).unwrap() {
                    if *n == self.end {
                        break 'main
                    }
                    let dist = self.distances.get_mut(n).unwrap();
                    //dbg!(d, n, &dist);
                    if *dist == Distance::Unknown {
                        *dist = Distance::Value(d);
                        new_frontier.insert(*n);
                    }
                }
            }
            for loc in new_frontier.drain() {
                frontier.push(loc);
            }
            d += 1;
            if d > 1000 {
                break 'main;
            }
        }
        d
    }

    fn find_best(&mut self) -> u32 {
        let mut best = 1000;
        for i in 0..self.low_points.len() {
            let t = self.find_end(i);
            if t < best {
                best = t;
            }
            self.reset();
        }
        best
    }
}

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let (heightmap, start, end) = parse_heightmap(input);

        let mut mmap = MountainMap::new(&heightmap, start, end);

        mmap.find_end(0).to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let (heightmap, start, end) = parse_heightmap(input);

        let mut mmap = MountainMap::new(&heightmap, start, end);

        mmap.find_best().to_string()

    }
}

fn parse_heightmap (input: AOCInput) -> (Vec<Vec<i32>>, (usize, usize), (usize, usize)) {
    let mut heightmap : Vec<Vec<i32>> = Vec::new();

    for l in input.as_lines() {
        heightmap.push(
            l.chars().into_iter().map(|x| (x as i32) - 97).collect()
        )
    }
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for r in 0..heightmap.len() {
        for c in 0..heightmap[0].len() {
            if heightmap[r][c] == -14 {
                heightmap[r][c] = 0;
                start = (r, c);
            } else if heightmap[r][c] == -28 {
                heightmap[r][c] = 0;
                end = (r, c);
            }
        }
    }
    //dbg!(&heightmap, start, end);
    (heightmap, start, end)
}