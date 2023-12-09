use std::fs;
use std::cmp::min;

fn get_location(reference: &Vec<Vec<Option<u32>>>, i: u32) -> u32{
    let mut result = i;
    for j in 0..7 {
        result  = reference[j][result as usize].unwrap();
    }
    return result;
}

fn part1(lines: &Vec<&str>) {
    let mut reference: Vec<Vec<Option<u32>>> = vec![vec![None; u32::MAX as usize]; 7];
    let mut idx: isize = -1;

    for line in lines {
        if !line.chars().nth(0).unwrap().is_numeric() {
            idx += 1;
            continue;
        }

        let components: Vec<u32> = line
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| s.parse().unwrap())
            .collect();

        let dest = components[0];
        let src = components[1];
        let range = components[2];

        for i in 0..=range {
            reference[idx as usize][(src+i) as usize] = Some(dest+i);
        }
    }

    let mut min_val = u32::MAX;
    for i in 0..u32::MAX {
        min_val = min(min_val, get_location(&reference, i));
    }
    
    println!("{min_val}");

}
fn main() {
    let contents = fs::read_to_string("data/sample.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').filter(|s| *s != "").collect();

    part1(&lines);
}
