use std::{fs, collections::HashMap};


fn part1(contents: &String) {
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut adj_list: Vec<Vec<usize>> = Vec::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let src = split[0];
        let cur = *map.entry(src).or_insert_with(|| {
            adj_list.push(Vec::new());
            adj_list.len()-1
        });
        for dst in split[1].split(" ") {
            let dst_cur = *map.entry(dst).or_insert_with(|| {
                adj_list.push(Vec::new());
                adj_list.len()-1
            });
            adj_list[cur].push(dst_cur);
            adj_list[dst_cur].push(cur);
        }
    }

    // println!("{map:#?}");

    let mut start = 0;
    let mut start_len = 0;
    for i in 0..adj_list.len() {
        if adj_list[i].len() > start_len {
            start = i;
            start_len = adj_list[i].len();
        }
    }

    let mut group: Vec<usize> = Vec::new();
    group.push(start);
    let mut outgoing: Vec<i32> = Vec::new();
    let mut count = 0;
    for i in 0..adj_list.len() {
        if adj_list[start].contains(&i) {
            outgoing.push(1);
            count += 1;
        } else {
            outgoing.push(0);
        }
    }

    while count != 3 {
        // println!("{group:#?}");
        let mut min_change = 1000;
        let mut min_index = 0;
        for i in 0..outgoing.len() {
            if outgoing[i] == 0 {
                continue;
            }
            let mut change = -outgoing[i];
            for k in &adj_list[i] {
                if !group.contains(k) {
                    change += 1;
                }
            }
            if change < min_change {
                min_change = change;
                min_index = i;
            }
        }
        outgoing[min_index] = 0;
        count += min_change;
        group.push(min_index);
        for k in &adj_list[min_index] {
            if !group.contains(k) {
                outgoing[*k] += 1;
            }
        }
        // println!("{min_index}");
    }

    let result = group.len() * (adj_list.len() - group.len());
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day25.txt").expect("");

    part1(&contents);
}