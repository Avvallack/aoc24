use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Wall,
    Empty,
    Robot,
    Box,
    BoxRight,
    BoxLeft,
}

impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '#' => TileType::Wall,
            '.' => TileType::Empty,
            '@' => TileType::Robot,
            'O' => TileType::Box,
            ']' => TileType::BoxRight,
            '[' => TileType::BoxLeft,
            _ => panic!("Invalid tile type"),
        }
    }
}

struct Input {
    grid: Vec<Vec<TileType>>,
    moves: Vec<Direction>,
    start_pos: (usize, usize),
}

#[aoc_generator(day15, part1)]
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

fn scale_up(c: char) -> Vec<char> {
    match c {
        '#' => vec!['#', '#'],   // Wall doubles horizontally
        'O' => vec!['[', ']'],   // Box becomes "[ ]"
        '.' => vec!['.', '.'],   // Floor doubles horizontally
        '@' => vec!['@', '.'],   // Robot becomes "@."
        // anything else (like newlines, etc.) just return as is:
        _ => vec![c],
    }
}

#[aoc_generator(day15, part2)]
fn parse_inputs_scaleup(inp: &str) -> Input{
    let (warehouse, path) = inp.split_once("\n\n")
        .expect("Expected empty line separating warehouse from path");
    let mut grid = Vec::new();
    let mut start_pos = (0, 0);

    for (y, line) in warehouse.lines().enumerate() {
        let scaled_chars: Vec<char> = line
            .chars()
            .flat_map(scale_up)
            .collect();
        let mut row = Vec::new();
        for (x, c) in scaled_chars.iter().enumerate() {
            let tile = TileType::from_char(*c);
            if tile == TileType::Robot {
                start_pos = (x, y);
            }
            row.push(tile);
        }
        grid.push(row);
    }
    let moves: Vec<Direction> = path
    .lines()
    .flat_map(|line| line.chars())  // flatten each line
    .map(Direction::from_char)
    .collect();

    Input { grid, moves, start_pos }
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
        TileType::BoxLeft | TileType::BoxRight => {

            match dir {
                Direction::Up | Direction::Down => {
                    if v_push_box(area_map, (next_x, next_y), &dir) {
                        area_map[next_y][next_x] = TileType::Robot;
                        area_map[start_point.1][start_point.0] = TileType::Empty;
                        return (next_x, next_y);
                    } else {
                        return start_point;
                    }
                }
                Direction::Left | Direction::Right => {
                    if push_box(area_map, (next_x, next_y), dir) {
                        area_map[next_y][next_x] = TileType::Robot;
                        area_map[start_point.1][start_point.0] = TileType::Empty;
                        return (next_x, next_y);
                    } else {
                        return start_point;
                    }

                }
            }
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
            area_map[next_y][next_x] =  area_map[start_point.1][start_point.0];
            area_map[start_point.1][start_point.0] = TileType::Empty;
            true
        }
        TileType::Box | TileType::BoxLeft | TileType::BoxRight => {
            if push_box(area_map, (next_x, next_y), dir) {
                area_map[next_y][next_x] = area_map[start_point.1][start_point.0];
                area_map[start_point.1][start_point.0] = TileType::Empty;
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn v_push_box(area_map: &mut Vec<Vec<TileType>>, start: (usize, usize), dir: &Direction) -> bool {
    if !matches!(area_map[start.1][start.0], TileType::BoxLeft | TileType::BoxRight) {
        return false;
    }
    if !can_v_push_box(area_map, start, dir) {
        return false;
    }
    do_v_push_box(area_map, start, dir);
    true
}

fn can_v_push_box(area_map: &Vec<Vec<TileType>>, start: (usize, usize), dir: &Direction) -> bool {
    use TileType::*;

    let (bx, by) = start;
    let is_left  = matches!(area_map[by][bx], BoxLeft);
    let is_right = matches!(area_map[by][bx], BoxRight);
    if !is_left && !is_right {
        return false;
    }

    let partner_x = if is_left { bx + 1 } else { bx - 1 };
    let partner_y = by;

    if is_left && area_map[partner_y][partner_x] != BoxRight { return false; }
    if is_right && area_map[partner_y][partner_x] != BoxLeft { return false; }

    let (nx1, ny1) = get_next_position((bx, by), dir);
    let (nx2, ny2) = get_next_position((partner_x, partner_y), dir);

    if !is_within_bounds(area_map, nx1, ny1) || !is_within_bounds(area_map, nx2, ny2) {
        return false;
    }

    match area_map[ny1][nx1] {
        Empty => {}
        BoxLeft | BoxRight => {
            if !can_v_push_box(area_map, (nx1, ny1), dir) {
                return false;
            }
        }
        _ => return false,
    }

    match area_map[ny2][nx2] {
        Empty => {}
        BoxLeft | BoxRight => {
            if !can_v_push_box(area_map, (nx2, ny2), dir) {
                return false;
            }
        }
        _ => return false,
    }

    true
}

fn do_v_push_box(area_map: &mut Vec<Vec<TileType>>, start: (usize, usize), dir: &Direction) {
    use TileType::*;

    let (bx, by) = start;
    let is_left  = matches!(area_map[by][bx], BoxLeft);
    let partner_x = if is_left { bx + 1 } else { bx - 1 };
    let partner_y = by;
    let (nx1, ny1) = get_next_position((bx, by), dir);
    let (nx2, ny2) = get_next_position((partner_x, partner_y), dir);
    if matches!(area_map[ny1][nx1], BoxLeft | BoxRight) {
        do_v_push_box(area_map, (nx1, ny1), dir);
    }
    if matches!(area_map[ny2][nx2], BoxLeft | BoxRight) {
        do_v_push_box(area_map, (nx2, ny2), dir);
    }

    let (lx, ly)    = if is_left { (bx, by) } else { (partner_x, partner_y) };
    let (rx, ry)    = if is_left { (partner_x, partner_y) } else { (bx, by) };
    let (lnx, lny)  = get_next_position((lx, ly), dir);
    let (rnx, rny)  = get_next_position((rx, ry), dir);

    area_map[ly][lx]   = Empty;
    area_map[ry][rx]   = Empty;
    area_map[lny][lnx] = BoxLeft;
    area_map[rny][rnx] = BoxRight;
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
            if *tile == TileType::Box || *tile == TileType::BoxLeft {
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
                TileType::BoxRight => print!("]"),
                TileType::BoxLeft => print!("["),
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

#[aoc(day15, part2)]
fn part2(input: &Input) -> i32 {
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

const TEST_INPUT2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<<^^^^";

    #[test]
    fn test_part1() {
        let input = parse_inputs(TEST_INPUT);
        assert_eq!(part1(&input), 10092);
    }

    #[test]
    fn test_part2() {
        let input = parse_inputs_scaleup(TEST_INPUT);
        assert_eq!(part2(&input), 9021);
    }


    #[test]
    fn test_scale(){
        let inp = parse_inputs_scaleup(TEST_INPUT2);
        part2(&inp);
    }
}
