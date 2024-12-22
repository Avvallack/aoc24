use std::{collections::HashMap, iter::zip, ops::BitXor};
use memoize::memoize;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<i128> {
    input
        .split("\n")
        .map(|l| l.parse().unwrap())
        .collect()
}

#[memoize]
fn get_next_secret_number(secret_number: i128) -> i128 {
    
    // step 1
    let mut result = secret_number * 64;
    let mut new_secret = secret_number;
    new_secret = new_secret.bitxor(result);
    new_secret %= 16777216;

    // step 2
    result = new_secret / 32;
    new_secret = new_secret.bitxor(result);
    new_secret %= 16777216;

    // step 3
    result = new_secret * 2048;
    new_secret = new_secret.bitxor(result);
    new_secret %= 16777216;

    new_secret
    }


fn get_i_iterations(secret_number: i128, i: isize) -> i128 {
    let mut new_secret = secret_number;
    for _ in 0..i {
        new_secret = get_next_secret_number(new_secret);
    }
    new_secret
}

fn get_i_iteration_last_digit(secret_number: i128, i: isize) -> Vec<i32> {
    let mut new_secret = secret_number;
    let mut last_digits = Vec::new();
    last_digits.push((new_secret % 10) as i32);
    for _ in 0..i {
        new_secret = get_next_secret_number(new_secret);
        last_digits.push((new_secret % 10) as i32);

    }
    last_digits
}

fn get_diff(last_digits: &Vec<i32>) -> Vec<i32> {
    last_digits.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}


fn get_last_digits(sec_nums: &Vec<i128>, i: isize) -> Vec<Vec<i32>> {
    sec_nums.iter().map(|sn| get_i_iteration_last_digit(*sn, i)).collect::<Vec<_>>()
}

fn get_diffs(last_digits: &Vec<Vec<i32>>, i: isize) -> Vec<Vec<i32>> {
    last_digits.iter().map(|ld| get_diff(ld)).collect::<Vec<_>>()
}
    

fn get_best_sequence(secret_numbers: &Vec<i128>) -> (HashMap<Vec<i32>, i32>, i32) {
    let mut best_changes =HashMap::new();
    
    let ld = get_last_digits(secret_numbers, 2000);
    let diffs = get_diffs(&ld, 2000);
    for (digits, diff) in zip(ld, diffs) {
        let mut visited_seqs = HashMap::new();
        for i in 0..diff.len() - 4 {
            let seq = diff[i..i+4].to_vec();
            if visited_seqs.contains_key(&seq) {
                continue;
            } else {
                visited_seqs.insert(seq.clone(), 1);
                let score = digits[i+4];
                let count = best_changes.entry(seq).or_insert(0);
                *count += score;
            }
        }
    }
    let max_value = *best_changes.values().max().unwrap();
    (best_changes, max_value)
}

#[aoc(day22, part1)]
fn part1(input: &Vec<i128>) -> i128 {
    let mut final_sum = 0;
    for it in input {
        final_sum += get_i_iterations(*it, 2000);
    }
    final_sum
}

#[aoc(day22, part2)]
fn part2(input: &Vec<i128>) -> i32 {
    let (_, max_value) = get_best_sequence(input);
    max_value
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1
10
100
2024";

    const SAMPLE2: &str = "1
2
3
2024";

    #[test]
    fn sample1() {
        assert_eq!(&input_generator(SAMPLE), &vec![1, 10, 100, 2024]);
    }

    #[test]
    fn sample2() {
        assert_eq!(get_next_secret_number(123), 15887950);
    }

    #[test]
    fn sample3() {
        assert_eq!(get_i_iterations(2024, 2000), 8667524);
    }

    #[test]
    fn sample4() {
        assert_eq!(part1(&input_generator(SAMPLE)), 37327623);
    }

    #[test]
    fn sample5() {
        assert_eq!(get_i_iteration_last_digit(123, 4), vec![3, 0, 6, 5, 4]);
    }

    #[test]
    fn sample6() {
        let inp = input_generator(SAMPLE2);
        let (_, score) = get_best_sequence(&inp);
        assert_eq!(score, 23);
    }


}
