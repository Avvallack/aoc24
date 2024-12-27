use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point (usize, usize);

struct Input{
    geomap: Vec<Vec<u32>>,
    zero_coords: Vec<Point>,
}

#[aoc_generator(day10)]
fn read_inputs(inp: &str) -> Input{
    let mut geomap: Vec<Vec<u32>> = vec![];
    let mut zero_coords: Vec<Point> = vec![];

    for (y, line) in inp.lines().enumerate() {
        let mut row: Vec<u32> = vec![];
        for (x, c) in line.chars().enumerate() {
            let c = c.to_digit(10).unwrap();
            row.push(c);
            if c == 0 {
                zero_coords.push(Point(x, y));
            }
        }
        geomap.push(row);
    }
    Input { geomap, zero_coords }
}

fn check_valid_path(p: &Vec<Point>, value: u32, geomap: &Vec<Vec<u32>>,) -> Vec<Point> {
    if value == 9 {
        return p.to_owned().to_vec();
    }
    else {
        let mut valid_path: Vec<Point> = vec![];
        for point in p{
            let x = point.0;
            let y = point.1;
            if x > 0 && geomap[y][x-1] == value + 1 {
                valid_path.push(Point(x-1, y));
            }
            if x < geomap[0].len() - 1 && geomap[y][x+1] == value + 1 {
                valid_path.push(Point(x+1, y));
            }
            if y > 0 && geomap[y-1][x] == value + 1 {
                valid_path.push(Point(x, y-1));
            }
            if y < geomap.len() - 1 && geomap[y+1][x] == value + 1 {
                valid_path.push(Point(x, y+1));
            }
       }
         return check_valid_path(&valid_path, value + 1, geomap);
    }
}


fn calculate_score_and_rating(inp: &Input) -> (usize, usize){
    let geo_map = &inp.geomap;
    let zero_coords = &inp.zero_coords;

    let mut total_score = 0;
    let mut total_rating = 0;
    for &start in zero_coords {
        let endpoints = check_valid_path(&vec![start], 0, &geo_map);
        total_rating += endpoints.len();
        let unique_endpoints: HashSet<Point> = endpoints.into_iter().collect();
        total_score += unique_endpoints.len();
        
    }

    (total_score, total_rating)
}

#[aoc(day10, part1)]
fn part1(inp: &Input) -> usize {
    calculate_score_and_rating(inp).0
}

#[aoc(day10, part2)]
fn part2(inp: &Input) -> usize {
    calculate_score_and_rating(inp).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let inp = read_inputs(TEST_INPUT);
        let res = part1(&inp);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_part2() {
        let inp = read_inputs(TEST_INPUT);
        let res = part2(&inp);
        assert_eq!(res, 81);
    }
}
