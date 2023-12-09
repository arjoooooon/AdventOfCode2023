use std::fs;
use std::cmp::min;
fn main() {
    let contents: String = fs::read_to_string("data/input.txt").expect("Failed to read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut copies: Vec<i32> = vec![1; lines.len()];

    for (i, line) in (&lines).into_iter().enumerate() {
        let nums = line.split(':').nth(1).unwrap();
        let pieces: Vec<&str> = nums.split('|').collect();

        let winning: Vec<i32> = pieces[0]
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| s.parse().unwrap())
            .collect();
        let hand: Vec<i32> = pieces[1]
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| s.parse().unwrap())
            .collect();

        let count: Vec<i32> = hand.into_iter().filter(|i| winning.contains(i)).collect();

        for idx in (i+1)..=min(i+count.len(), lines.len()-1) {
            copies[idx] += copies[i]
        }
    }

    let total: i32 = (&copies).into_iter().sum();
    println!("{total}");
}
