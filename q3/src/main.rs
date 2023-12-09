use std::fs;

fn in_bounds(row: isize, col: isize, lines: &Vec<&str>) -> bool {
    let row_in_bounds = row >= 0 && row < lines.len().try_into().unwrap();
    let col_in_bounds = col >= 0 && col < lines[0].len().try_into().unwrap();
    return row_in_bounds && col_in_bounds;
}

fn print_around(i: isize, start: isize, end: isize, lines: &Vec<&str>) {
    for j in i-1..=i+1 {
        for k in start-1..=end {
            if !in_bounds(j, k, lines) { continue; }
            let chr = lines[j as usize].chars().nth(k as usize).unwrap();
            print!("{chr}");
        }
        println!();
    }
}

fn process_integer(i: isize, start: isize, end: isize, lines: &Vec<&str>) -> i32 {
    let drow: Vec<isize>= vec![0, 1, -1];
    let dcol: Vec<isize>= vec![0, 1, -1];

    let num: &str = &lines[i as usize][(start as usize)..(end as usize)];
    // println!("{num}");

    for pos in start..=end {
        for dr in (&drow).into_iter() {
            for dc in (&dcol).into_iter() {
                if *dr == 0 && *dc == 0 { continue; }
                let r = i + dr;
                let c = pos + dc;

                if !in_bounds(r, c, lines) { continue; }
                let chr = lines[r as usize].as_bytes()[c as usize] as char;
                if !chr.is_numeric() && chr != '.' && num != "" {
                    print_around(i, start, end, lines);
                    println!();
                    let num: &str = &lines[i as usize][(start as usize)..(end as usize)];
                    return num.parse().unwrap();
                }
            }
        }
    }

    return 0;
}

fn main() {
    let contents = fs::read_to_string("data/inputls.txt")
        .expect("Failed to read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut total = 0;
    for (i, line) in (&lines).into_iter().enumerate() {
        let mut start: isize = -1;
        let mut end: isize = 0;

        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() { 
                end += 1; 
                if end as usize == line.len() {
                    total += process_integer(i as isize, start+1, end, &lines);
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
