use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    field: [[i32; 71]; 71],
    bites: Vec<(usize, usize)>,
}

#[aoc_generator(day18)]
fn parse_input(inp: &str) -> Input {
    let mut bites = Vec::new();
    let mut field = [[0; 71]; 71];
        for (i, line) in inp.lines().enumerate() {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].parse::<usize>().unwrap();
            let y = parts[1].parse::<usize>().unwrap();
            bites.push((x, y));
            if i < 1024{
                field[y][x] = i32::MAX;
            }
        }

    Input { field, bites }
}

fn visualise_field(field: &[[i32; 71]; 71]) {
    for row in field.iter() {
        for cell in row.iter() {
            if *cell == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn get_neighbors(field: &[[i32; 71]; 71], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < 71 && ny >= 0 && ny < 71 {
            let nxu = nx as usize;
            let nyu = ny as usize;
            if field[nyu][nxu] == 0 {
                neighbors.push((nxu, nyu));
            }
        }
    }
    neighbors
}

fn bfs_shortest_path(field: &[[i32; 71]; 71]) -> Option<Vec<(usize, usize)>> {
    let start = (0, 0);
    let goal = (70, 70);

    let mut visited = [[false; 71]; 71];
    let mut parent = [[None; 71]; 71];

    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start.1][start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == goal {
            let mut path = Vec::new();
            let mut current = Some((x, y));
            while let Some((cx, cy)) = current {
                path.push((cx, cy));
                current = parent[cy][cx];
            }
            path.reverse();
            return Some(path);
        }

        for (nx, ny) in get_neighbors(field, x, y) {
            if !visited[ny][nx] {
                visited[ny][nx] = true;
                parent[ny][nx] = Some((x, y));
                queue.push_back((nx, ny));
            }
        }
    }

    None
}

fn visualise_path(field: &[[i32; 71]; 71], path: &[(usize, usize)]) {
    use std::collections::HashSet;
    let path_set: HashSet<(usize, usize)> = path.iter().cloned().collect();
    for y in 0..71 {
        for x in 0..71 {
            if path_set.contains(&(x, y)) {
                print!("O");
            } else if field[y][x] == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> Option<usize> {
    let path = bfs_shortest_path(&input.field);
    match path {
        Some(p) => {
            visualise_path(&input.field, &p);
            Some(p.len() - 1)
        }
        None => None,
    }
    
}

#[aoc(day18, part2)]
fn part2(inp: &Input) -> String {
    let mut max_bites = 1024;
    let bites = &inp.bites;
    let mut field = inp.field;
    for i in max_bites..bites.len() {
        field[bites[i].1][bites[i].0] = i32::MAX;
        if let Some(_) = bfs_shortest_path(&field) {
            max_bites += 1;
        } else {
            return format!("{},{}", bites[i].0, bites[i].1)
        }
    }
    "None".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        visualise_field(&input.field);
        println!();
        assert_eq!(part1(&input), Some(146));
    }
}