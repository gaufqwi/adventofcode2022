use std::collections::HashSet;
use crate::aoc::AOCProblem;
use crate::aoc::AOCInput;

pub struct Problem();

impl AOCProblem for Problem {
    fn part1(&self, input: AOCInput) -> String {
        let mut grid : Vec<Vec<i32>> = Vec::new();

        for l in input.as_lines() {
            grid.push(l.chars().into_iter().map(|x| x as i32 - 0x30).collect::<Vec<i32>>());
        }
        let height = grid.len();
        let width = grid[0].len();
        //let mut visible = 0;
        let mut visible:  HashSet<(usize,usize)> = HashSet::new();

        // Count from left
        for row in 0..height {
            let mut highest = -1;
            for c in 0..width {
                if grid[row][c] > highest {
                    //visible += 1;
                    visible.insert((row, c));
                    highest = grid[row][c];
                }
                if highest == 9 {
                    break;
                }
            }
        }

        // Count from right
        for row in 0..height {
            let mut highest = -1;
            for c in 0..width {
                if grid[row][width - c - 1] > highest {
                    //visible += 1;
                    visible.insert((row, width - c - 1));
                    highest = grid[row][width - c - 1];
                }
                if highest == 9 {
                    break;
                }
            }
        }

        // Count from top
        for col in 0..width {
            let mut highest = -1;
            for r in 0..height {
                if grid[r][col] > highest {
                    //visible += 1;
                    visible.insert((r, col));
                    highest = grid[r][col];
                }
                if highest == 9 {
                    break;
                }
            }
        }

        // Count from bottom
        for col in 0..width {
            let mut highest = -1;
            for r in 0..height {
                if grid[height - r - 1][col] > highest {
                    //visible += 1;
                    visible.insert((height - r - 1, col));
                    highest = grid[height - r - 1][col];
                }
                if highest == 9 {
                    break;
                }
            }
        }

        visible.len().to_string()
    }

    fn part2(&self, input: AOCInput) -> String {
        let mut grid : Vec<Vec<i32>> = Vec::new();

        for l in input.as_lines() {
            grid.push(l.chars().into_iter().map(|x| x as i32 - 0x30).collect::<Vec<i32>>());
        }
        let height = grid.len();
        let width = grid[0].len();

        let mut best_view = 0;

        for r in 1..(height-1) {
            for c in 1..(width-1) {
                let tree = grid[r][c];
                let mut view = 1;

                // look right
                let mut i = 1;
                while grid[r][c+i] < tree && c+i < width-1 {
                    i += 1;
                }
                view *= i;

                // look left
                i = 1;
                while grid[r][c-i] < tree && c-i > 0 {
                    i += 1;
                }
                view *= i;

                // look down
                i = 1;
                while grid[r+i][c] < tree && r+i < height-1 {
                    i += 1;
                }
                view *= i;

                // look up
                i = 1;
                while grid[r-i][c] < tree && r-i > 0 {
                    i += 1;
                }
                view *= i;

                if view > best_view {
                    best_view = view;
                }
            }
        }

        best_view.to_string()
    }
} 