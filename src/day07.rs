use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day07)]
fn parse_input(input: &str) -> HashMap<i64, Vec<i64>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let key = parts[0].trim().parse::<i64>().unwrap();
        let values = parts[1]
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        map.insert(key, values);
    }
    map
}

fn check_tests(map: &HashMap<i64, Vec<i64>>, allow_concat: bool) -> i64 {
    let mut correct_tests = 0;
    
    for (key, values) in map.iter() {
        if check_value(key, values, allow_concat) {
            correct_tests += key;
        }
    }
    correct_tests
}

fn construct_res(current: i64, index: usize, values: &Vec<i64>, target: i64, allow_concat: bool) -> bool {
    if index == values.len() {
        return current == target;
    }

    let next_value = values[index];

    // Addition
    if construct_res(current + next_value, index + 1, values, target, allow_concat) {
        return true;
    }

    // Multiplication
    if construct_res(current * next_value, index + 1, values, target, allow_concat) {
        return true;
    }

    // Concatenation:
    if allow_concat {
        if let Some(concat_value) = Some(current.to_string() + &next_value.to_string())
            .and_then(|concat_str| concat_str.parse::<i64>().ok())
        {
            if construct_res(concat_value, index + 1, values, target, allow_concat) {
                return true;
            }
        }
    }

    false
}

fn check_value(key: &i64, values: &Vec<i64>, allow_concat: bool) -> bool {
    if values.is_empty() {
        return false;
    }
    construct_res(values[0], 1, values, *key, allow_concat)
}

#[aoc(day07, part1)]
fn part1(map: &HashMap<i64, Vec<i64>>) -> i64 {
    check_tests(map, false)
}

#[aoc(day07, part2)]
fn part2(map: &HashMap<i64, Vec<i64>>) -> i64 {
    check_tests(map, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input[&3267], vec![81, 40, 27]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 11387);
    }
}