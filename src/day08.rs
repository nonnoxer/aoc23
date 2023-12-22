use std::{fs, collections::HashMap};
use num::integer::lcm;

fn part1(contents: &String) {
    let mut lines = contents.lines();
    let steps = lines.next().expect("Should have first line");
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    let mut result = 0;
    let mut cur = "AAA";
    'outer: loop {
        for step in steps.chars() {
            match step {
                'L' => cur = map.get(cur).expect("Should have valid value").0,
                'R' => cur = map.get(cur).expect("Should have valid value").1,
                _ => (),
            }
            result += 1;
            if cur == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("{result}");
}

fn part2(contents: &String) {
    let mut lines = contents.lines();
    let steps = lines.next().expect("Should have first line");
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curs: Vec<&str> = Vec::new();
    let mut results = Vec::new();
    for line in lines {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
        if &line[2..=2] == "A" {curs.push(&line[0..3])};
    }

    for i in 0..curs.len() {
        let mut result: u128 = 0;
        'repeat: loop {
            for step in steps.chars() {
                match step {
                    'L' => curs[i] = map.get(curs[i]).expect("Should have valid value").0,
                    'R' => curs[i] = map.get(curs[i]).expect("Should have valid value").1,
                    _ => (),
                }
                result += 1;
                if &curs[i][2..=2] == "Z" {
                    break 'repeat;
                }
            }
        }
        results.push(result);
    }
    let mut ans = 1;
    for result in results {
        ans = lcm(ans, result);
    }
    println!("{ans}");
}

pub fn main() {
    let file_path = "input/day08.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    part1(&contents);
    part2(&contents);
}