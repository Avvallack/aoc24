use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

const FIELD: (isize, isize) = (101, 103);

#[derive(Clone, Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}


impl Robot {
    fn make_move (&mut self) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(FIELD.0);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(FIELD.1);
    }
}

fn move_robots(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        robot.make_move();
    }
}

fn check_quadrants (robots: &mut Vec<Robot>) -> i32 {
    let mut quadrants = vec![0; 4];
    for robot in robots.iter() {
        if robot.position.0 > FIELD.0 / 2 && robot.position.1 > FIELD.1 / 2 {
            quadrants[0] += 1;
        }
        else if robot.position.0 < FIELD.0 / 2 && robot.position.1 > FIELD.1 / 2 {
            quadrants[1] += 1;
        }
        else if robot.position.0 < FIELD.0 / 2 && robot.position.1 < FIELD.1 / 2 {
            quadrants[2] += 1;
        }
        else if robot.position.0 > FIELD.0 / 2 && robot.position.1 < FIELD.1 / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().product()

}

#[aoc_generator(day14)]
fn read_inputs(inp: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    
    let re = Regex::new(r"p=(?P<x1>-?\d+),(?P<x2>-?\d+) v=(?P<v1>-?\d+),(?P<v2>-?\d+)").unwrap();
    
    for line in inp.lines() {
        let Some(caps) = re.captures(&line) else 
        {   
            println!("No match");
            continue;
        };
        let x1 = caps["x1"].parse::<isize>().unwrap();
        let x2 = caps["x2"].parse::<isize>().unwrap();
        let v1 = caps["v1"].parse::<isize>().unwrap();
        let v2 = caps["v2"].parse::<isize>().unwrap();
        robots.push(Robot {position: (x1, x2), velocity: (v1, v2)});
    }

    robots
}

fn visualize_robots(robots: &Vec<Robot>) {
    let mut field = vec![vec!['.'; FIELD.0 as usize]; FIELD.1 as usize];
    for robot in robots.iter() {
        field[robot.position.1 as usize][robot.position.0 as usize] = '*';
    }

    let output: String = field.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output);
}

fn check_field(field: &Vec<Vec<i32>>) -> bool {
    for x in 0..FIELD.0 as usize {
        let mut streak = 0;

        for y in 0..FIELD.1 as usize { 
            if field[y][x] > 0 {
                streak += 1;
                if streak > 20 {
                    return true;
                }
            } else {
                streak = 0;
            }
        }
    }
    false
}

fn construct_field(robots: &Vec<Robot>) -> Vec<Vec<i32>> {
    let mut field = vec![vec![0; FIELD.0 as usize]; FIELD.1 as usize];
    for robot in robots.iter() {
        field[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    field
}

#[aoc(day14, part1)]
fn part1(robots: &Vec<Robot>) -> i32 {
    let mut robots = robots.clone();
    for _ in 0..100{
        move_robots(&mut robots);
    }
    visualize_robots(&robots);
    check_quadrants(&mut robots)
}

#[aoc(day14, part2)]
fn part2(robots: &Vec<Robot>) -> i32 {
    let mut robots = robots.clone();
    let mut elka = false;
        let mut counter = 0;
        while !elka {
            move_robots(&mut robots);
            let field = construct_field(&robots);
            elka = check_field(&field);
            counter += 1;
    }
    visualize_robots(&robots);
    counter
}
