use std::cmp::min;
use std::fs;

fn get_location(start: i64, trans: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut current = start;
    for i in 0..7 {
        for entry in &trans[i as usize] {
            let (dest, src, range) = *entry; 
            if current < src || current >= src+range { continue; }
            current = dest + (current - src);
            break;
        }
    }

    return current;
}

fn part1(lines: &Vec<&str>) {
    let parts: Vec<&str> = lines[0].split(':').collect();
    let seeds: Vec<i64> = parts[1]
        .split(' ')
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut trans: Vec<Vec<(i64, i64, i64)>> = vec![vec![]; 7];

    for (idx, piece) in lines.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let lines: Vec<&str> = piece.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            if j == 0 {
                continue;
            }
            let data: Vec<i64> = line
                .split(' ')
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap())
                .collect();

            let dest = data[0];
            let src = data[1];
            let range = data[2];

            trans[idx-1].push((dest, src, range));
        }
    }

    let min_loc = seeds.iter().map(|s| get_location(*s, &trans)).min().unwrap();
    println!("{min_loc}");
}

fn part2(lines: &Vec<&str>) {
    let parts: Vec<&str> = lines[0].split(':').collect();
    let seedranges: Vec<i64> = parts[1]
        .split(' ')
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut trans: Vec<Vec<(i64, i64, i64)>> = vec![vec![]; 7];

    for (idx, piece) in lines.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let lines: Vec<&str> = piece.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            if j == 0 {
                continue;
            }
            let data: Vec<i64> = line
                .split(' ')
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap())
                .collect();

            let dest = data[0];
            let src = data[1];
            let range = data[2];

            trans[idx-1].push((dest, src, range));
        }
    }

    let mut min_loc = i64::MAX;
    for chunk in seedranges.chunks(2) {
        let start = chunk[0];
        let range = chunk[1];

        let ml = (start..start+range).map(|s| get_location(s, &trans)).min().unwrap();
        min_loc = min(ml, min_loc);
    }    

    println!("{min_loc}");
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split("\n\n").collect();

    // println!("{:?}", lines);
    part1(&lines);
    part2(&lines);
}
