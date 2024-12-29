use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }    
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileType {
    Wall,
    Empty,
    Robot,
    Box,
}

impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '#' => TileType::Wall,
            '.' => TileType::Empty,
            '@' => TileType::Robot,
            'O' => TileType::Box,
            _ => panic!("Invalid tile type"),
        }
    }
}

struct Input {
    grid: Vec<Vec<TileType>>,
    moves: Vec<Direction>,
    start_pos: (usize, usize),
}

#[aoc_generator(day15)]
fn parse_inputs(inp: &str) -> Input {
    let mut grid = Vec::new();
    let mut moves = Vec::new();
    let mut switch = false;
    let mut start_pos = (0, 0);

     for (y, line) in inp.lines().enumerate() {
        let line = line.trim().to_string();
        if line.is_empty() {
           switch = true;
           continue; 
        }
        
        if !switch {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = TileType::from_char(c);
                if tile == TileType::Robot {
                    start_pos = (x, y);
                }
                row.push(tile);
            }
            grid.push(row);
        } else {
            for c in line.chars() {
                moves.push(Direction::from_char(c));
            }
        }
    }
    Input {grid, moves, start_pos}
}

fn move_robot(area_map: &mut Vec<Vec<TileType>>, start_point: (usize, usize), dir: Direction) -> (usize, usize) {
    let (next_x, next_y) = get_next_position(start_point, &dir);

    // Boundary check
    if !is_within_bounds(area_map, next_x, next_y) {
        panic!("Out of bounds");
    }

    match area_map[next_y][next_x] {
        TileType::Empty => {
            area_map[next_y][next_x] = TileType::Robot;
            area_map[start_point.1][start_point.0] = TileType::Empty;
            return (next_x, next_y);
        }
        TileType::Box => {
            if push_box(area_map, (next_x, next_y), dir) {
                area_map[next_y][next_x] = TileType::Robot;
                area_map[start_point.1][start_point.0] = TileType::Empty;
                return (next_x, next_y);
            }
            return start_point;
        }
        _ => {
            return start_point;
        } 
    }
}


fn push_box(area_map: &mut Vec<Vec<TileType>>, start_point: (usize, usize), dir: Direction) -> bool {
    let (next_x, next_y) = get_next_position(start_point, &dir);

    // Boundary check
    if !is_within_bounds(area_map, next_x, next_y) {
        return false;
    }

    match area_map[next_y][next_x] {
        TileType::Empty => {
            area_map[next_y][next_x] = TileType::Box;
            area_map[start_point.1][start_point.0] = TileType::Empty;
            true
        }
        TileType::Box => {
            if push_box(area_map, (next_x, next_y), dir) {
                area_map[next_y][next_x] = TileType::Box;
                area_map[start_point.1][start_point.0] = TileType::Empty;
                true
            } else {
                false
            }
        }
        _ => false,
    }
}


fn get_next_position(start_point: (usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        &Direction::Up => (start_point.0, start_point.1.saturating_sub(1)),
        &Direction::Down => (start_point.0, start_point.1 + 1),
        &Direction::Left => (start_point.0.saturating_sub(1), start_point.1),
        &Direction::Right => (start_point.0 + 1, start_point.1),
    }
}

fn is_within_bounds(area_map: &Vec<Vec<TileType>>, x: usize, y: usize) -> bool {
    y < area_map.len() && x < area_map[0].len()
}

fn calculate_box_coords_sum(area_map: &Vec<Vec<TileType>>) -> i32{
    let mut box_sum = 0;
    
    for (y, row) in area_map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TileType::Box {
                box_sum += x as i32 + y as i32 * 100;
            }
        }
    }
    box_sum
}

fn visualize_map(area_map: &Vec<Vec<TileType>>) {
    for row in area_map {
        for tile in row {
            match tile {
                TileType::Wall => print!("#"),
                TileType::Empty => print!("."),
                TileType::Robot => print!("@"),
                TileType::Box => print!("O"),
            }
        }
        println!();
    }
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i32 {
    let mut area_map = input.grid.clone();
    let mut robot_pos = input.start_pos;

    for dir in &input.moves {
        robot_pos = move_robot(&mut area_map, robot_pos, dir.clone());
    }

    visualize_map(&area_map);
    calculate_box_coords_sum(&area_map)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        let input = parse_inputs(TEST_INPUT);
        assert_eq!(part1(&input), 10092);
    }
}