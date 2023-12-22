use std::fs;

struct PokerBid {
    hand: Hand,
    bid: i32,
}

enum Hand {
    Five(i32),
    Four(i32, i32), 
    FullHouse(i32, i32),
    Three(i32, [i32; 2]),
    TwoPair([i32; 2], i32),
    Pair(i32, [i32; 3]),
    One([i32; 5]),
}

struct Bid<'a> {
    rank: i32,
    hand: &'a str,
    bid: i32,
}

fn val(c: char) -> i32 {
    match c {
        '2'..='9' => c.to_digit(10).expect("") as i32 - 2,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid character"),
    }
}

fn parse_poker_hand(hand_str: &str) -> Hand {
    let mut counts: [Vec<i32>; 5] = Default::default();
    let mut count_card = [0; 13];
    
    for card in hand_str.chars() {
        count_card[val(card) as usize] += 1;
    }
    for (i, count) in count_card.into_iter().enumerate() {
        if count > 0 {counts[count-1].push(i as i32);}
    }
    for i in 0..5 {
        counts[i].sort();
        counts[i].reverse();
    }

    if counts[4].len() == 1 {
        Hand::Five(counts[4][0])
    }
    else if counts[3].len() == 1 {
        Hand::Four(counts[3][0], counts[0][0])
    }
    else if counts[2].len() == 1 {
        if counts[1].len() == 1 {
            Hand::FullHouse(counts[2][0], counts[1][0])
        }
        else {
            let mut others: [i32; 2] = [0; 2];
            for i in 0..2 {
                others[i] = counts[0][i];
            }
            Hand::Three(counts[2][0], others)
        }
    }
    else if counts[1].len() == 2 {
        let mut others: [i32; 2] = [0; 2];
        for i in 0..2 {
            others[i] = counts[1][i];
        }
        Hand::TwoPair(others, counts[0][0])
    }
    else if counts[1].len() == 1 {
        let mut others: [i32; 3] = [0; 3];
        for i in 0..3 {
            others[i] = counts[0][i];
        }
        Hand::Pair(counts[1][0], others)
    }
    else {
        let mut others: [i32; 5] = [0; 5];
        for i in 0..5 {
            others[i] = counts[0][i];
        }
        Hand::One(others)
    }
}

fn parse_hand(hand_str: &str) -> i32 {
    let mut counts: [Vec<i32>; 5] = Default::default();
    let mut count_card = [0; 13];

    for card in hand_str.chars() {
        count_card[val(card) as usize] += 1;
    }
    for (i, count) in count_card.into_iter().enumerate() {
        if count > 0 {counts[count-1].push(i as i32);}
    }

    if counts[4].len() == 1 {
        6
    }
    else if counts[3].len() == 1 {
        5
    }
    else if counts[2].len() == 1 {
        if counts[1].len() == 1 {
            4
        }
        else {
            3
        }
    }
    else if counts[1].len() == 2 {
        2
    }
    else if counts[1].len() == 1 {
        1
    }
    else {
        0
    }
}

fn calculate_poker_hand(hand: &Hand) -> i32 {
    match hand {
        Hand::Five(n) => {
            7000000 + n
        },
        Hand::Four(n, o) => {
            6000000 + n * 100 + o
        },
        Hand::FullHouse(n, o) => {
            5000000 + n * 100 + o
        },
        Hand::Three(n, o) => {
            4000000 + n * 169 + o[0] * 13 + o[1]
        },
        Hand::TwoPair(n, o) => {
            3000000 + n[0] * 169 + n[1] * 13 + o
        },
        Hand::Pair(n, o) => {
            2000000 + n * 2197 + o[0] * 169 + o[1] * 13 + o[2]
        },
        Hand::One(o) => {
            o[0] * 28561 + o[1] * 2197 + o[2] * 169 + o[3] * 13 + o[4]
        },
    }
}

fn calculate_bid(bid: &Bid) -> i32 {
    let mut out = bid.rank;
    for c in bid.hand.chars() {
       out = out * 13 + val(c); 
    }
    out
}

fn part1(contents: &String) {
    let mut bids: Vec<Bid> = Vec::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        bids.push(Bid{
            rank: parse_hand(split[0]),
            hand: split[0],
            bid: split[1].parse::<i32>().expect("Should be integer")
        });
    }
    bids.sort_by_key(|x| calculate_bid(&x));
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += (i as u64 + 1) * bid.bid as u64;
    }
    println!("{result}");
}

fn val_j(c: char) -> i32 {
    match c {
        'J' => 0,
        '2'..='9' => c.to_digit(10).expect("") as i32 - 1,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid character"),
    }
}

fn parse_hand_j(hand_str: &str) -> i32 {
    let mut counts: [Vec<i32>; 5] = Default::default();
    let mut count_card = [0; 13];
    let mut jokers = 0;

    for card in hand_str.chars() {
        if card == 'J' {
            jokers += 1;
        } else {
            count_card[val_j(card) as usize] += 1;
        }
    }
    let mut max_i = 0; 
    for (i, count) in count_card.into_iter().enumerate() {
        if count > count_card[max_i] {max_i = i;}
    }
    count_card[max_i] += jokers;
    for (i, count) in count_card.into_iter().enumerate() {
        if count > 0 {counts[count-1].push(i as i32);}
    }

    if counts[4].len() == 1 {
        6
    }
    else if counts[3].len() == 1 {
        5
    }
    else if counts[2].len() == 1 {
        if counts[1].len() == 1 {
            4
        }
        else {
            3
        }
    }
    else if counts[1].len() == 2 {
        2
    }
    else if counts[1].len() == 1 {
        1
    }
    else {
        0
    }
}

fn calculate_bid_j(bid: &Bid) -> i32 {
    let mut out = bid.rank;
    for c in bid.hand.chars() {
       out = out * 13 + val_j(c); 
    }
    out
}

fn part2(contents: &String) {
    let mut bids: Vec<Bid> = Vec::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        bids.push(Bid{
            rank: parse_hand_j(split[0]),
            hand: split[0],
            bid: split[1].parse::<i32>().expect("Should be integer")
        });
    }
    bids.sort_by_key(|x| calculate_bid_j(&x));
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += (i as u64 + 1) * bid.bid as u64;
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day07.txt").expect("");

    part1(&contents);
    part2(&contents);
}