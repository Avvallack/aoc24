use std::{collections::{HashMap, HashSet, VecDeque}, usize};

use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day21)]
fn parse_inputs(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

struct DirectionMap {
    map: HashMap<(isize, isize), char>,
}

impl DirectionMap {
    fn new() -> Self {
        DirectionMap {
            map: Self::direction_map(),
        }
    }

    fn direction_map() -> HashMap<(isize, isize), char> {
        let mut map = HashMap::new();
        map.insert((0, 1), '>');
        map.insert((1, 0), 'v');
        map.insert((0, -1), '<');
        map.insert((-1, 0), '^');
        map
    }
    
}
struct NumericPad {
    pad: HashMap<char, (usize, usize)>,
    reverse_map: HashMap<(usize, usize), char>,
    current_pos: (usize, usize)
}

impl NumericPad {
    fn new() -> Self {
        NumericPad {
            pad: Self::char_map(),
            reverse_map: Self::reverse_map(),
            current_pos: (3, 2)
        }
    }

    fn char_map() -> HashMap<char, (usize, usize)> {
        let mut map = HashMap::new();
        map.insert('A', (3, 2));
        map.insert('0', (3, 1));
        map.insert('1', (2, 0));
        map.insert('2', (2, 1));
        map.insert('3', (2, 2));
        map.insert('4', (1, 0));
        map.insert('5', (1, 1));
        map.insert('6', (1, 2));
        map.insert('7', (0, 0));
        map.insert('8', (0, 1));
        map.insert('9', (0, 2));
        map
    }

    fn reverse_map() -> HashMap<(usize, usize), char> {
        let mut map = HashMap::new();
        for (k, v) in Self::char_map().iter() {
            map.insert(*v, *k);
        }
        map
    }

    fn get_manhattan_distance(&self, target_pos: (usize, usize)) -> usize {
        (self.current_pos.0 as isize - target_pos.0 as isize).abs() as usize 
        + (self.current_pos.1 as isize - target_pos.1 as isize).abs() as usize
    }

    fn find_shortest_seq(&mut self, target: char) -> Vec<String> {
        let mut queue = VecDeque::new();
        let visited = HashSet::new();
        let mut move_list = Vec::new();
        let target_pos = self.pad[&target];
        let dist = self.get_manhattan_distance(target_pos);
        let current_pos = self.current_pos;
        let moves = String::new();
        let dir_map = DirectionMap::new();
        queue.push_back((dist, current_pos, moves, visited.clone()));

        while !queue.is_empty() {
            let (_, pos, moves, mut visited_local) = queue.pop_front().unwrap();
            visited_local.insert(pos);

            if pos == target_pos {
                self.current_pos = pos;
                if dist == moves.len() {
                    move_list.push(moves.to_owned() + "A");
                }
                continue;
            }

            let (r, c) = pos;
            for &(dr, dc) in dir_map.map.keys() {
                let ch = dir_map.map[&(dr, dc)];
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;
                if new_r < 0 || new_c < 0 {
                    continue;
                }
                let (new_r_usize, new_c_usize) = (new_r as usize, new_c as usize);

                if !self.reverse_map.contains_key(&(new_r_usize, new_c_usize)) {
                    continue;
                }
                if visited_local.contains(&(new_r_usize, new_c_usize)) {
                    continue;
                }

                let new_dist = (new_r - target_pos.0 as isize).abs() 
                    + (new_c - target_pos.1 as isize).abs();

                let new_moves = moves.to_owned() + &ch.to_string();
                let visited_clone = visited_local.clone();
                queue.push_back((new_dist as usize, (new_r_usize, new_c_usize), new_moves, visited_clone));
            }
        }
        move_list

    }

    fn process_code(&mut self, code: &str) -> Vec<String> {
        let mut move_list = Vec::new();
        for c in code.chars() {
            let seq = self.find_shortest_seq(c);
            move_list.push(seq);
        }

        let mut all_seqs = Vec::new();
        let mut queue = VecDeque::new();
        // Use an isize or i32 for an index that might start at -1.
        queue.push_back((String::new(), -1_i32));

        while let Some((current_moves, index)) = queue.pop_front() {
            if (index + 1) as usize >= move_list.len() {
                all_seqs.push(current_moves);
                continue;
            }
            for move_seq in &move_list[(index + 1) as usize] {
                queue.push_back((current_moves.clone() + move_seq, index + 1));
            }
        }

        all_seqs
    }
}

struct ArrowPad {
    pad: HashMap<char, HashMap<char, String>>,
}

impl ArrowPad {
    fn new() -> Self {
        ArrowPad {
            pad: Self::pad_move_map(),
        }
    }
    fn pad_move_map()->HashMap<char, HashMap<char, String>> {
        let mut map = HashMap::new();
        let mut inner_map = HashMap::new();
        inner_map.insert('A', "A".to_string());
        inner_map.insert('^', "<A".to_string());
        inner_map.insert('v', "<vA".to_string());
        inner_map.insert('<', "v<<A".to_string());
        inner_map.insert('>', "vA".to_string());
        map.insert('A', inner_map);
        let mut inner_map = HashMap::new();
        inner_map.insert('A', ">A".to_string());
        inner_map.insert('^', "A".to_string());
        inner_map.insert('v', "vA".to_string());
        inner_map.insert('<', "v<A".to_string());
        inner_map.insert('>', "v>A".to_string());
        map.insert('^', inner_map);
        let mut inner_map = HashMap::new();
        inner_map.insert('A', "^>A".to_string());
        inner_map.insert('^', "^A".to_string());
        inner_map.insert('v', "A".to_string());
        inner_map.insert('<', "<A".to_string());
        inner_map.insert('>', ">A".to_string());
        map.insert('v', inner_map);
        let mut inner_map = HashMap::new();
        inner_map.insert('A', ">>^A".to_string());
        inner_map.insert('^', ">^A".to_string());
        inner_map.insert('v', ">A".to_string());
        inner_map.insert('<', "A".to_string());
        inner_map.insert('>', ">>A".to_string());
        map.insert('<', inner_map);
        let mut inner_map = HashMap::new();
        inner_map.insert('A', "^A".to_string());
        inner_map.insert('^', "<^A".to_string());
        inner_map.insert('v', "<A".to_string());
        inner_map.insert('<', "<<A".to_string());
        inner_map.insert('>', "A".to_string());
        map.insert('>', inner_map);
        map
    }
}

fn get_seq_count(seq: &str) -> usize {
    let mut count = 0;
    let mut seq_iter = seq.chars();
    let current_char = match seq_iter.next() {
        Some(ch) => ch,
        None => return 0,
    };
    for c in seq_iter {
        if c == current_char {
            count += 1;
        }
    }
    count
}

fn get_max_consecutive_sequences(sequence_list: Vec<String>) -> Vec<String> {
    let mut sequence_count_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut max_consecutive_count: usize = 0; // use usize

    for sequence in sequence_list.iter() {
        let consecutive_count = get_seq_count(sequence);

        sequence_count_map
            .entry(consecutive_count)
            .or_insert_with(Vec::new)
            .push(sequence.clone());

        if consecutive_count > max_consecutive_count {
            max_consecutive_count = consecutive_count;
        }
    }

    sequence_count_map
        .get(&max_consecutive_count)
        .cloned()
        .unwrap_or_else(Vec::new)
}

#[memoize]
fn find_seq_len(prev_button: char, current_button: char, num_robots: usize, robot_num: usize) -> usize {
    let arrow_pad = ArrowPad::new();
    if robot_num == num_robots {
        return arrow_pad.pad[&prev_button][&current_button].len();
    }
    let mut total_len = 0;
    let mut next_move_prev_button = 'A';
    for button in arrow_pad.pad[&prev_button][&current_button].chars() {
        total_len += find_seq_len(next_move_prev_button, button, num_robots, robot_num + 1);
        next_move_prev_button = button;
    }
    total_len
}

fn get_seq_len(seq: &str, num_robots: usize) -> usize {
    let mut total_len = 0;
    let mut prev_button = 'A';
    for button in seq.chars() {
        total_len += find_seq_len(prev_button, button, num_robots, 1);
        prev_button = button;
    }
    total_len
}

fn solve(codes: &[String], num_robots: usize) -> usize {
    let mut total = 0;

    for code in codes.iter() {
        let mut door = NumericPad::new();
        let all_seqs = door.process_code(code);
        let max_consecutive_sequences = get_max_consecutive_sequences(all_seqs);

        let mut min_len = usize::MAX;
        for seq in max_consecutive_sequences.iter() {
            let seq_len = get_seq_len(seq, num_robots);
            if seq_len < min_len {
                min_len = seq_len;
            }
        }

        // parse all but the last character
        let code_value: usize = code[..code.len() - 1].parse().unwrap();
        total += code_value * min_len;
    }

    total
}

#[aoc(day21, part1)]
fn part1(input: &[String]) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
fn part2(input: &[String]) -> usize {
    solve(input, 25)
}

#[cfg(test)]
mod tests{
    use super::*;

    const TEST_INPUT: &str = r#"029A
980A
179A
456A
379A
"#;

    #[test]
    fn test_part1() {
        let inp = parse_inputs(TEST_INPUT);
        assert_eq!(part1(&inp), 126384);
    }
}