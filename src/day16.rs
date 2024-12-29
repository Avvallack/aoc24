use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use aoc_runner_derive::{aoc, aoc_generator};

use pathfinding::prelude::Matrix;

#[derive(Debug)]
struct Maze {
    grid: Matrix<char>,
    start_pos: (usize, usize),
    stop_pos: (usize, usize),
    width: usize,
    height: usize,
}

impl Maze {
    fn is_open(&self, x: usize, y: usize) -> bool {
        self.grid[(y, x)] != '#'
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, Direction)> {
        let dirs = [(0,-1),(1,0),(0,1),(-1,0)];
        let mut neigh = Vec::new();
        for &(dx,dy) in &dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                let nxu = nx as usize;
                let nyu = ny as usize;
                if self.is_open(nxu, nyu) {
                    neigh.push((nxu, nyu, Direction::from_dxy(dx,dy)));
                }
            }
        }
        neigh
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn list_all() -> [Direction;4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    fn from_dxy(dx: isize, dy: isize) -> Direction {
        match (dx, dy) {
            (0, -1) => Direction::North,
            (1, 0) => Direction::East,
            (0, 1) => Direction::South,
            (-1,0) => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }

    fn turn_cost(self, target: Direction) -> u32 {
        if self == target {
            1
        } else {
            1001
        }
    }

    fn to_index(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    x: usize,
    y: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

#[aoc_generator(day16)]
fn read_inputs(input: &str) -> Maze {
    let mut rows = Vec::new();
    let mut start_pos = (0, 0);
    let mut stop_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start_pos = (x, y);
            } else if c == 'E' {
                stop_pos = (x, y);
            }
        }
        rows.push(line.chars().collect::<Vec<_>>());
    }


    let grid = Matrix::from_rows(rows).expect("Invalid matrix");
    let width = grid.columns;
    let height = grid.rows;
    Maze { grid, start_pos, stop_pos , width, height }
}

fn index_of(x: usize, y: usize, dir: Direction, width: usize) -> usize {
    let dir_i = dir.to_index();
    (y * width + x) * 4 + dir_i
}

fn reconstruct_all_paths(
    end_states: &[(usize, usize, Direction)],
    start_state: (usize, usize, Direction),
    parents: &Vec<Vec<(usize, usize, Direction)>>,
    width: usize
) -> Vec<Vec<(usize,usize,Direction)>> {
    let mut results = Vec::new();
    for &end_st in end_states {
        dfs_all_paths(end_st, start_state, parents, &mut Vec::new(), &mut results, width);
    }
    results
}

fn dfs_all_paths(
    current: (usize,usize,Direction),
    start: (usize,usize,Direction),
    parents: &Vec<Vec<(usize,usize,Direction)>>,
    path: &mut Vec<(usize,usize,Direction)>,
    results: &mut Vec<Vec<(usize,usize,Direction)>>,
    width: usize
) {
    let idx = index_of(current.0, current.1, current.2, width);
    path.push(current);
    if current == start {
        path.reverse();
        results.push(path.clone());
        path.reverse();
    } else {
        for &p in &parents[idx] {
            dfs_all_paths(p, start, parents, path, results, width);
        }
    }
    path.pop();
}

fn solve(maze: &Maze) -> (u32, usize){
    let (sx, sy) = maze.start_pos;
    let (ex, ey) = maze.stop_pos;

    let width = maze.width;
    let height = maze.height;
    // dist array: width * height * 4 directions
    let mut dist = vec![u32::MAX; width * height * 4];
    let mut parents: Vec<Vec<(usize,usize,Direction)>> = vec![Vec::new(); width*height*4];

    let start_dir = Direction::East; // start facing East
    let start_idx = index_of(sx, sy, start_dir, width);
    dist[start_idx] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, x: sx, y: sy, dir: start_dir });

    while let Some(State { cost, x, y, dir }) = heap.pop() {
        let idx = index_of(x,y,dir,width);
        if cost > dist[idx] {
            continue;
        }
        for (nx, ny, ndir) in maze.neighbors(x,y) {
            let step_cost =  ndir.turn_cost(dir);
            let new_cost = cost + step_cost;
            let nidx = index_of(nx, ny, ndir, width);
            if new_cost < dist[nidx] {
                dist[nidx] = new_cost;
                parents[nidx].clear();
                parents[nidx].push((x,y,dir));
                heap.push(State { cost: new_cost, x: nx, y: ny, dir: ndir });
            } else if new_cost == dist[nidx] {
                // another equally good predecessor
                parents[nidx].push((x,y,dir));
            }
        }
    }

    let mut best_cost = u32::MAX;
    let mut end_states = Vec::new();
    for d in Direction::list_all() {
        let eidx = index_of(ex, ey,d, width);
        if dist[eidx] < best_cost {
            best_cost = dist[eidx];
            end_states.clear();
            end_states.push((ex,ey,d));
        } else if dist[eidx] == best_cost {
            end_states.push((ex,ey,d));
        }
    }
    let start_state = (sx, sy, start_dir);
    let all_paths = reconstruct_all_paths(&end_states, start_state, &parents, width);

    let mut unique_tiles = HashSet::new();
    for path in &all_paths {
        for &(x,y,_) in path {
            unique_tiles.insert((x,y));
        }
    }

    (best_cost, unique_tiles.len())

}

#[aoc(day16, part1)]
fn part1(maze: &Maze) -> u32 {
    solve(maze).0
}

#[aoc(day16, part2)]
fn part2(maze: &Maze) -> usize {
    solve(maze).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_parse() {
        let input = read_inputs(TEST_INPUT);
        println!("{:?}", input.grid);
    }

    #[test]
    fn test_part1() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn test_part2() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(part2(&input), 45);
    }
}
