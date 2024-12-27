use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day11)]
fn read_inputs(input: &str) -> Vec<i128> {
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

#[memoize]
fn get_span(stone: i128, num_blinks: i128) -> i128 {
    if num_blinks == 0 {
        return 1;
    }
    if stone == 0 {
        return get_span(1, num_blinks - 1);
    } else if stone.to_string().len() % 2 == 0 {
        let n = stone.to_string().len();
        let left = stone.to_string()[..n / 2].parse().unwrap();
        let right= stone.to_string()[n / 2..].parse().unwrap();
        return get_span(left, num_blinks - 1) + get_span(right, num_blinks - 1);
    } else {
        return get_span(2024 * stone, num_blinks - 1);
    }
}

#[aoc(day11, part1)]
fn part1(stones: &Vec<i128>) -> i128 {
    stones.iter().map(|&stone| get_span(stone, 25)).sum()
}

#[aoc(day11, part2)]
fn part2(stones: &Vec<i128>) -> i128 {
    stones.iter().map(|&stone| get_span(stone, 75)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        let inpt = read_inputs(TEST_INPUT);
        assert_eq!(part1(&inpt), 55312);
    }

}