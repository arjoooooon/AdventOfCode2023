use std::fs;

fn part1(lines: &Vec<&str>) {
    let mut total = 0;

    for line in lines {
        let mut tree: Vec<Vec<i32>> = vec![];
        tree.push(
            line.split(' ')
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap())
                .collect(),
        );

        while tree.last().unwrap().iter().sum::<i32>() != 0 {
            let last = tree.last().unwrap();
            tree.push(last.windows(2).map(|v| v[1]-v[0]).collect())
        }

        let mut last = 0;
        for (idx, line) in tree.iter().rev().enumerate() {
            if idx == 0 { continue; }
            last = line.last().unwrap() + last;
        }

        total += last;
    }

    println!("{total}");
}

fn part2(lines: &Vec<&str>) {
    let mut total = 0;

    for line in lines {
        let mut tree: Vec<Vec<i32>> = vec![];
        tree.push(
            line.split(' ')
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap())
                .collect(),
        );

        while tree.last().unwrap().iter().sum::<i32>() != 0 {
            let last = tree.last().unwrap();
            tree.push(last.windows(2).map(|v| v[1]-v[0]).collect())
        }

        let mut last = 0;
        for (idx, line) in tree.iter().rev().enumerate() {
            if idx == 0 { continue; }
            last = line[0] - last;
        }

        total += last;
    }

    println!("{total}");
}
fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').collect();

    part1(&lines);
    part2(&lines);
}
