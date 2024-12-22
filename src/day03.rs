use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn read_inputs(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part_1 (inp: &String) -> i32 {
    let mul: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in mul.captures_iter(inp) {
        sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    sum
}

#[aoc(day3, part2)]
fn part_2 (inp: &String) -> i32 {
    let mul: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_flag: Regex = Regex::new(r"do\(\)").unwrap();
    let dont_flag: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut sum = 0;
    let mut start_pos = 0;
    let mut skip_flag = false;

    while start_pos < inp.len() {
        if skip_flag{
            if let Some(do_cap) = do_flag.find(&inp[start_pos..]) {
                start_pos += do_cap.start();
                skip_flag = false;
            }
            else {
                break;
            }
        } else {
            if let Some(dont_cap) = dont_flag.find(&inp[start_pos..]) {
                let slice = &inp[start_pos..start_pos + dont_cap.start()];
                for cap in mul.captures_iter(slice) {
                    sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
                }
                start_pos += dont_cap.start();
                skip_flag = true;
            } else {
                for cap in mul.captures_iter(&inp[start_pos..]) {
                    sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
                }
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let input = &read_inputs(INPUT1);
        assert_eq!(part_1(&read_inputs(input)), 161);
    }

    #[test]
    fn test_part2() {
        let input = &read_inputs(INPUT2);
        assert_eq!(part_2(&read_inputs(input)), 48);
    }
}

