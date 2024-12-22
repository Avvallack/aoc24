use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    l1: Vec<i32>,
    l2: Vec<i32>,
}

#[aoc_generator(day01)]
fn read_inputs(input: &str) -> Input {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let num1 = split[0].parse::<i32>().unwrap();
        let num2 = split[1].parse::<i32>().unwrap();
        l1.push(num1);
        l2.push(num2);
    }
    l1.sort();
    l2.sort();
    Input{l1, l2}
}


#[aoc(day01, part1)]
fn part_1 (inp: &Input) -> i32 {
    let l1= &inp.l1;
    let l2 = &inp.l2;
    let distance: i32 = l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum();
    distance
}

fn count_elements(list1: &Vec<i32>, list2: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counter = HashMap::new();
    let mut counts_in_list2 = HashMap::new();
    for &item in list2 {
        *counts_in_list2.entry(item).or_insert(0) += 1;
    }
    for &item in list1 {
        let count = counts_in_list2.get(&item).unwrap_or(&0);
        let c = counter.entry(item).or_insert(0);
        *c += *count;
    }

    counter
}

#[aoc(day01, part2)]
fn part_2 (inp: &Input) -> i32 {
    let l1= &inp.l1;
    let l2 = &inp.l2;
    let counter = count_elements(l1, l2);
    counter.iter().map(|(&key, &value)| key * value).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let input = read_inputs(EXAMPLE);
        assert_eq!(part_1(&input), 11);
        
    }

    #[test]
    fn test_part2() {
        let input = read_inputs(EXAMPLE);
        assert_eq!(part_2(&input), 31);
    }


}