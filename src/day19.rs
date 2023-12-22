use std::{fs, collections::HashMap};


fn part1(contents: &String) {
    let split: Vec<&str> = contents.split("\n\n").collect();

    let mut flows: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in split[0].lines() {
        let split: Vec<&str> = line.split(['{', '}']).collect();
        let id = split[0];
        let steps: Vec<&str> = split[1].split(',').collect();
        flows.insert(id, steps);
    }

    let mut result = 0;
    'outer: for line in split[1].lines() {
        let mut part: HashMap<&str, u64> = HashMap::new();
        let mut rating = 0;
        for attr in line[1..line.len()-1].split(',') {
            let split: Vec<&str> = attr.split('=').collect();
            let num = split[1].parse::<u64>().expect("Should be number");
            rating += num;
            part.insert(split[0], num);
        }
        let mut cur = "in";
        'inner: loop {
            let flow = &flows[cur];
            for i in 0..flow.len()-1 {
                let split: Vec<&str> = flow[i].split(':').collect();
                let attr = &split[0][0..=0];
                let sign = &split[0][1..=1];
                let num = &split[0][2..].parse::<u64>().expect("Should be number");
                let dst = split[1];
                if (sign == "<" && part[attr] < *num) || (sign == ">" && part[attr] > *num) {
                    match dst {
                        "A" => {
                            result += rating;
                            continue 'outer;
                        },
                        "R" => {
                            continue 'outer;
                        },
                        dst => {
                            cur = dst;
                            continue 'inner;
                        },
                    }
                }
            }
            match flow[flow.len()-1] {
                "A" => {
                    result += rating;
                    continue 'outer;
                },
                "R" => continue 'outer,
                dst => cur = dst,
            }
        }
    }
    println!("{result}");
}

fn recursive<'a>(mut range: HashMap<&'a str, (u64, u64)>, flows: &HashMap<&str, Vec<&'a str>>, cur: &str) -> u64 {
    if cur == "A" {
        let mut result = 1;
        for attr in ["x", "m", "a", "s"] {
            result *= range[attr].1 - range[attr].0 + 1;
        }
        return result;
    }
    if cur == "R" {
        return 0;
    }
    let mut result = 0;
    let flow = &flows[cur];
    for i in 0..flow.len()-1 {
        let split: Vec<&str> = flow[i].split(':').collect();
        let attr = &split[0][0..=0];
        let sign = &split[0][1..=1];
        let num = split[0][2..].parse::<u64>().expect("Should be number");
        let dst = split[1];
        let min = range[attr].0;
        let max = range[attr].1;
        if sign == "<" {
            if min >= num {
                continue;
            }
            if max >= num {
                let mut range_cpy = range.clone();
                range_cpy.insert(attr, (min, num-1));
                result += recursive(range_cpy, flows, dst);
                range.insert(attr, (num, max));
            }
        } else if sign == ">" {
            if max <= num {
                continue;
            }
            if min <= num {
                let mut range_cpy = range.clone();
                range_cpy.insert(attr, (num+1, max));
                result += recursive(range_cpy, flows, dst);
                range.insert(attr, (min, num));
            }
        }
    }
    result += recursive(range, flows, flow[flow.len()-1]);
    result
}

fn part2(contents: &String) {
    let split: Vec<&str> = contents.split("\n\n").collect();

    let mut flows: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in split[0].lines() {
        let split: Vec<&str> = line.split(['{', '}']).collect();
        let id = split[0];
        let steps: Vec<&str> = split[1].split(',').collect();
        flows.insert(id, steps);
    }

    let mut initial_range: HashMap<&str, (u64, u64)> = HashMap::new();
    for attr in ["x", "m", "a", "s"] {
        initial_range.insert(attr, (1, 4000));
    }
    let result = recursive(initial_range, &flows, "in");
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day19.txt").expect("");

    part1(&contents);
    part2(&contents);
}