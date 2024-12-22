use aoc_runner_derive::{aoc, aoc_generator};

const XMAS: &str = "XMAS";

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),  // Up
    (1, 0),   // Down
    (0, -1),  // Left
    (0, 1),   // Right
    (-1, -1), // Up-Left
    (-1, 1),  // Up-Right
    (1, -1),  // Down-Left
    (1, 1),   // Down-Right
];

#[aoc_generator(day4)]
fn parse_inputs(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line| line.chars().collect()).collect()
        
}

fn count_xmas(inp: &Vec<Vec<char>>, ) -> usize {
    let rows = inp.len();
    let cols = inp[0].len();
    let mut count = 0;
    let word_chars = XMAS.chars().collect::<Vec<char>>();
    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in DIRECTIONS {
                if can_match(&inp, &word_chars, i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn can_match(grid: &[Vec<char>], word: &[char], start_r: usize, start_c: usize, dx: isize, dy: isize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let word_len = word.len() as isize;

    let mut r = start_r as isize;
    let mut c = start_c as isize;

    for i in 0..word_len {
        // Check boundary
        if r < 0 || r >= rows || c < 0 || c >= cols {
            return false; 
        }

        // Check character match
        if grid[r as usize][c as usize] != word[i as usize] {
            return false;
        }

        // Move to the next character in the given direction
        r += dx;
        c += dy;
    }

    true
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if grid[i][j] == 'A' {
                // Check the diagonals
                let top_left = grid[i-1][j-1];
                let bottom_right = grid[i+1][j+1];
                let top_right = grid[i-1][j+1];
                let bottom_left = grid[i+1][j-1];

                let diag1_valid = (top_left == 'M' && bottom_right == 'S') 
                                || (top_left == 'S' && bottom_right == 'M');


                let diag2_valid = (top_right == 'M' && bottom_left == 'S')
                                || (top_right == 'S' && bottom_left == 'M');

                if diag1_valid && diag2_valid {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day4, part1)]
fn part1(inp: &Vec<Vec<char>>) -> usize {
    count_xmas(inp)
}

#[aoc(day4, part2)]
fn part2(inp: &Vec<Vec<char>>) -> usize {
    count_x_mas(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let inp = parse_inputs(TEST_INPUT);
        assert_eq!(part1(&inp), 18);
    }

    #[test]
    fn test_part2() {
        let inp = parse_inputs(TEST_INPUT);
        assert_eq!(part2(&inp), 9);
    }
}