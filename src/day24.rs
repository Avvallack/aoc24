use std::{clone, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn apply(&self, a: u16, b: u16) -> u16 {
        match self {
            Operator::AND => a & b,
            Operator::OR => a | b,
            Operator::XOR => a ^ b,
        }
    }

    fn from_str(op: &str) -> Operator {
        match op {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Operator,
    ina: String,
    inb: String,
    result: String,
}

impl Instruction {
    fn apply(&self, wires: &mut HashMap<String, u16>) {
        let a = wires.get(&self.ina).unwrap_or_else(|| {&0}).clone();
        let b = wires.get(&self.inb).unwrap_or_else(|| {&0}).clone();
        let res = self.op.apply(a, b);
        wires.insert(self.result.clone(), res);
    }
}

#[derive(Debug)]
struct Input {
    start_wires: HashMap<String, u16>,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day24)]
fn read_inputs(input: &str) -> Input {
    let mut start_wires = HashMap::new();
    let mut instructions = Vec::new();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    for line in parts[0].lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let value = parts[1].parse::<u16>().unwrap_or_else(|_| {0});
        let wire = parts[0].to_string();
        start_wires.insert(wire, value);
    }

    for line in parts[1].lines() {
        let re = Regex::new(r"(?P<ina>\w+)\s+(?P<operator>\w+)\s+(?P<inb>\w+)\s*->\s*(?P<res>\w+)").unwrap();
        if let Some(captures) = re.captures(line) {
            let ina = captures.name("ina").unwrap().as_str();
            let operator = captures.name("operator").unwrap().as_str();
            let inb = captures.name("inb").unwrap().as_str();
            let res = captures.name("res").unwrap().as_str();
            instructions.push(Instruction {
                op: Operator::from_str(operator),
                ina: ina.to_string(),
                inb: inb.to_string(),
                result: res.to_string(),
            });
        }
    }
    Input {
        start_wires,
        instructions,
    }
}

fn process_instructions(input: &Input) -> (HashMap<String, u16>, HashMap<String, u16>)  {
    let mut wires = input.start_wires.clone();
    let mut result_wires = HashMap::new();
    let mut instructions = input.instructions.clone();
    while !instructions.is_empty() {
        let mut new_instructions = Vec::new();
        for inst in instructions {
            if wires.contains_key(&inst.ina) && wires.contains_key(&inst.inb) {
                inst.apply(&mut wires);
                if inst.result.starts_with("z") {
                    result_wires.insert(inst.result.clone(), wires.get(&inst.result).unwrap().clone());
                }
            } else {
                new_instructions.push(inst);
            }
        }
        instructions = new_instructions;
    }
    (wires, result_wires)
}

fn get_desired_result(wires: &HashMap<String, u16>) -> String {
    let mut result = 0;
    let mut x_s = Vec::new();
    let mut y_s = Vec::new();
    for (key, value) in wires {
        if key.starts_with("x") {
            x_s.push((key, value));
        } else if key.starts_with("y") {
            y_s.push((key, value));
        }
    }
    x_s.sort_by(|a, b| {
        let a_num = a.0[1..].parse::<u32>().unwrap();
        let b_num = b.0[1..].parse::<u32>().unwrap();
        b_num.cmp(&a_num)
    });
    y_s.sort_by(|a, b| {
        let a_num = a.0[1..].parse::<u32>().unwrap();
        let b_num = b.0[1..].parse::<u32>().unwrap();
        b_num.cmp(&a_num)
    });

    let mut binary_string_x = String::new();
    for (key, value) in x_s {
        binary_string_x.push_str(&value.to_string());
    }
    let mut binary_string_y = String::new();
    for (key, value) in y_s {
        binary_string_y.push_str(&value.to_string());
    }

    let int_x = u64::from_str_radix(&binary_string_x, 2).unwrap();
    let int_y = u64::from_str_radix(&binary_string_y, 2).unwrap();

    // Perform addition
    let result = int_x + int_y;

    // Convert result to binary string
    format!("{:b}", result)
    
}

fn get_string_from_wires(input: &Input) -> String {
    let (_, res_wires) = process_instructions(input);
    
    let mut keys: Vec<_> = res_wires.keys().collect();
    keys.sort_by(|a, b| {
        let a_num = a[1..].parse::<u32>().unwrap();
        let b_num = b[1..].parse::<u32>().unwrap();
        b_num.cmp(&a_num)
    });

    let mut binary_string = String::new();
    for key in keys {
        if let Some(&value) = res_wires.get(key) {
            binary_string.push_str(&value.to_string());
        }
    }
    binary_string
}

#[aoc(day24, part1)]
fn part1(input: &Input) -> u64 {
    let binary_string = get_string_from_wires(input);
    u64::from_str_radix(&binary_string, 2).unwrap()
}



#[cfg(test)]
mod tests{
    use super::*;

    const TEST_INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const TEST_INPUT2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const TEST_INPUT3: &str = "x00: 1
x01: 1
x02: 0
x03: 1
y00: 1
y01: 0
y02: 1
y03: 1

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    #[test]
    fn test_part1() {
        let input = read_inputs(TEST_INPUT);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part12() {
        let input = read_inputs(TEST_INPUT2);
        assert_eq!(part1(&input), 2024);
    }

    #[test]
    fn test_addition(){
        let input = read_inputs(TEST_INPUT3);
        assert_eq!(get_desired_result(&input.start_wires), "11000");
    }
}
