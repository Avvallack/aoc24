use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

struct Lock {
    pins: Vec<u16>,
}
struct Input {
    keys: Vec<Lock>,
    locks: Vec<Lock>,
}

#[aoc_generator(day25)]
fn read_input(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut key_flag;
    let chunks = input.split("\n\n").collect::<Vec<&str>>();
    for chunk in chunks {
        let lines = chunk.lines().collect::<Vec<&str>>();
        if lines[0].chars().all(|c| c == '.') {
            key_flag = true;
        }
        else {
            key_flag = false;
        }
        let mut pins = vec![0; lines[0].len()];
        for line in lines[1..lines.len()-1].iter() {
            
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    pins[i] += 1;
                }
            }
        }
        if key_flag {
            keys.push(Lock { pins });
        }
        else {
            locks.push(Lock { pins });
        }        
    }  

    Input { keys, locks }
}

fn check_pins(key: &Lock, lock: &Lock) -> bool {
    for (k, l) in key.pins.iter().zip(lock.pins.iter()) {
        if k + l > 5 {
            return false;
        }
    }
    true
}

fn check_all_locks(inp: &Input) -> usize {
    let mut count = 0;
    for (key, lock) in iproduct!(inp.keys.iter(), inp.locks.iter()) {
        if check_pins(key, lock) {
            count += 1;
        }
    }
    count
}

#[aoc(day25, part1)]
fn part1(inp: &Input) -> usize {
    check_all_locks(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_parse() {
        let input = read_input(TEST_INPUT);
        let key = &input.keys[0];
        assert_eq!(key.pins, vec![5, 0, 2, 1, 3]);
    }

    #[test]
    fn test_part1() {
        let input = read_input(TEST_INPUT);
        assert_eq!(part1(&input), 3);
    }

}