use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub struct Input {
    tokens: Vec<String>,
    patterns: Vec<String>,
}

#[aoc_generator(day19)]
fn read_inputs(input: &str) -> Option<Input> {
    let mut iter = input.split("\n\n");
    let first = iter.next().unwrap().split(", ").map(|s| s.to_string()).collect();
    let second = iter.next().unwrap().lines().map(|x| x.to_string()).collect();
    Some(Input{tokens: first, 
            patterns: second})
}

fn count_tokenization(text: &str, patterns: &Vec<String>) -> u64 {
    let n = text.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..n + 1 {
        for j in 0..=i {
            if dp[j] > 0 && patterns.contains(&text[j..i].to_string()) {
                dp[i] += dp[j];

            }
        }
    }
    dp[n]
}

fn can_tokenize(text: &str, patterns: &Vec<String>) -> bool {
    let n = text.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..n + 1 {
        for j in 0..=i {
            if dp[j] && patterns.contains(&text[j..i].to_string()) {
                dp[i] = true;
            }
        }
    }
    dp[n]
}

#[aoc(day19, part1)]
fn part_1 (inp: &Input) -> u64 {
    let mut possible  = Vec::new();
    for stripe in &inp.patterns {
        if can_tokenize(&stripe, &inp.tokens) {
            possible.push(stripe);
        }
    }

    possible.len() as u64
}

#[aoc(day19, part2)]
fn part_2 (inp: &Input) -> u64 {
    let mut possible  = Vec::new();
    for stripe in &inp.patterns {
        if can_tokenize(&stripe, &inp.tokens) {
            possible.push(stripe);
        }
    }
    let mut sums = Vec::new();
    for towel in possible {
        let sum = count_tokenization(&towel, &inp.tokens);
        sums.push(sum);
    }
    sums.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br\n\
                              \n\
                              brwrr\n\
                              bggr\n\
                              gbbr\n\
                              rrbgbr\n\
                              ubwu\n\
                              bwurrg\n\
                              brgr\n\
                              bbrgwb";

    #[test]
    fn test_p1() {
        let inp = read_inputs(TEST_INPUT).expect("valid input");
        let res = part_1(&inp);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_p2() {
        let inp  = read_inputs(TEST_INPUT).expect("valid input");
        let res = part_2(&inp);
        assert_eq!(res, 16);
    }
}