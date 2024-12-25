use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::Matrix;


struct Input {
    points: HashMap<char, Vec<(usize, usize)>>,
    grid: Matrix<char>,
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

#[aoc_generator(day08)]
fn read_inputs(input: &str) -> Input {
    let v = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let grid: Matrix<char> = Matrix::from_rows(v).unwrap();
    let mut points = HashMap::new();
    for r in 0..grid.rows {
        for c in 0..grid.columns {
            if grid[(r, c)] != '.' {
                let entry = points.entry(grid[(r, c)]).or_insert(Vec::new());
                entry.push((r, c));
            }
        }
    }
    Input {
        points,
        grid,
    }
}

fn get_antinodes(p1: &(usize, usize), p2: &(usize, usize), max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let dx = p2.1 as i32 - p1.1 as i32;
    let dy = p2.0 as i32 - p1.0 as i32;

    let mut antinodes = vec![];
    for &t in &[-1, 2] {
        let px: i32 = p1.1 as i32 + t * dx;
        let py = p1.0 as i32 + t * dy;
        if px >= 0 && px < max_x as i32 && py >= 0 && py < max_y as i32 {
            antinodes.push((py as usize, px as usize));
        }
        
    }
    antinodes
}

fn get_antinodes_harmonics(points: &Vec<(usize, usize)>, max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut unique_antinodes = HashSet::new();

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let pair_antinodes = get_line_points_in_bounds(&points[i], &points[j], max_x, max_y);
            for p in pair_antinodes {
                unique_antinodes.insert(p);
            }
            unique_antinodes.insert(points[i]);
            unique_antinodes.insert(points[j]);
        }
    }

    unique_antinodes.into_iter().map(|(x, y)| (x as usize, y as usize)).collect()

}

fn get_line_points_in_bounds(p1: &(usize, usize), p2: &(usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let dx = p2.1 as i32 - p1.1 as i32;
    let dy = p2.0 as i32 - p1.0 as i32;
    let g = gcd(dx, dy);
    let dxr = dx / g;
    let dyr = dy / g;

    let mut points = vec![];
    let mut x = p1.1 as i32;
    let mut y = p1.0 as i32;
    loop {
        x += dxr;
        y += dyr;
        if x < 0 || x >= max_x as i32 || y < 0 || y >= max_y as i32 {
            break;
        }
        points.push((y as usize, x as usize));
    }
    x = p1.1 as i32;
    y = p1.0 as i32;
    loop {
        x -= dxr;
        y -= dyr;
        if x < 0 || x >= max_x as i32 || y < 0 || y >= max_y as i32 {
            break;
        }
        points.push((x as usize, y as usize));
    }

    points
}

#[aoc(day08, part1)]
fn part1(input: &Input) -> usize {
    let max_x = input.grid.columns;
    let max_y = input.grid.rows;
    let mut unique_antinodes = HashSet::new();
    for (_k, v) in &input.points {
        for i in 0..v.len() {
            for j in i+1..v.len() {
                let pair_antinodes = get_antinodes(&v[i], &v[j], max_y, max_x);
                for antinode in pair_antinodes {
                    unique_antinodes.insert(antinode);
                }
            }
        }
    }
    unique_antinodes.len()
}

#[aoc(day08, part2)]
fn part2(input: &Input) -> usize {
    let max_x = input.grid.columns;
    let max_y = input.grid.rows;
    let mut unique_antinodes = HashMap::new();
    for (_k, v) in &input.points {
        let antinodes = get_antinodes_harmonics(v, max_y, max_x);
        for p in antinodes {
            let entry = unique_antinodes.entry(p).or_insert(0);
            *entry += 1;
        }
    }
    unique_antinodes.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_input() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(input.grid[(1,8)], '0');
    }

    #[test]
    fn test_part1() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_part2() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(part2(&input), 34);
    }
}


