use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn read_inputs(input: &str) -> Vec<Vec<i32>> {
    let mut inp = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace()
                        .map(|x| x.parse::<i32>())
                        .collect::<Result<Vec<i32>, _>>()
                        .unwrap();
        inp.push(parts);
        
    }
    inp
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();
    diffs.iter().all(|&dif| (1..=3).contains(&dif)) || diffs.iter().all(|&dif| (-3..=-1).contains(&dif))
}

#[aoc(day2, part1)]
fn part_1 (inp: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for levels in inp {
        if is_safe(levels) {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
fn part_2 (inp: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for levels in inp {
        if is_safe(levels) {
            count += 1;
        }
        else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe(&new_levels) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_read_input() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(read_inputs(input), expected);
    }
    

    #[test]
    fn test_part1() { 
        let input = read_inputs(INPUT);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part2() { 
        let input = read_inputs(INPUT);
        assert_eq!(part_2(&input), 4);
    }
}