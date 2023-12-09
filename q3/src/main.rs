use std::fs;
use std::collections::HashMap;

fn in_bounds(row: isize, col: isize, lines: &Vec<String>) -> bool {
    let row_in_bounds = row >= 0 && row < lines.len().try_into().unwrap();
    let col_in_bounds = col >= 0 && col < lines[0].len().try_into().unwrap();
    return row_in_bounds && col_in_bounds;
}

#[allow(dead_code)]
fn print_around(i: isize, start: isize, end: isize, lines: &Vec<String>) {
    for j in i - 1..=i + 1 {
        for k in start - 1..=end {
            if !in_bounds(j, k, lines) {
                continue;
            }
            let chr = lines[j as usize].chars().nth(k as usize).unwrap();
            print!("{chr}");
        }
        println!();
    }
}

fn process_integer(i: isize, start: isize, end: isize, lines: &Vec<String>) -> i32 {
    for row in i - 1..=i + 1 {
        for col in start - 1..=end {
            if !in_bounds(row, col, lines) {
                continue;
            }
            let chr = lines[row as usize].chars().nth(col as usize).unwrap();
            if chr.is_numeric() || chr == '.' {
                continue;
            }

            // print_around(i, start, end, lines);
            // println!();
            return lines[i as usize][(start as usize)..(end as usize)]
                .parse()
                .unwrap();
        }
    }

    return 0;
}

fn process_integer_help(
    i: isize,
    start: isize,
    end: isize,
    lines: &Vec<String>,
    help: &mut HashMap<(isize, isize), Vec<i32>>
) -> i32 {
    let mut result: i32 = 0;

    for row in i - 1..=i + 1 {
        for col in start - 1..=end {
            if !in_bounds(row, col, lines) {
                continue;
            }
            let chr = lines[row as usize].chars().nth(col as usize).unwrap();
            if chr.is_numeric() || chr == '.' {
                continue;
            }

            // print_around(i, start, end, lines);
            // println!();
            result = lines[i as usize][(start as usize)..(end as usize)]
                .parse()
                .unwrap();
            // println!("{result}");
            
            
            (*help).entry((row, col)).or_insert(vec![]).push(result);
        }
    }

    return result;
}

fn part1(lines: &Vec<String>) {
    let mut total = 0;
    for (i, line) in (&lines).into_iter().enumerate() {
        let mut start: isize = -1;
        let mut end: isize = 0;

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                end += 1;
                if (end as usize) == line.len() - 1 {
                    total += process_integer(i as isize, start + 1, end, lines);
                }
            } else {
                if end != start {
                    total += process_integer(i as isize, start + 1, j as isize, &lines);
                }
                start = j as isize;
                end = j as isize;
            }
        }
    }

    println!("Part 1: {total}");
}

fn part2(lines: &Vec<String>) {
    let mut total = 0;

    let mut help: HashMap<(isize, isize), Vec<i32>> = HashMap::new();

    for (i, line) in (&lines).into_iter().enumerate() {
        let mut start: isize = -1;
        let mut end: isize = 0;

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                end += 1;
                if (end as usize) == line.len() - 1 {
                    let _ = process_integer_help(i as isize, start+1, end, &lines, &mut help);
                }
            } else {
                if end != start {
                    process_integer_help(i as isize, start+1, j as isize, &lines, &mut help);
                }
                start = j as isize;
                end = j as isize;
            }
        }
    }

    for key in help.keys() {
        let value = help.get(key).unwrap();
        if value.len() < 2 { continue; }

        for i in 0..value.len() {
            for j in i+1..value.len() {
                total += value[i] * value[j]
            }
        }
    }

    println!("Part 2: {total}");
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file");

    let lines: Vec<String> = contents
        .split('\n')
        .map(|l| [".", l, "."].join(""))
        .collect();

    part1(&lines);
    part2(&lines);
}
