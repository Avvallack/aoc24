use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> HashSet<(String, String)> {
    input.lines().map(|l| {
        let mut parts = l.split("-");
        (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
    }).collect()
}

fn get_connections(input: &HashSet<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    for (a, b) in input {
        connections.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        connections.entry(b.clone()).or_insert_with(Vec::new).push(a.clone());
    }
    connections
}

fn get_cliques(conns: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut all_cliques: Vec<Vec<String>> = vec![];
    for (node, succs) in conns.iter().sorted() {
        if let Some(part) = all_cliques
            .iter_mut()
            .find(|cq| cq.iter().all(|ch| succs.contains(ch)))
        {
            part.push(node.clone());
        } else {
            all_cliques.push(vec![node.clone()]);
        }
    }

    all_cliques
}

fn get_t_triples(connections: &HashMap<String, Vec<String>>) -> HashSet<BTreeSet<String>> {
    let mut t_triples = HashSet::new();
    for (a, b) in connections.iter() {
        if a.starts_with("t") {
        
            for c in b {
                for d in connections.get(c).unwrap() {
                    if b.contains(d) {
                        let mut t = BTreeSet::new();
                        t.insert(a.clone());
                        t.insert(c.clone());
                        t.insert(d.clone());

                        t_triples.insert(t);
                    }
                }
            }
        }
    }
    t_triples
}

fn find_largest_lan(input: HashSet<(String, String)>) -> Vec<String> {
    let connections = get_connections(&input);
    let cliques = get_cliques(&connections);
    let largest_clique = cliques.into_iter().max_by_key(|c| c.len()).unwrap_or_default();

    let mut result = largest_clique;
    result.sort();
    result
}


#[aoc(day23, part1)]
fn part1(input: &HashSet<(String, String)>) -> usize {
    let connections = get_connections(input);
    let t_triples = get_t_triples(&connections);
    t_triples.len()
}

#[aoc(day23, part2)]
fn part2(input: &HashSet<(String, String)>) -> String {
    let lan = find_largest_lan(input.clone());
    lan.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

const TEST_INPUT2: &str = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT2);
        assert_eq!(part2(&input), "co,de,ka,ta");
    }
}