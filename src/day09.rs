use std::fs;

fn calc_next_seq(seq: Vec<i32>) -> i32 {
    let mut d_seq: Vec<i32> = Vec::new();
    let mut all_zero = true;
    for i in 0..seq.len()-1 {
        let d = seq[i+1] - seq[i];
        if d != 0 {all_zero = false;}
        d_seq.push(d);
    }
    if all_zero {seq[seq.len() - 1]}
    else {seq[seq.len() - 1] + calc_next_seq(d_seq)}
}

fn calc_prev_seq(seq: Vec<i32>) -> i32 {
    let mut d_seq: Vec<i32> = Vec::new();
    let mut all_zero = true;
    for i in 0..seq.len()-1 {
        let d = seq[i+1] - seq[i];
        if d != 0 {all_zero = false;}
        d_seq.push(d);
    }
    if all_zero {seq[0]}
    else {seq[0] - calc_prev_seq(d_seq)}
}

fn part1(contents: &String) {
    let mut result: i32 = 0;
    for line in contents.lines() {
        let mut seq: Vec<i32> = Vec::new();
        for num in line.split(" ") {
            seq.push(num.parse::<i32>().expect("Should be integer"));
        }
        result += calc_next_seq(seq);
    }
    println!("{result}");
}

fn part2(contents: &String) {
    let mut result: i32 = 0;
    for line in contents.lines() {
        let mut seq: Vec<i32> = Vec::new();
        for num in line.split(" ") {
            seq.push(num.parse::<i32>().expect("Should be integer"));
        }
        result += calc_prev_seq(seq);
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day09.txt").expect("");

    part1(&contents);
    part2(&contents);
}