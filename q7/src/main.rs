use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
enum Card {
    Joker, 
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
enum Hand {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

fn to_card(chr: char, j: Card) -> Card {
    match chr {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => j,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Incorrect card value passed")
    } 
}

fn to_hand(hand: &Vec<Card>) -> Hand {
    let mut counts: HashMap<Card, u8> = HashMap::new();
    let mut jokers: u8 = 0;

    for card in hand {
        if *card == Card::Joker {
            jokers += 1;
            continue;
        }
        *counts.entry(*card).or_insert(0) += 1;
    }

    if counts.len() <= 1 { return Hand::FiveKind; }
    if counts.len() == 2 {
        let max_c = counts.values().max().unwrap();
        if (*max_c) + jokers == 4 { return Hand::FourKind; }
        return Hand::FullHouse;
    }
    if counts.len() == 3 {
        let max_c = counts.values().max().unwrap();
        if (*max_c) + jokers == 3 { return Hand::ThreeKind; }
        return Hand::TwoPair;
    }
    if counts.len() == 4 { return Hand::OnePair; }

    return Hand::High;
}


fn part1(lines: &Vec<&str>) {
    let procstrings = |vals: &Vec<&str>| {
        let cards: Vec<Card> = vals[0].chars().map(|c| to_card(c, Card::Jack)).collect();
        let hand: Hand = to_hand(&cards);
        let bid: i32 = vals[1].parse().unwrap();
        return (hand, cards, bid);
    };

    let procline = |line: &str| {
        let tokens: Vec<&str> = line.split(' ').collect();
        return procstrings(&tokens);
    };

    let mut elems: Vec<(Hand, Vec<Card>, i32)> = lines.iter().map(|l| procline(l)).collect();
    elems.sort();

    let mut total = 0;
    for (rank, elem) in elems.iter().enumerate() {
        total += (rank+1) * (((*elem).2) as usize);
    }

    println!("Part 1: {total}");
}

fn part2(lines: &Vec<&str>) {
    let procstrings = |vals: &Vec<&str>| {
        let cards: Vec<Card> = vals[0].chars().map(|c| to_card(c, Card::Joker)).collect();
        let hand: Hand = to_hand(&cards);
        let bid: i32 = vals[1].parse().unwrap();
        return (hand, cards, bid);
    };

    let procline = |line: &str| {
        let tokens: Vec<&str> = line.split(' ').collect();
        return procstrings(&tokens);
    };

    let mut elems: Vec<(Hand, Vec<Card>, i32)> = lines.iter().map(|l| procline(l)).collect();
    elems.sort();

    let mut total = 0;
    for (rank, elem) in elems.iter().enumerate() {
        total += (rank+1) * (((*elem).2) as usize);
    }

    println!("Part 2: {total}");
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').collect();

    part1(&lines);
    part2(&lines);
}
