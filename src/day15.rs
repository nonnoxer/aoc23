use std::fs;

fn part1(contents: &String) {
    let mut result = 0;
    for s in contents.trim().split(",") {
        let mut hash = 0;
        for c in s.chars() {
            hash = (hash + c as u32) * 17 % 256;
        }
        result += hash;
    }
    println!("{result}")
}

fn hash(label: &str) -> usize {
    let mut hash = 0;
    for c in label.chars() {
        hash = (hash + c as usize) * 17 % 256;
    }
    hash
}

fn part2(contents: &String) {
    let mut boxes: Vec<Vec<(&str, i32)>> = Vec::new();
    for _ in 0..256 {
        let mut b: Vec<(&str, i32)> = Vec::new();
        boxes.push(b);
    }

    for s in contents.trim().split(",") {
        if s.ends_with("-") {
            let label = &s[..s.len()-1];
            let cur = hash(label);
            for i in 0..boxes[cur].len() {
                if boxes[cur][i].0 == label {
                    boxes[cur].remove(i);
                    break;
                }
            }
        }
        else {
            let split: Vec<&str> = s.split("=").collect();
            let label = split[0];
            let length = split[1].parse::<i32>().expect("Should be number");
            let cur = hash(label);
            let mut found = false;
            for i in 0..boxes[cur].len() {
                if boxes[cur][i].0 == label {
                    boxes[cur][i].1 = length;
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[cur].push((label, length));
            }
        }
    }

    let mut result = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            result += (i+1) * (j+1) * boxes[i][j].1 as usize;
        }
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day15.txt").expect("");
    part1(&contents);
    part2(&contents);
}
