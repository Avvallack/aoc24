use std::collections::HashSet;
use itertools::iproduct;
use pathfinding::prelude::Matrix;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Guard {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Guard {
    fn turn_right(&mut self) {
       match self.dir {
              (0, -1) => self.dir = (-1, 0),
              (1, 0) => self.dir = (0, -1),
              (0, 1) => self.dir = (1, 0),
              (-1, 0) => self.dir = (0, 1),
           _ => panic!("Invalid direction"),
       }
    }
    fn make_move(&mut self) {
        self.pos.0 = (self.pos.0 as isize + self.dir.0) as usize;
        self.pos.1 = (self.pos.1 as isize + self.dir.1) as usize;
    }
    fn next_pos(&self) -> (usize, usize) {
        ((self.pos.0 as isize + self.dir.0) as usize, (self.pos.1 as isize + self.dir.1) as usize)
    }
}

fn walk_path(grid: &Matrix<char>, mut guard: Guard) -> Matrix<bool>{
    let mut visited = Matrix::new(grid.rows, grid.columns, false);
    visited[guard.pos] = true;

    while let Some(c) = grid.get(guard.next_pos()) {
        if *c == '.' {
            guard.make_move();
            visited[guard.pos] = true;
            continue;
        }
        if *c == '#' {
            guard.turn_right();
            
        }
    }
    visited
}

#[aoc_generator(day6)]
fn read_inputs(input: &str) -> Matrix<char> {
    let v = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    Matrix::from_rows(v).unwrap()
}

fn get_start_pos(grid: &Matrix<char>) -> (usize, usize) {
    grid.keys().find(|&(r, c)| grid[(r, c)] == '^').unwrap()
}

#[aoc(day6, part1)]
fn part1(inp: &Matrix<char>) -> usize {
    let mut grid = inp.clone();
    let (start_row, start_col) = get_start_pos(&grid);

    grid[(start_row, start_col)] = '.';

    let guard = Guard { pos: (start_row, start_col), dir: (-1, 0) };
    let visited = walk_path(&grid, guard);
    visited.values().filter(|&v| *v).count()
}

fn count_loops(inp: &Matrix<char>) -> usize {
    let mut count = 0;
    let mut grid = inp.clone();
    let (start_row, start_col) = get_start_pos(&grid);
    grid[(start_row, start_col)] = '.';

    let start_guard = Guard { pos: (start_row, start_col), dir: (-1, 0) };

    let visited = walk_path(&grid, start_guard);

    for (r, c) in iproduct!(0..inp.rows, 0..inp.columns) {
        if grid[(r, c)] == '#' || !visited[(r, c)] || (r == start_row && c == start_col) {
            continue;
        }
        grid[(r, c)] = '#';
        let mut g = start_guard;

        let mut v = HashSet::new();
        while let Some(c) = grid.get(g.next_pos()) {
            if !v.insert(g) {
                count += 1;
                break;
            }
            match c {
                '.' => g.make_move(),
                '#' => g.turn_right(),
                _ => panic!("Invalid character"),
                
            };
        }
        grid[(r, c)] = '.';
    }

    count
}

#[aoc(day6, part2)]
fn part2(inp: &Matrix<char>) -> usize {
    count_loops(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "....#.....\n\
                              .........#\n\
                              ..........\n\
                              ..#.......\n\
                              .......#..\n\
                              ..........\n\
                              .#..^.....\n\
                              ........#.\n\
                              #.........\n\
                              ......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_inputs(EXAMPLE1)), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_inputs(EXAMPLE1)), 6);
    }
}

