use std::{cmp::Ordering, collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub struct Input {
    rules: HashSet<(String, String)>,
    pages: Vec<Vec<String>>,
}

#[aoc_generator(day05)]
fn read_inputs(input: &str) -> Input {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let rules_str = sections[0];
    let pages_str = sections[1];
    let mut pages = Vec::new();
    let mut rules = HashSet::new();
    for line in rules_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let before = parts[0].trim().to_string();
            let after = parts[1].trim().to_string();
            rules.insert((before, after));
        }
    }
    for update_line in pages_str.lines() {
        let update_line = update_line.trim();
        if update_line.is_empty() {
            continue;
        }
        let page: Vec<String> = update_line.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        pages.push(page);
    }
    Input {
        rules,
        pages,
    }

    
}

fn sort_pages(inp: &Input) -> [i32; 2] {
    let mut pages = inp.pages.clone();
    let mut results = [0; 2];
    for page in &mut pages {
        let original = page.clone();
        page.sort_by(|x, y| {
            if inp.rules.contains(&(x.clone(), y.clone())) {
                Ordering::Less
            } else if inp.rules.contains(&(y.clone(), x.clone())) {
                Ordering::Greater
            } else {
                // If no rule, consider them equal (this may lead to arbitrary stable sorting)
                Ordering::Equal
            }
        });
        let changed_idx = if original != *page { 1 } else { 0 };
        let mid_idx = page.len() / 2;
        let mid_el = page[mid_idx].parse::<i32>().unwrap();
        results[changed_idx] += mid_el;
    }
    results
}

#[aoc(day05, part1)]
fn part_1 (inp: &Input) -> usize {
    sort_pages(inp)[0] as usize
}

#[aoc(day05, part2)]
fn part_2 (inp: &Input) -> usize {
    sort_pages(inp)[1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let inp = read_inputs(TEST_INPUT);
        assert_eq!(part_1(&inp), 143);
    }

    #[test]
    fn test_part2() {
        let inp = read_inputs(TEST_INPUT);
        assert_eq!(part_2(&inp), 123);
    }
}