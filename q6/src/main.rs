use std::fs;
use itertools::{Itertools};

fn part1(lines: &Vec<&str>) {
    let proc = |s: &str| {
        let result: Vec<i32> = s
            .split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| s.parse().unwrap())
            .collect();

        return result;
    };

    let times = proc(lines[0]);
    let dists = proc(lines[1]);

    let mut prod = 1;
    for (d, t) in dists.iter().zip(times.iter()) {
        let mut count = 0;

        for b in 1..*t {
            let nd = b*(t-b);
            // println!("{nd}");
            if nd > *d { count += 1; }
        }

        prod *= count;
        // println!("{count}");
    }

    println!("Part 1: {prod}");
}

fn part2(lines: &Vec<&str>) {
    let proc = |s: &str| {
        let result: i64 = s
            .split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|s| *s != "")
            .join("")
            .parse()
            .unwrap();

        return result
    };

    let t = proc(lines[0]);
    let d = proc(lines[1]);

    let mut count = 0;

    for b in 1..t {
        let nd = b*(t-b);
        // println!("{nd}");
        if nd > d { count += 1; }
    }
    
    println!("Part 2: {count}");
}
fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').collect();

    part1(&lines);
    part2(&lines);
}
