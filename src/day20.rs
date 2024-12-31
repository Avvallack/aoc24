use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{Matrix, Weights};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point (usize, usize);
struct Input {
    field: Matrix<char>,
    start_pos: Point,
    finish_pos: Point,
}

#[aoc_generator(day20)]
fn read_inputs(inp: &str) -> Input {
    let mut map = Vec::new();
    let mut start_pos = Point(0, 0);
    let mut finish_pos = Point(0, 0);
    for (y, line) in inp.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start_pos = Point(x, y);
            }
            if c == 'E' {
                finish_pos = Point(x, y);
            }
            row.push(c);
        }
        map.push(row);
    }
    let field = Matrix::from_rows(map).expect("Invalid matrix");
    Input { field, start_pos, finish_pos }
}

fn get_neighbors(field: &Matrix<char>, x: usize, y: usize) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < field.columns() as isize && ny >= 0 && ny < field.rows() as isize {
            let nxu = nx as usize;
            let nyu = ny as usize;
            if field[(nyu, nxu)] != '#' {
                neighbors.push(Point(nxu, nyu));
            }
        }
    }
    neighbors
}

fn bfs_shortest_path(field: &Matrix<char>, start: Point, goal: Point) -> Option<Vec<Point>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut current = goal;
            while current != start {
                path.push(current);
                current = parent[&current];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in get_neighbors(field, current.0, current.1) {
            if visited.insert(neighbor) {
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn find_all_cheats(field: &Matrix<char>, path: &Vec<Point>, max_steps: usize) -> HashMap<isize, isize> {
    let mut point_to_index = HashMap::new();
    for (idx, &p) in path.iter().enumerate() {
        point_to_index.insert(p, idx);
    }
    let directions = [(0isize,1isize),(1,0),(0,-1),(-1,0)];
    let field_height = field.rows();
    let field_width = field.columns();

    let mut cheat_savings = HashMap::new();
    let mut cheats = HashSet::new();

    for (i, &start_point) in path.iter().enumerate() {
        let (sx, sy) = (start_point.0, start_point.1);

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy, 0));

        while let Some((x, y, steps)) = queue.pop_front() {
            if steps >0 && steps <= max_steps && field[(y, x)] != '#' {
                if let Some(&j) = point_to_index.get(&Point(x,y)) {
                    if j > i {
                        let saving = (j as isize - i as isize) - (steps as isize);
                        if cheats.insert((i,j)) {
                            *cheat_savings.entry(saving).or_insert(0) += 1;
                        }
                    }
                }
            }
            if steps < max_steps {
                for (dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || ny < 0 || nx >= field_width as isize || ny >= field_height as isize {
                        continue;
                    }
                    let nxu = nx as usize;
                    let nyu = ny as usize;

                    if !visited.contains(&(nyu, nxu, steps+1)) {
                        visited.insert((nyu, nxu, steps+1));
                        queue.push_back((nxu, nyu, steps+1));
                    }
                }
            }
            
        }
    }
    cheat_savings
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> isize {
    let pth = bfs_shortest_path(&input.field, input.start_pos, input.finish_pos).unwrap();
    let cheat_savings = find_all_cheats(&input.field, &pth, 2);
    cheat_savings.iter()
    .filter(|(&saving, _)| saving >= 100)
    .map(|(_, &count)| count)
    .sum()
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> isize {
    let pth = bfs_shortest_path(&input.field, input.start_pos, input.finish_pos).unwrap();
    let cheat_savings = find_all_cheats(&input.field, &pth, 20);
    cheat_savings.iter()
    .filter(|(&saving, _)| saving >= 100)
    .map(|(_, &count)| count)
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        let input = read_inputs(TEST_INPUT);
        let pth = bfs_shortest_path(&input.field, input.start_pos, input.finish_pos).unwrap();
        let cheats = find_all_cheats(&input.field, &pth, 2);
        print!("{:?}", cheats);
    }
    
    #[test]
    fn test_part2() {
        let input = read_inputs(TEST_INPUT);
        let pth = bfs_shortest_path(&input.field, input.start_pos, input.finish_pos).unwrap();
        let cheats = find_all_cheats(&input.field, &pth, 20);
        print!("{:?}", cheats[&76]);
    }
}