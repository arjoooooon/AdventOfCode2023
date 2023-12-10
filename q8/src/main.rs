use std::fs;
use std::collections::HashMap;
use num_integer::lcm;

fn count_steps(start: &str, move_string: &str, transitions: &HashMap<&str, (&str, &str)>) -> i32 {
    let mut count = 0;
    let mut current: &str = start;
    let mut idx = 0;

    while current.chars().nth(2).unwrap() != 'Z' {
        count += 1;
        if move_string.chars().nth(idx).unwrap() == 'L' {
            current = transitions.get(current).unwrap().0;
        } else {
            current = transitions.get(current).unwrap().1;
        }

        idx = (idx + 1) % move_string.len();
    }

    return count;
}

fn part1(lines: &Vec<&str>) {
    let move_string = lines[0];
    let mut transitions: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in 1..lines.len() {
        let line = lines[i];
        let src = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        transitions.insert(src, (left, right));
    }
    
    let result = count_steps("AAA", move_string, &transitions);
    println!("Part 1: {result}");
}

fn part2(lines: &Vec<&str>) {
    let move_string = lines[0];
    let mut transitions: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in 1..lines.len() {
        let line = lines[i];
        let src = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        transitions.insert(src, (left, right));
    }


    let mut basis: u64 = 1;
    for i in 1..lines.len() {
        let line = lines[i];
        if line.chars().nth(2).unwrap() != 'A' { continue; }
        let result = count_steps(&line[0..3], move_string, &transitions);

        basis = lcm(basis as u64, result as u64);
    }

    println!("Part 2: {basis}");
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').filter(|l| *l != "").collect();

    // part1(&lines);
    part2(&lines);
}

