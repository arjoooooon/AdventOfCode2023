use std::fs;

fn in_bounds(row: isize, col: isize, lines: &Vec<String>) -> bool {
    let row_in_bounds = row >= 0 && row < lines.len().try_into().unwrap();
    let col_in_bounds = col >= 0 && col < lines[0].len().try_into().unwrap();
    return row_in_bounds && col_in_bounds;
}

fn print_around(i: isize, start: isize, end: isize, lines: &Vec<String>) {
    for j in i-1..=i+1 {
        for k in start-1..=end {
            if !in_bounds(j, k, lines) { continue; }
            let chr = lines[j as usize].chars().nth(k as usize).unwrap();
            print!("{chr}");
        }
        println!();
    }
}

fn process_integer(i: isize, start: isize, end: isize, lines: &Vec<String>) -> i32 {
    for row in i-1..=i+1 {
        for col in start-1..=end {
            if !in_bounds(row, col, lines) { continue; }
            let chr = lines[row as usize].chars().nth(col as usize).unwrap();
            if chr.is_numeric() || chr == '.' { continue; }

            // print_around(i, start, end, lines);
            // println!();
            return lines[i as usize][(start as usize)..(end as usize)].parse().unwrap();
        }
    }

    return 0;
}

fn part1(lines: &Vec<String>) {
    let mut total = 0;
    for (i, line) in (&lines).into_iter().enumerate() {
        let mut start: isize = -1;
        let mut end: isize = 0;

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() { 
                end += 1; 
                if (end as usize) == line.len()-1 {
                    total += process_integer(i as isize, start+1, end, lines);
                }
            }
            else {
                if end != start { total += process_integer(i as isize, start+1, j as isize, &lines); }
                start = j as isize;
                end = j as isize;
            }
        }
    }

    println!("{total}");
}

fn main() {
    let contents = fs::read_to_string("data/input.txt")
        .expect("Failed to read file");

    let lines: Vec<String> = contents.split('\n').map(|l| [".", l, "."].join("")).collect();

    part1(&lines);
}
