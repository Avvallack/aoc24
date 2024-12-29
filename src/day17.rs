use aoc_runner_derive::{aoc, aoc_generator};

use std::ops::BitXor;

struct Input {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<i32>,
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());

    let reg_a_line = lines.next().unwrap();
    let reg_a = reg_a_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let reg_b_line = lines.next().unwrap();
    let reg_b = reg_b_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let reg_c_line = lines.next().unwrap();
    let reg_c = reg_c_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Parse program line similarly
    let program_line = lines.next().unwrap();
    let program_part = program_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim();

    // Now split by ',' for each instruction
    let program = program_part
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    Input { reg_a, reg_b, reg_c, program }
}

#[derive(Debug)] 
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Opcode {
    fn from_i32(value: i32) -> Option<Opcode> {
        match value {
            0 => Some(Opcode::ADV),
            1 => Some(Opcode::BXL),
            2 => Some(Opcode::BST),
            3 => Some(Opcode::JNZ),
            4 => Some(Opcode::BXC),
            5 => Some(Opcode::OUT),
            6 => Some(Opcode::BDV),
            7 => Some(Opcode::CDV),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    opcode: Opcode,
    operand: i32,
}

impl Instruction {
    fn match_combo_operand(&self) -> u32 {
        match self.operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a as u32,
            5 => self.reg_b as u32,
            6 => self.reg_c as u32,
            _ => panic!("Invalid combo operand"),
        }
    }
}

fn run_program(program_stack: &[i32], reg_a: i64, reg_b: i64, reg_c: i64) -> Vec<i32> {
    let mut pc = 0;
    let mut instruction = Instruction {
        reg_a,
        reg_b,
        reg_c,
        opcode: Opcode::from_i32(program_stack[pc]).unwrap(),
        operand: program_stack[pc + 1],
    };
    let mut output = Vec::new();

    loop {
        match instruction.opcode {
            Opcode::ADV => {
                instruction.reg_a = instruction.reg_a / (2_i64).pow(instruction.match_combo_operand());
                pc += 2;
            }
            Opcode::BXL => {
                instruction.reg_b = instruction.reg_b.bitxor(instruction.operand as i64);
                pc += 2;
            }
            Opcode::BST => {
                instruction.reg_b = (instruction.match_combo_operand() % 8) as i64;
                pc += 2;
            }
            Opcode::JNZ => {
                if instruction.reg_a == 0 {
                    pc += 2;
                } else {
                    pc = instruction.operand as usize;
                }
            }
            Opcode::BXC => {
                instruction.reg_b = instruction.reg_b.bitxor(instruction.reg_c as i64);
                pc += 2;
            }
            Opcode::OUT => {
                let outp = (instruction.match_combo_operand() % 8) as i32;
                output.push(outp);
                pc += 2;
            }
            Opcode::BDV => {
                instruction.reg_b = instruction.reg_a / (2_i64).pow(instruction.match_combo_operand());
                pc += 2;
            }
            Opcode::CDV => {
                instruction.reg_c = instruction.reg_a / (2_i64).pow(instruction.match_combo_operand());
                pc += 2;
            }
        }

        if pc >= program_stack.len() {
            break;
        }

        instruction.opcode = Opcode::from_i32(program_stack[pc]).unwrap();
        instruction.operand = program_stack[pc + 1];
    }

    output
}

fn search_a(program: &[i32], iteration: usize, base_a: i64, reg_b: i64, reg_c: i64) -> Option<i64> {
    for remainder in 0..8 {
        let multiplier = 8_i64.pow(iteration as u32);

        if base_a + multiplier * (remainder as i64) < 8_i64.pow((program.len() - 1) as u32) {
            continue;
        }

        let candidate_a = base_a + multiplier * remainder as i64;

        let result = run_program(program, candidate_a, reg_b, reg_c);

        if result.len() > iteration && result[iteration] == program[iteration] {
            if iteration == 0 {
                return Some(base_a + multiplier * remainder as i64);
            } else {

                if let Some(found_a) = search_a(program, iteration - 1, base_a + multiplier * remainder as i64, reg_b, reg_c) {
                    return Some(found_a);
                }
            }
        }
    }
    None
}

fn find_minimal_a(program: &[i32]) -> Option<i64> {
    search_a(program, program.len() - 1, 0, 0, 0)
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> String {
    let stack = input.program.as_slice();
    let output = run_program(stack, input.reg_a, input.reg_b, input.reg_c);
    output.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> i64 {
    find_minimal_a(input.program.as_slice()).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const TEST_INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_run_program() {
        let inp= parse_input(TEST_INPUT);
        println!("{:?}", inp.program);
    }

    #[test]
    fn test_part1(){
        let inp = parse_input(TEST_INPUT);
        let output = part1(&inp);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_part2(){
        let inp = parse_input(TEST_INPUT2);
        let output = part2(&inp);
        assert_eq!(output, 117440);
    }
}