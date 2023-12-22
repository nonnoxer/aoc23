use std::fs;

fn parse_map(map: &str) -> Vec<(u64, u64, u64)>{
    let mut result: Vec<(u64, u64, u64)> = Vec::new();
    for line in map.lines().into_iter().skip(1) {
        let split: Vec<&str> = line.split(" ").collect();
        result.push((
            split[0].parse::<u64>().expect("Should be u64"),
            split[1].parse::<u64>().expect("Should be u64"),
            split[2].parse::<u64>().expect("Should be u64")
        ));
    }
    result
}

fn transform(src: Vec<u64>, map: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    for s in src {
        let mut result = s;
        for row in map {
            if s >= row.1 && s < row.1 + row.2 {
                result = row.0 + s - row.1;
            }
        }
        output.push(result);
    }
    output
}

fn part1(contents: &String) {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let mut seeds: Vec<u64> = Vec::new();
    for seed in &split[0].split(" ").collect::<Vec<&str>>()[1..] {
        seeds.push(seed.parse::<u64>().expect("Should be u64"));
    }
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    for i in 1..split.len() {
        maps.push(parse_map(&split[i]));
    }

    let mut values = seeds;
    for map in maps {
        values = transform(values, &map);
    }

    let mut min = values[0];
    for value in values {
        if value < min {min = value};
    }
    println!("{min}");
}

fn transform_range(src: Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut output: Vec<(u64, u64)> = Vec::new();
    for s in src {
        let mut cur = s.0;
        while cur <= s.0 + s.1 {
            let mut next: u64 = 100000000000;
            let mut next_pos = 0; // won't be used if dst is unchanged
            let mut dst_next: u64;
            let dst: u64;
            for (i, row) in map.into_iter().enumerate() {
                // Choose the lowest mapping greater or equal to our current value
                if row.1 < next && row.1 + row.2 > cur {
                    next = row.1;
                    next_pos = i;
                }
            }
            // If the next range starts before current, take destination start
            // as the transform of current, take destination end as end of
            // range
            if next <= cur {
                dst_next = map[next_pos].0 + map[next_pos].2 - 1;
                next = map[next_pos].1 + map[next_pos].2 - 1;
                dst = cur - map[next_pos].1 + map[next_pos].0;
            }
            // If the next range starts after current, take destination start
            // as current, take destination end as last element before next
            // range
            else {
                dst_next = next - 1;
                next = dst_next;
                dst = cur;
            }

            // If the next range exceeds the current range, truncate it and
            // destination range
            if next > s.0 + s.1 {
                dst_next -= next - s.0 - s.1;
                next = s.0 + s.1;
            }
            output.push((dst, dst_next - dst));
            cur = next + 1;
        }
    }
    output
}

fn part2(contents: &String) {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let split_seeds: Vec<&str> = split[0].split(" ").collect();
    for i in 0..(split_seeds.len()-1)/2 {
        seeds.push((
            split_seeds[2*i+1].parse::<u64>().expect("Should be u64"),
            split_seeds[2*i+2].parse::<u64>().expect("Should be u64")
        ));
    }
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    for i in 1..split.len() {
        maps.push(parse_map(&split[i]));
    }

    let mut values = seeds;
    for map in maps {
        values = transform_range(values, &map);
        for value in &values {
            print!("({}, {}) ", value.0, value.1);
        } println!();
    }

    let mut min = values[0].0;
    for value in values {
        if value.0 < min {min = value.0};
    }
    println!("{min}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day05.txt").expect("");
    part1(&contents);
    part2(&contents);
}